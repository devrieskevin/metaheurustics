use metaheurustics::individual::Individual;
use pyo3::{pyclass, pymethods, IntoPy, PyClass, PyObject, Python};

#[pyclass(name = "Individual", module = "metaheurustics")]
#[derive(Clone)]
pub struct PyIndividual {
    individual: PyObject,
}

#[pymethods]
impl PyIndividual {
    #[new]
    pub fn new(individual: PyObject) -> Self {
        Self { individual }
    }

    #[getter]
    pub fn individual(&self) -> &PyObject {
        &self.individual
    }
}

impl<F> Individual<F> for PyIndividual
where
    F: PartialOrd + Clone + PyClass + IntoPy<PyObject>,
{
    fn fitness(&self) -> F {
        Python::with_gil(|py| {
            self.individual
                .as_ref(py)
                .call_method("get_fitness", (), None)
                .unwrap()
                .extract()
                .unwrap()
        })
    }

    fn set_fitness(&mut self, fitness: F) -> &mut Self {
        Python::with_gil(|py| {
            let py_fitness = fitness.clone();
            let py_individual = self.individual.as_ref(py);
            py_individual
                .call_method("set_fitness", (py_fitness,), None)
                .unwrap();
        });

        self
    }

    fn age(&self) -> u32 {
        Python::with_gil(|py| {
            self.individual
                .as_ref(py)
                .call_method("get_age", (), None)
                .unwrap()
                .extract()
                .unwrap()
        })
    }

    fn set_age(&mut self, age: u32) -> &mut Self {
        Python::with_gil(|py| {
            let py_age = age;
            let py_individual = self.individual.as_ref(py);
            py_individual
                .call_method("set_age", (py_age,), None)
                .unwrap();
        });

        self
    }
}
