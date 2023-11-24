use metaheurustics::mutation::{integer::BitFlip, Mutator};
use pyo3::{pyclass, pymethods};
use rand::Rng;

#[pyclass]
pub struct PyBifFlip {
    mutator: BitFlip<i32>,
}

#[pymethods]
impl PyBifFlip {
    #[new]
    pub fn new(probability: f64, max_bit_count: u32, min_value: i32, max_value: i32) -> Self {
        Self {
            mutator: BitFlip::new(probability, max_bit_count, min_value, max_value),
        }
    }

}

impl Mutator<i32> for PyBifFlip {
    fn mutate<'a, R: Rng + ?Sized>(&self, rng: &mut R, parameter: &'a mut i32) -> &'a mut i32 {
        self.mutator.mutate(rng, parameter)
    }
}