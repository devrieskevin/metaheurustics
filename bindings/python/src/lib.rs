use pyo3::{prelude::*, types::PyType};

mod composable;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

#[pyfunction]
fn type_name(py_type: &PyType) -> PyResult<String> {
    let name = py_type.name()?;
    Ok(format!("Type: {}", name))
}

/// A Python module implemented in Rust.
#[pymodule]
fn metaheurustics(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(type_name, m)?)?;
    Ok(())
}
