use ndarray_rand::{
    rand::Rng,
    rand_distr::{Uniform, WeightedIndex},
};

use crate::{individual::Individual, population::Population};

/// Selects parents from a population using the fitness proportionate selection method.
/// Due to stochastic noise, `stochastic_universal_sampling` is recommended over this method.
pub fn fitness_proportionate_selection<R: Rng + ?Sized>(
    rng: &mut R,
    population: &Population<f64>,
) -> Population<f64> {
    let mut population = population.clone();

    let dist = WeightedIndex::new(
        population
            .individuals
            .iter()
            .map(|individual| individual.fitness),
    )
    .unwrap();

    population.individuals = rng
        .sample_iter(dist)
        .take(population.individuals.len())
        .map(|i| population.individuals[i].clone())
        .collect();

    population
}

/// Selects parents from a population using the stochastic universal sampling method.
pub fn stochastic_universal_sampling<R: Rng + ?Sized>(
    rng: &mut R,
    population: &Population<f64>,
) -> Population<f64> {
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
    let mut r = rng.sample(Uniform::new(0.0, 1.0 / population.individuals.len() as f64));
    for (i, cumulative_probability) in cumulative_probabilities {
        while r <= cumulative_probability {
            selection.push(population.individuals[i].clone());
            r += 1.0 / population.individuals.len() as f64;
        }
    }
    population.individuals = selection;
    population
}

/// Selects parents from a population using uniform selection.
pub fn uniform<R: Rng + ?Sized>(rng: &mut R, population: &Population<f64>) -> Population<f64> {
    let population_size = population.individuals.len();

    let selection = (0..population_size)
        .map(|_| rng.gen_range(0..population_size))
        .map(|i| population.individuals[i].clone())
        .collect();

    Population::new_from_individuals(selection)
}

/// Selects parents from a population using tournament selection.
pub fn tournament<R: Rng + ?Sized>(
    rng: &mut R,
    population: &Population<f64>,
    tournament_size: usize,
    number_accepted: usize,
    number_children: usize,
) -> Population<f64> {
    let min_value = population.individuals[0].min_value;
    let max_value = population.individuals[0].max_value;
    let length = population.individuals[0].value.len();

    let mut mating_pool =
        vec![Individual::new_empty(min_value, max_value, length); number_children];
    let mut candidates = vec![Individual::new_empty(min_value, max_value, length); tournament_size];

    for n in (0..number_children).step_by(number_accepted) {
        // Sample tournament candidates
        candidates.iter_mut().for_each(|individual| {
            *individual =
                population.individuals[rng.gen_range(0..population.individuals.len())].clone();
        });

        // Choose fittest candidates
        candidates.sort_by(|a, b| b.compare_fitness(a));
        for m in 0..number_accepted {
            if n + m < mating_pool.len() {
                mating_pool[n + m] = candidates[tournament_size - 1 - m].clone();
            }
        }
    }

    Population::new_from_individuals(mating_pool)
}
