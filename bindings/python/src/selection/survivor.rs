use metaheurustics::{selection::survivor::{ReplaceWorstSelector, SurvivorSelector}, individual::Individual};
use pyo3::{pyclass, pymethods};
use rand::Rng;

#[pyclass(name = "ReplaceWorst")]
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
    fn select<'a, R, I, F>(
        &self,
        rng: &mut R,
        individuals: &mut [I],
        offspring: Vec<I>,
    )
    where
        R: Rng + ?Sized,
        I: Individual<F>,
        F: PartialOrd,
    {
        self.selector.select(rng, individuals, offspring);
    }
}