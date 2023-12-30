use metaheurustics::mutation::{integer::BitFlip, Mutator};
use pyo3::{pyclass, pymethods, PyCell, PyObject, Python};

use crate::{individual::PyIndividual, rand::PySmallRng};

#[pyclass(module = "metaheurustics", name = "IndividualMutator")]
pub struct PyIndividualMutator {
    individual_mutator: PyObject,
}

#[pymethods]
impl PyIndividualMutator {
    #[new]
    pub fn new(individual_mutator: PyObject) -> Self {
        Self { individual_mutator }
    }

    pub fn mutate<'py>(
        &self,
        py: Python<'py>,
        rng: &'py PyCell<PySmallRng>,
        parameter: &'py PyCell<PyIndividual>,
    ) -> &'py PyCell<PyIndividual> {
        self.individual_mutator
            .call_method(py, "mutate", (rng, parameter), None)
            .expect("Failed to call mutate");
        parameter
    }
}

#[pyclass(module = "metaheurustics", name = "BitFlip")]
pub struct PyBitFlip {
    mutator: BitFlip<i32>,
}

#[pymethods]
impl PyBitFlip {
    #[new]
    pub fn new(probability: f64, max_bit_count: u32, min_value: i32, max_value: i32) -> Self {
        Self {
            mutator: BitFlip::new(probability, max_bit_count, min_value, max_value),
        }
    }

    pub fn mutate(&self, rng: &mut PySmallRng, parameter: i32) -> i32 {
        let mut parameter = parameter;
        self.mutator.mutate(rng, &mut parameter);
        parameter
    }
}
