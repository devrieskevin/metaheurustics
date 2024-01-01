use std::cmp::Ordering;

use pyo3::{FromPyObject, PyObject, Python};

#[derive(FromPyObject)]
#[pyo3(transparent)]
pub struct PyFitness {
    inner: PyObject,
}

impl PartialEq for PyFitness {
    fn eq(&self, other: &Self) -> bool {
        Python::with_gil(|py| {
            self.inner
                .as_ref(py)
                .eq(other.inner.as_ref(py))
                .expect("Equality comparison failed")
        })
    }
}

impl PartialOrd for PyFitness {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Python::with_gil(|py| self.inner.as_ref(py).compare(other.inner.as_ref(py)).ok())
    }
}
