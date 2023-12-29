use pyo3::prelude::*;

mod individual;
mod mutation;
mod rand;
mod recombination;
mod selection;

/// A Python module implemented in Rust.
#[pymodule]
fn metaheurustics(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<mutation::PyBitFlip>()?;
    m.add_class::<recombination::PyOnePoint>()?;
    m.add_class::<selection::parent::PyLinearRanking>()?;
    m.add_class::<selection::survivor::PyReplaceWorst>()?;
    m.add_class::<rand::PySmallRng>()?;
    Ok(())
}
