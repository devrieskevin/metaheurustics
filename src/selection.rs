use ndarray_rand::{rand::{self, Rng, thread_rng}, rand_distr::{Uniform, WeightedIndex}};


use crate::population::Population;

mod survivor;
mod parent;

pub fn select_fitness_proportionate(population: &Population<f64>) -> Population<f64> {
    let mut population = population.clone();
    population.individuals.sort_by(|a, b| b.compare_fitness(a));

    let dist = WeightedIndex::new(
        population.individuals
            .iter()
            .map(|individual| individual.fitness)
    ).unwrap();

    population.individuals = thread_rng().sample_iter(dist)
        .take(population.individuals.len())
        .map(|i| population.individuals[i].clone())
        .collect();

    population
}
