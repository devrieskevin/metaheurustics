use std::{cmp::Ordering, collections::HashSet, mem::swap};

use rand::{
    seq::{IteratorRandom, SliceRandom},
    Rng,
};

use crate::{
    individual::{BasicIndividual, Individual},
    population::BasicPopulation,
};

pub trait SurvivorSelector {
    fn select<R, I, F>(&self, rng: &mut R, population: &mut [I], offspring: Vec<I>)
    where
        R: Rng + ?Sized,
        I: Individual<F>,
        F: PartialOrd;
}

#[derive(PartialEq)]
enum GroupType {
    Population,
    Offspring,
}

struct TournamentCandidate<F>
where
    F: PartialOrd,
{
    pub candidate: usize,
    pub group_type: GroupType,
    wins: u32,
    fitness: F,
}

impl<F> TournamentCandidate<F>
where
    F: PartialOrd,
{
    pub fn new(candidate: usize, group_type: GroupType, fitness: F) -> Self {
        Self {
            candidate,
            group_type,
            wins: 0,
            fitness,
        }
    }

    pub fn compete(&mut self, other: &Self) -> &mut Self {
        self.wins += match self.fitness > other.fitness {
            true => 1,
            false => 0,
        };

        self
    }

    pub fn compare_wins(&self, other: &Self) -> Ordering {
        self.wins.cmp(&other.wins)
    }
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

pub struct RoundRobinTournament {
    number_rivals: usize,
}

impl SurvivorSelector for RoundRobinTournament {
    fn select<R, I, F>(&self, rng: &mut R, population: &mut [I], offspring: Vec<I>)
    where
        R: Rng + ?Sized,
        I: Individual<F>,
        F: PartialOrd,
    {
        let mut offspring = offspring;

        let population_candidates = population
            .iter()
            .enumerate()
            .map(|(i, x)| TournamentCandidate::new(i, GroupType::Population, x.fitness()));

        let mut candidates: Vec<_> = offspring
            .iter()
            .enumerate()
            .map(|(i, x)| TournamentCandidate::new(i, GroupType::Offspring, x.fitness()))
            .chain(population_candidates)
            .collect();

        let merged_size = candidates.len();
        // Determine wins in tournament
        for n in 0..merged_size {
            let (left, rest) = candidates.split_at_mut(n);
            let (candidate, right) = rest.split_first_mut().unwrap();

            let rivals = (0..merged_size)
                .choose_multiple(rng, merged_size)
                .into_iter()
                // Candidate does not battle itself
                .filter(|rival| *rival != n - 1)
                .take(self.number_rivals);

            for m in rivals {
                let rival = match m {
                    m if m < n - 1 => &left[m],
                    m if m > n - 1 => &right[m - n],
                    _ => panic!("Should not be happening"),
                };
                candidate.compete(rival);
            }
        }

        // Sort based on wins
        candidates.sort_by(|a, b| b.compare_wins(a));

        let (population_winners, offspring_winners): (Vec<_>, Vec<_>) = candidates
            .iter()
            .take(population.len())
            .partition(|x| x.group_type == GroupType::Population);

        let population_winners_set: HashSet<_> =
            population_winners.iter().map(|x| x.candidate).collect();
        let offspring_winners_set: HashSet<_> =
            offspring_winners.iter().map(|x| x.candidate).collect();

        let offspring_winner_refs = offspring
            .iter_mut()
            .enumerate()
            .filter(|(i, _)| offspring_winners_set.contains(i))
            .map(|(_, x)| x);

        let population_loser_refs = population
            .iter_mut()
            .enumerate()
            .filter(|(i, _)| !population_winners_set.contains(i))
            .map(|(_, x)| x);

        // Insert survivors into population
        population_loser_refs
            .zip(offspring_winner_refs)
            .for_each(|(a, b)| swap(a, b));
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
