use std::iter::FromIterator;

use metaheurustics::{selection::parent::{LinearRanking, ParentSelector}, individual::Individual};
use pyo3::{pyclass, pymethods};
use rand::Rng;

#[pyclass(name = "LinearRanking")]
pub struct PyLinearRanking {
    selector: LinearRanking,
}

#[pymethods]
impl PyLinearRanking {
    #[new]
    pub fn new(s: f64) -> Self {
        Self {
            selector: LinearRanking::new(s),
        }
    }
}

impl ParentSelector<f64> for PyLinearRanking {
    fn select<'a, R, I, C>(
        &self,
        rng: &mut R,
        individuals: &'a [I],
        number_children: usize,
    ) -> C
    where
        R: Rng + ?Sized,
        I: Individual<f64>,
        C: FromIterator<&'a I>,
    {
        self.selector.select(rng, individuals, number_children)
    }
}