use std::collections::HashMap;

use pyo3::FromPyObject;

#[derive(FromPyObject)]
pub enum PyInputParameter {
    Int(i64),
}

pub struct ComposableIndividual {
    parameters: HashMap<String, PyInputParameter>
}

impl ComposableIndividual {
    pub fn new() -> Self {
        Self {
            parameters: HashMap::new()
        }
    }

    pub fn get_parameters(&self) -> &HashMap<String, PyInputParameter> {
        &self.parameters
    }
}