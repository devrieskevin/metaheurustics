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
    mutator: PyIndividualMutator,
    recombinator: PyIndividualRecombinator,
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
        mutator: PyObject,
        recombinator: PyObject,
        survivor_selector: PyObject,
        evaluator: Py<PyFunction>,
        initializer: Py<PyFunction>,
    ) -> Self {
        Self {
            rng,
            parent_selector,
            mutator: PyIndividualMutator::new(mutator),
            recombinator: PyIndividualRecombinator::new(recombinator),
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
            .extract::<Vec<PyIndividual>>(py)?;
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
                individual
                    .borrow_mut()
                    .set_fitness(self.evaluator.call1(py, (individual,))?.extract(py)?);
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
