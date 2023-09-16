use rand::Rng;
use rand_distr::{Uniform, WeightedIndex};

use crate::{
    individual::{BasicIndividual, Individual},
    population::Population,
};

pub trait ParentSelector {
    fn select<R, I, F, C>(&self, rng: &mut R, individuals: &[I], number_children: usize) -> C
    where
        R: Rng + ?Sized,
        I: Individual<F>,
        F: PartialOrd,
        C: FromIterator<usize>;
}

pub struct UniformSelector;

impl UniformSelector {
    pub fn new() -> Self {
        UniformSelector
    }
}

impl Default for UniformSelector {
    fn default() -> Self {
        Self::new()
    }
}

impl ParentSelector for UniformSelector {
    fn select<R, I, F, C>(&self, rng: &mut R, individuals: &[I], number_children: usize) -> C
    where
        R: Rng + ?Sized,
        I: Individual<F>,
        F: PartialOrd,
        C: FromIterator<usize>,
    {
        let population_size = individuals.len();

        (0..number_children)
            .map(|_| rng.gen_range(0..population_size))
            .collect()
    }
}

pub fn roulette_wheel<R: Rng + ?Sized>(
    rng: &mut R,
    population: &Population<f64>,
    number_children: usize,
    weights: &[f64],
) -> Population<f64> {
    let dist = WeightedIndex::new(weights).unwrap();
    let individuals = rng
        .sample_iter(dist)
        .take(number_children)
        .map(|i| population.individuals[i].clone())
        .collect();

    Population::new_from_individuals(individuals)
}

/// Selects parents from a population using the stochastic universal sampling method.
pub fn stochastic_universal_sampling<R: Rng + ?Sized>(
    rng: &mut R,
    population: &Population<f64>,
    number_children: usize,
    probabilities: &[f64],
) -> Population<f64> {
    let cumulative_probabilities = probabilities
        .iter()
        .scan(0.0, |state, x| {
            *state += x;
            Some(*state)
        })
        .enumerate();

    let mut selection = Vec::with_capacity(number_children);
    let mut r = rng.sample(Uniform::new(0.0, 1.0 / number_children as f64));
    for (i, cumulative_probability) in cumulative_probabilities {
        while r <= cumulative_probability {
            selection.push(population.individuals[i].clone());
            r += 1.0 / population.individuals.len() as f64;
        }
    }

    Population::new_from_individuals(selection)
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
        vec![BasicIndividual::new_empty(min_value, max_value, length); number_children];
    let mut candidates =
        vec![BasicIndividual::new_empty(min_value, max_value, length); tournament_size];

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

/// Selects parents from a population using the fitness proportionate selection method.
/// Due to stochastic noise, `stochastic_universal_sampling` is recommended over this method.
pub fn fitness_proportionate_selection<R: Rng + ?Sized>(
    rng: &mut R,
    population: &Population<f64>,
    number_children: usize,
) -> Population<f64> {
    let minimum_fitness = population
        .individuals
        .iter()
        .map(|x| x.fitness)
        .fold(None, |acc, x| match acc {
            Some(v) => Some(f64::min(v, x)),
            None => Some(x),
        })
        .unwrap();
    let mut sum_fitnesses = 0.0;
    let mut probabilities = Vec::new();
    for fitness in population
        .individuals
        .iter()
        // Add 1 to prevent `sum_fitnesses == 0`
        .map(|x| x.fitness - minimum_fitness + 1.0)
    {
        probabilities.push(fitness);
        sum_fitnesses += fitness;
    }

    probabilities.iter_mut().for_each(|fitness| {
        *fitness /= sum_fitnesses;
    });

    stochastic_universal_sampling(rng, population, number_children, &probabilities)
}

/// Selects parents from a population using linear ranking selection.
pub fn linear_ranking<R: Rng + ?Sized>(
    rng: &mut R,
    population: &mut Population<f64>,
    s: f64,
    number_children: usize,
) -> Population<f64> {
    let length = population.individuals.first().unwrap().value.len();
    let mu: f64 = length as f64;

    // Sort group based on fitness for ranking
    population.individuals.sort_by(|a, b| a.compare_fitness(b));

    // Compute probabilities from ranking
    let probabilities = (0..length)
        .map(|i| (2.0 - s) / mu + 2.0 * (i as f64) * (s - 1.0) / (mu * (mu - 1.0)))
        .collect::<Vec<f64>>();

    stochastic_universal_sampling(rng, population, number_children, &probabilities)
}

pub fn exponential_ranking<R: Rng + ?Sized>(
    rng: &mut R,
    population: &mut Population<f64>,
    number_children: usize,
) -> Population<f64> {
    let length = population.individuals.first().unwrap().value.len();

    // Sort group based on fitness for ranking
    population.individuals.sort_by(|a, b| a.compare_fitness(b));

    let mut probabilities: Vec<f64> = (0..length).map(|i| 1.0 - f64::exp(-(i as f64))).collect();
    let sum_probabilities: f64 = probabilities.iter().sum();

    probabilities.iter_mut().for_each(|probability| {
        *probability /= sum_probabilities;
    });

    stochastic_universal_sampling(rng, population, number_children, &probabilities)
}
