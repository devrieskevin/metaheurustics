use rand::{seq::SliceRandom, Rng};

use crate::{
    individual::{BasicIndividual, Individual},
    population::BasicPopulation,
};

pub trait SurvivorSelector {
    fn select<R, I, F>(&self, rng: &mut R, parents: &mut [I], offspring: Vec<I>)
    where
        R: Rng + ?Sized,
        I: Individual<F>,
        F: PartialOrd;
}

pub struct ReplaceWorstSelector {
    replacement_rate: f64,
}

impl ReplaceWorstSelector {
    pub fn new(replacement_rate: f64) -> Self {
        ReplaceWorstSelector {
            replacement_rate: replacement_rate.clamp(0.0, 1.0),
        }
    }
}

impl SurvivorSelector for ReplaceWorstSelector {
    fn select<R, I, F>(&self, _rng: &mut R, population: &mut [I], offspring: Vec<I>)
    where
        R: Rng + ?Sized,
        I: Individual<F>,
        F: PartialOrd,
    {
        // Consume and make given offspring value mutable
        let mut offspring = offspring;

        population.sort_by(|a, b| a.compare_fitness(b));
        offspring.sort_by(|a, b| b.compare_fitness(a));

        let replacement_count = (self.replacement_rate * population.len() as f64) as usize;
        if replacement_count > population.len() {
            panic!("Replacement count must be less than population size");
        }

        population
            .iter_mut()
            .zip(offspring.into_iter())
            .take(replacement_count)
            .for_each(|(value, offspring)| {
                *value = offspring;
            });
    }
}

pub fn replace_worst_selection(
    population: &mut BasicPopulation<f64>,
    offspring: &mut BasicPopulation<f64>,
    replacement_rate: f64,
) {
    population.individuals.sort_by(|a, b| b.compare_fitness(a));
    offspring.individuals.sort_by(|a, b| a.compare_fitness(b));

    let replacement_count = (replacement_rate * population.individuals.len() as f64) as usize;
    if replacement_count > population.individuals.len() {
        panic!("Replacement count must be less than population size");
    }

    for n in (population.individuals.len() - replacement_count)..population.individuals.len() {
        population.individuals[n] = offspring.individuals[n].clone();
    }
}

/// Selects survivors from a population using round robin tournament selection.
pub fn round_robin_tournament<R: Rng + ?Sized>(
    rng: &mut R,
    population: &mut BasicPopulation<f64>,
    offspring: &mut BasicPopulation<f64>,
    number_rivals: usize,
) {
    let mut count: usize;
    let mut candidate: &mut BasicIndividual<f64>;

    let merged = merge_populations(population, offspring);
    let mut merged = BasicPopulation::new_from_individuals(merged);

    let merged_size = merged.individuals.len();
    // Determine wins in tournament
    for n in 0..merged_size {
        let rivals = merged.individuals.clone();
        let rivals = rivals.choose_multiple(rng, merged_size);

        // Initialize candidate for tournament
        candidate = &mut merged.individuals[n];
        candidate.wins = 0;

        count = 0;
        for rival in rivals {
            // Candidate does not battle itself
            if rival != candidate {
                candidate.compete(rival);
                count += 1;
            }

            if count >= number_rivals {
                break;
            };
        }
    }

    // Sort based on wins
    merged.individuals.sort_by(|a, b| b.compare_wins(a));

    // Insert survivors into population
    for n in 0..population.individuals.len() {
        population.individuals[n] = merged.individuals[n].clone();
    }
}

/// Selects survivors from a population using mu + lambda selection.
pub fn merge_ranked(population: &mut BasicPopulation<f64>, offspring: &mut BasicPopulation<f64>) {
    let mut merged: Vec<BasicIndividual<f64>> = merge_populations(population, offspring);
    merged.sort_by(|a, b| b.compare_fitness(a));
    population.individuals = merged[0..population.individuals.len()].to_vec();
}

/// Selects survivors from a population using (mu, lambda) selection.
pub fn generational_ranked(
    population: &mut BasicPopulation<f64>,
    offspring: &mut BasicPopulation<f64>,
) {
    offspring.individuals.sort_by(|a, b| b.compare_fitness(a));
    population.individuals = offspring.individuals[0..population.individuals.len()].to_vec();
}

fn merge_populations<B: FromIterator<BasicIndividual<f64>>>(
    population: &mut BasicPopulation<f64>,
    offspring: &mut BasicPopulation<f64>,
) -> B {
    population
        .individuals
        .iter()
        .chain(offspring.individuals.iter())
        .cloned()
        .collect()
}
