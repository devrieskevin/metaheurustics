use metaheurustics::recombination::{integer::OnePoint, Recombinator};
use pyo3::{pyclass, pymethods};
use rand::Rng;

#[pyclass(name = "OnePoint")]
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
}

impl Recombinator<i32, 2> for PyOnePoint {
    fn recombine<'a, R: Rng + ?Sized>(&self, rng: &mut R, parents: &[&'a i32; 2]) -> [i32; 2] {
        self.recombinator.recombine(rng, parents)
    }
}