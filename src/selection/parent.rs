use std::iter::successors;

use ndarray_rand::{
    rand::{thread_rng, Rng},
    rand_distr::{Uniform, WeightedIndex},
};

use crate::population::Population;

/// Selects parents from a population using the fitness proportionate selection method.
/// Due to stochastic noise, `stochastic_universal_sampling` is recommended over this method.
pub fn fitness_proportionate_selection(population: &Population<f64>) -> Population<f64> {
    let mut population = population.clone();

    let dist = WeightedIndex::new(
        population
            .individuals
            .iter()
            .map(|individual| individual.fitness),
    )
    .unwrap();

    population.individuals = thread_rng()
        .sample_iter(dist)
        .take(population.individuals.len())
        .map(|i| population.individuals[i].clone())
        .collect();

    population
}

/// Selects parents from a population using the stochastic universal sampling method.
pub fn stochastic_universal_sampling(population: &Population<f64>) -> Population<f64> {
    let mut population = population.clone();
    population.individuals.sort_by(|a, b| b.compare_fitness(a));

    let sum_fitnesses = population
        .individuals
        .iter()
        .map(|individual| individual.fitness)
        .sum::<f64>();
    let cumulative_probabilities = population
        .individuals
        .iter()
        .map(|individual| individual.fitness / sum_fitnesses)
        .scan(0.0, |state, x| {
            *state += x;
            Some(*state)
        })
        .enumerate();

    let mut selection = Vec::with_capacity(population.individuals.len());
    let mut r = thread_rng().sample(Uniform::new(0.0, 1.0 / population.individuals.len() as f64));
    for (i, cumulative_probability) in cumulative_probabilities {
        while r <= cumulative_probability {
            selection.push(population.individuals[i].clone());
            r += 1.0 / population.individuals.len() as f64;
        }
    }
    population.individuals = selection;
    population
}
