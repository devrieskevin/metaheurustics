use ndarray_rand::rand::{seq::SliceRandom, Rng};

use crate::{individual::Individual, population::Population};

pub fn replace_worst_selection(
    population: &mut Population<f64>,
    offspring: &mut Population<f64>,
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
    population: &mut Population<f64>,
    offspring: &mut Population<f64>,
    number_rivals: usize,
) {
    let mut count: usize;
    let mut candidate: &mut Individual<f64>;

    let merged = population
        .individuals
        .iter()
        .chain(offspring.individuals.iter())
        .cloned()
        .collect();
    let mut merged = Population::new_from_individuals(merged);

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
                candidate.wins += if candidate.fitness > rival.fitness {
                    1
                } else {
                    0
                };
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
pub fn merge_ranked(population: &mut Population<f64>, offspring: &mut Population<f64>) {
    let mut merged: Vec<Individual<f64>> = population
        .individuals
        .iter()
        .chain(offspring.individuals.iter())
        .cloned()
        .collect();
    merged.sort_by(|a, b| b.compare_fitness(a));
    population.individuals = merged[0..population.individuals.len()].to_vec();
}
