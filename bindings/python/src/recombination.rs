use metaheurustics::recombination::{integer::OnePoint, Recombinator};
use pyo3::{pyclass, pymethods, PyCell, PyObject, Python};

use crate::{individual::PyIndividual, rand::PySmallRng};

#[pyclass(module = "metaheurustics", name = "IndividualRecombinator")]
pub struct PyIndividualRecombinator {
    individual_recombinator: PyObject,
}

#[pymethods]
impl PyIndividualRecombinator {
    #[new]
    pub fn new(individual_recombinator: PyObject) -> Self {
        Self {
            individual_recombinator,
        }
    }

    pub fn recombine<'py>(
        &self,
        py: Python<'py>,
        rng: &'py PyCell<PySmallRng>,
        parents: [&'py PyCell<PyIndividual>; 2],
    ) -> [PyIndividual; 2] {
        let result = self
            .individual_recombinator
            .call_method1(py, "recombine", (rng, parents))
            .expect("Failed to call recombine")
            .extract::<Vec<_>>(py)
            .expect("Failed to extract recombine result")
            .try_into();

        match result {
            Ok(result) => result,
            _ => panic!("Failed to convert recombine result"),
        }
    }
}

#[pyclass(module = "metaheurustics", name = "OnePoint")]
pub struct PyOnePoint {
    recombinator: OnePoint,
}

#[pymethods]
impl PyOnePoint {
    #[new]
    fn new() -> Self {
        Self {
            recombinator: OnePoint,
        }
    }

    fn recombine(&self, rng: &mut PySmallRng, parents: [i32; 2]) -> [i32; 2] {
        let parent_values = [&parents[0], &parents[1]];
        self.recombinator.recombine(rng, &parent_values)
    }
}
