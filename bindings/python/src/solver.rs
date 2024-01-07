use std::{
    borrow::{Borrow, BorrowMut},
    ops::DerefMut,
};

use metaheurustics::{
    individual::Individual,
    selection::{parent::ParentSelector, survivor::SurvivorSelector},
};
use pyo3::{pyclass, pymethods, types::PyFunction, IntoPy, Py, PyCell, PyObject, PyResult, Python};

use crate::{
    individual::PyIndividual,
    mutation::PyIndividualMutator,
    rand::PySmallRng,
    recombination::PyIndividualRecombinator,
    selection::{parent::PyParentSelector, survivor::PySurvivorSelector},
};

#[pyclass(name = "Solver", module = "metaheurustics")]
pub struct PySolver {
    rng: PyObject,
    parent_selector: PyObject,
    recombinator: PyIndividualRecombinator,
    mutator: PyIndividualMutator,
    survivor_selector: PyObject,
    evaluator: Py<PyFunction>,
    initializer: Py<PyFunction>,
}

#[pymethods]
impl PySolver {
    #[new]
    fn new(
        rng: PyObject,
        parent_selector: PyObject,
        recombinator: PyObject,
        mutator: PyObject,
        survivor_selector: PyObject,
        evaluator: Py<PyFunction>,
        initializer: Py<PyFunction>,
    ) -> Self {
        Self {
            rng,
            parent_selector,
            recombinator: PyIndividualRecombinator::new(recombinator),
            mutator: PyIndividualMutator::new(mutator),
            survivor_selector,
            evaluator,
            initializer,
        }
    }

    fn solve<'py>(
        &'py self,
        py: Python<'py>,
        population_size: usize,
        number_generations: usize,
    ) -> PyResult<Vec<PyObject>> {
        let parent_selector: PyParentSelector = self.parent_selector.extract(py)?;
        let survivor_selector: PySurvivorSelector = self.survivor_selector.extract(py)?;
        let rng = self.rng.extract::<&'py PyCell<PySmallRng>>(py)?;
        let mut population = self
            .initializer
            .call1(py, (rng, population_size))?
            .extract::<Vec<PyObject>>(py)?
            .into_iter()
            .map(PyIndividual::new)
            .collect::<Vec<_>>();

        for individual in population.iter_mut() {
            let fitness = self
                .evaluator
                .call1(py, (individual.individual(),))?
                .extract(py)?;
            individual.set_fitness(fitness);
        }

        for _ in 0..number_generations {
            let mating_pool: Vec<_> = parent_selector.borrow().select(
                rng.borrow_mut().deref_mut(),
                &population,
                population_size,
            );
            let offspring: Vec<_> = mating_pool
                .chunks(2)
                .map(|x| {
                    x.iter()
                        .cloned()
                        .map(|x| PyCell::new(py, x.clone()).unwrap())
                        .collect()
                })
                .flat_map(|x: Vec<_>| self.recombinator.recombine(py, rng, x.try_into().unwrap()))
                .collect();

            for individual in offspring.iter() {
                let individual = PyCell::new(py, individual.clone()).unwrap();
                self.mutator.mutate(py, rng, individual);
                let fitness = self
                    .evaluator
                    .call1(py, (individual.borrow().individual(),))?
                    .extract(py)?;
                individual.borrow_mut().set_fitness(fitness);
            }

            survivor_selector.borrow().select(
                rng.borrow_mut().deref_mut(),
                population.borrow_mut(),
                offspring,
            );
        }

        Ok(population.iter().map(|x| x.clone().into_py(py)).collect())
    }
}
