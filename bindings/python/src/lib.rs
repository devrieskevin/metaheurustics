use pyo3::prelude::*;

mod fitness;
mod individual;
mod mutation;
mod rand;
mod recombination;
mod selection;
mod solver;

/// A Python module implemented in Rust.
#[pymodule]
fn metaheurustics(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<mutation::PyBitFlip>()?;
    m.add_class::<recombination::PyOnePoint>()?;
    m.add_class::<selection::parent::PyLinearRanking>()?;
    m.add_class::<selection::survivor::PyReplaceWorst>()?;
    m.add_class::<rand::PySmallRng>()?;
    m.add_class::<individual::PyIndividual>()?;
    m.add_class::<mutation::PyIndividualMutator>()?;
    m.add_class::<recombination::PyIndividualRecombinator>()?;
    m.add_class::<solver::PySolver>()?;
    Ok(())
}
