use std::marker::PhantomData;

use rand::{seq::SliceRandom, Rng};
use rand_distr::{uniform::SampleUniform, Uniform};

use crate::{
    individual::{BoundedVectorIndividual, Individual},
    parameter::BoundedVector,
};

pub enum MigrationType {
    Random,
    Best,
    Worst,
}

pub struct Population<I, F>
where
    I: Individual<F>,
    F: PartialOrd + Clone,
{
    individuals: Vec<I>,
    _marker: PhantomData<F>,
}

impl<I, F> Population<I, F>
where
    I: Individual<F>,
    F: PartialOrd + Clone,
{
    pub fn new_from_individuals(individuals: Vec<I>) -> Self {
        Self {
            individuals,
            _marker: PhantomData,
        }
    }

    pub fn individuals(&self) -> &[I] {
        &self.individuals
    }

    pub fn individuals_mut(&mut self) -> &mut [I] {
        &mut self.individuals
    }

    pub fn set_fitnesses(&mut self, fitnesses: &[F]) {
        if fitnesses.len() != self.individuals.len() {
            panic!("Length of fitnesses must be equal to the size of the population.");
        }

        self.individuals
            .iter_mut()
            .zip(fitnesses)
            .for_each(|(individual, fitness)| {
                individual.set_fitness(fitness.clone());
            });
    }

    pub fn increment_ages(&mut self) {
        self.individuals.iter_mut().for_each(|individual| {
            individual.set_age(individual.age() + 1);
        });
    }

    pub fn migrate<R: Rng + ?Sized>(
        rng: &mut R,
        archipelago: &mut [Self],
        number_swap: usize,
        migration_type: MigrationType,
        shuffle: bool,
    ) {
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

        let mut split_islands = archipelago.split_first_mut();
        while let Some((head, tail)) = split_islands {
            match tail.first_mut() {
                Some(next) => head.individuals[..number_swap]
                    .swap_with_slice(&mut next.individuals[..number_swap]),
                None => break,
            };
            split_islands = tail.split_first_mut();
        }
    }
}

impl<T, F> Population<BoundedVectorIndividual<T, F>, F>
where
    T: PartialOrd + SampleUniform + Copy,
    F: PartialOrd + Default + Copy,
{
    pub fn new<R: Rng + ?Sized>(
        rng: &mut R,
        min_value: T,
        max_value: T,
        length: usize,
        size: usize,
    ) -> Self {
        let individuals = (0..size)
            .map(|_| {
                BoundedVectorIndividual::new(BoundedVector {
                    min_value,
                    max_value,
                    value: rng
                        .sample_iter(Uniform::new_inclusive(min_value, max_value))
                        .take(length)
                        .collect(),
                })
            })
            .collect();

        Self {
            individuals,
            _marker: PhantomData,
        }
    }
}
