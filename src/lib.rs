mod individual;

#[cfg(test)]
mod tests {
    use super::individual::*;

    #[test]
    fn test_individual_creation() {
        let individual = NumericIndividual::new(0.0, 1.0, vec![0.5], vec![0.1]);
        let test_individual = NumericIndividual {
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
}
