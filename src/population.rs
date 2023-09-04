use rand::{seq::SliceRandom, Rng};
use rand_distr::Uniform;

use crate::individual::{BasicIndividual, Individual};

pub enum MigrationType {
    Random,
    Best,
    Worst,
}

#[derive(Clone, Debug)]
pub struct Population<T> {
    pub individuals: Vec<BasicIndividual<T>>,
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
                BasicIndividual::new(
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
    pub fn new_from_individuals(individuals: Vec<BasicIndividual<f64>>) -> Self {
        Self { individuals }
    }

    /// Sets the fitnesses of the population.
    pub fn set_fitnesses(&mut self, fitnesses: &[f64]) {
        if fitnesses.len() != self.individuals.len() {
            panic!("Length of fitnesses must be equal to the size of the population.");
        }

        self.individuals
            .iter_mut()
            .zip(fitnesses)
            .for_each(|(individual, fitness)| individual.fitness = *fitness);
    }

    /// Increments the age of all individuals in the population.
    pub fn increment_ages(&mut self) {
        self.individuals.iter_mut().for_each(|individual| {
            individual.age += 1;
        });
    }

    /// Migrates individuals between populations.
    pub fn migrate<R: Rng + ?Sized>(
        rng: &mut R,
        archipelago: &mut [Self],
        number_swap: usize,
        migration_type: MigrationType,
        shuffle: bool,
    ) {
        let archipelago_size = archipelago.len();
        let min_value = archipelago[0].individuals[0].min_value;
        let max_value = archipelago[0].individuals[0].max_value;
        let length = archipelago[0].individuals[0].value.len();

        // Cache array to store individuals for migration to next island
        let mut cache: Vec<BasicIndividual<f64>> =
            vec![BasicIndividual::new_empty(min_value, max_value, length); number_swap];

        // Choose if ring topology or random pairwise topology
        if shuffle {
            archipelago.shuffle(rng);
        }

        // Sort or shuffle individuals per island
        for island in archipelago.iter_mut() {
            match migration_type {
                MigrationType::Random => island.individuals.shuffle(rng),
                MigrationType::Best => island.individuals.sort_by(|a, b| b.compare_fitness(a)),
                MigrationType::Worst => island.individuals.sort_by(|a, b| a.compare_fitness(b)),
            }
        }

        // Swaps individuals between islands
        for i in 0..archipelago_size {
            let j = (i + 1) % archipelago_size;

            for (k, cache_value) in cache.iter_mut().enumerate().take(number_swap) {
                // Start cache values from i == 0
                if i == 0 {
                    *cache_value = archipelago[i].individuals[k].clone();
                }

                // Set individual and update cache
                let temp = archipelago[j].individuals[k].clone();
                archipelago[j].individuals[k] = cache_value.clone();
                *cache_value = temp;
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use rand::thread_rng;

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
