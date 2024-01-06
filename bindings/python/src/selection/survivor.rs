use metaheurustics::{
    individual::Individual,
    selection::survivor::{ReplaceWorstSelector, SurvivorSelector},
};
use pyo3::{pyclass, pymethods, FromPyObject, PyCell};
use rand::Rng;

#[derive(FromPyObject)]
pub enum PySurvivorSelector<'py> {
    ReplaceWorst(&'py PyCell<PyReplaceWorst>),
}

impl SurvivorSelector for PySurvivorSelector<'_> {
    fn select<'a, R, I, F>(&self, rng: &mut R, individuals: &mut [I], offspring: Vec<I>)
    where
        R: Rng + ?Sized,
        I: Individual<F>,
        F: PartialOrd,
    {
        match self {
            PySurvivorSelector::ReplaceWorst(selector) => {
                selector.borrow().select(rng, individuals, offspring)
            }
        }
    }
}

#[pyclass(module = "metaheurustics", name = "ReplaceWorst")]
pub struct PyReplaceWorst {
    selector: ReplaceWorstSelector,
}

#[pymethods]
impl PyReplaceWorst {
    #[new]
    pub fn new(replacement_rate: f64) -> Self {
        Self {
            selector: ReplaceWorstSelector::new(replacement_rate),
        }
    }
}

impl SurvivorSelector for PyReplaceWorst {
    fn select<'a, R, I, F>(&self, rng: &mut R, individuals: &mut [I], offspring: Vec<I>)
    where
        R: Rng + ?Sized,
        I: Individual<F>,
        F: PartialOrd,
    {
        self.selector.select(rng, individuals, offspring);
    }
}
