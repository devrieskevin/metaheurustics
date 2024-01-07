use std::iter::FromIterator;

use metaheurustics::{
    individual::Individual,
    selection::parent::{LinearRanking, ParentSelector},
};
use pyo3::{pyclass, pymethods, FromPyObject, PyCell};
use rand::Rng;

use crate::fitness::PyFitness;

#[derive(FromPyObject)]
pub enum PyParentSelector<'py> {
    LinearRanking(&'py PyCell<PyLinearRanking>),
}

impl ParentSelector<PyFitness> for PyParentSelector<'_> {
    fn select<'a, R, I, C>(&self, rng: &mut R, individuals: &'a [I], number_children: usize) -> C
    where
        R: Rng + ?Sized,
        I: Individual<PyFitness>,
        C: FromIterator<&'a I>,
    {
        match self {
            PyParentSelector::LinearRanking(selector) => {
                selector.borrow().select(rng, individuals, number_children)
            }
        }
    }
}

#[pyclass(module = "metaheurustics", name = "LinearRanking")]
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

impl ParentSelector<PyFitness> for PyLinearRanking {
    fn select<'a, R, I, C>(&self, rng: &mut R, individuals: &'a [I], number_children: usize) -> C
    where
        R: Rng + ?Sized,
        I: Individual<PyFitness>,
        C: FromIterator<&'a I>,
    {
        self.selector.select(rng, individuals, number_children)
    }
}
