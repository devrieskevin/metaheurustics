use ndarray_rand::{rand::Rng, rand_distr::Uniform};

use crate::individual::Individual;

#[derive(Clone, Debug)]
pub struct Population<T> {
    pub individuals: Vec<Individual<T>>,
}

impl Population<f64> {
    /// Creates a new [`Population<f64>`].
    pub fn new<R: Rng + ?Sized>(
        rng: &mut R,
        min_value: f64,
        max_value: f64,
        length: usize,
        size: usize,
    ) -> Self {
        let individuals = (0..size)
            .map(|_| {
                Individual::new(
                    min_value,
                    max_value,
                    rng.sample_iter(Uniform::new_inclusive(min_value, max_value))
                        .take(length)
                        .collect(),
                    (0..length).map(|_| 0.0).collect(),
                )
            })
            .collect();
        Self { individuals }
    }

    /// Creates a new [`Population<f64>`] from a vector of [`Individual<f64>`].
    pub fn new_from_individuals(individuals: Vec<Individual<f64>>) -> Self {
        Self { individuals }
    }
}

#[cfg(test)]
mod tests {

    use ndarray_rand::rand::thread_rng;

    use super::*;

    #[test]
    fn test_population() {
        let mut rng = thread_rng();
        let population: Population<f64> = Population::new(&mut rng, 0.0, 1.0, 5, 10);
        assert_eq!(population.individuals.len(), 10);
        population.individuals.iter().for_each(|individual| {
            assert_eq!(individual.value.len(), 5);
            assert_eq!(individual.std_dev.len(), 5);
        });
    }
}
