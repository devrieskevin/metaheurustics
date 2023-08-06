#[derive(Debug, PartialEq)]
/// Individual for numerical values
pub struct Individual<T> {
    pub min_value: T,
    pub max_value: T,
    pub value: Vec<T>,
    pub std_dev: Vec<T>,
    pub age: u32,
    pub fitness: T,
    pub wins: u32,
}

/// NumericIndividual implementation for f64
impl Individual<f64> {
    /// Create a new Individual
    pub fn new(
        min_value: f64,
        max_value: f64,
        value: Vec<f64>,
        std_dev: Vec<f64>,
    ) -> Individual<f64> {
        Individual {
            min_value,
            max_value,
            value,
            std_dev,
            age: 0,
            fitness: 0.0,
            wins: 0,
        }
    }

    pub fn set_fitness(&mut self, fitness: f64) -> &mut Self {
        self.fitness = fitness;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_individual_creation() {
        let individual = Individual::new(0.0, 1.0, vec![0.5], vec![0.1]);
        let test_individual = Individual {
            min_value: 0.0,
            max_value: 1.0,
            value: vec![0.5],
            std_dev: vec![0.1],
            age: 0,
            fitness: 0.0,
            wins: 0,
        };
        assert_eq!(individual, test_individual);
    }

    #[test]
    fn test_individual_set_fitness() {
        let mut individual = Individual::new(0.0, 1.0, vec![0.5], vec![0.1]);
        individual.set_fitness(0.5).set_fitness(1.0);
        assert_eq!(individual.fitness, 1.0);
    }
}
