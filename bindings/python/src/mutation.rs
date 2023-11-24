use metaheurustics::mutation::{integer::BitFlip, Mutator};
use pyo3::{pyclass, pymethods};
use rand::Rng;

#[pyclass(name = "BitFlip")]
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

}

impl Mutator<i32> for PyBitFlip {
    fn mutate<'a, R: Rng + ?Sized>(&self, rng: &mut R, parameter: &'a mut i32) -> &'a mut i32 {
        self.mutator.mutate(rng, parameter)
    }
}