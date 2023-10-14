use std::{cmp::Ordering, collections::HashSet, mem::swap};

use rand::{seq::IteratorRandom, Rng};

use crate::individual::Individual;

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

impl GroupType {
    pub fn is_population(&self) -> bool {
        matches!(self, Self::Population)
    }
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
            .partition(|x| x.group_type.is_population());

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

pub struct MergeRanked;

impl SurvivorSelector for MergeRanked {
    fn select<R, I, F>(&self, _rng: &mut R, population: &mut [I], offspring: Vec<I>)
    where
        R: Rng + ?Sized,
        I: Individual<F>,
        F: PartialOrd,
    {
        let mut offspring = offspring;

        let offspring_candidates = offspring
            .iter()
            .enumerate()
            .map(|(i, x)| (i, GroupType::Offspring, x.fitness()));
        let mut candidates: Vec<_> = population
            .iter()
            .enumerate()
            .map(|(i, x)| (i, GroupType::Offspring, x.fitness()))
            .chain(offspring_candidates)
            .collect();

        candidates.sort_by(|(_, _, a), (_, _, b)| b.partial_cmp(a).unwrap());

        let (population_winners, offspring_winners): (Vec<_>, Vec<_>) = candidates
            .into_iter()
            .take(population.len())
            .partition(|(_, group_type, _)| group_type.is_population());

        let population_winners_set: HashSet<_> =
            population_winners.into_iter().map(|(i, _, _)| i).collect();
        let offspring_winners_set: HashSet<_> =
            offspring_winners.into_iter().map(|(i, _, _)| i).collect();

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

pub struct GenerationalRanked;

impl SurvivorSelector for GenerationalRanked {
    fn select<R, I, F>(&self, _rng: &mut R, population: &mut [I], offspring: Vec<I>)
    where
        R: Rng + ?Sized,
        I: Individual<F>,
        F: PartialOrd,
    {
        let population_size = population.len();
        let offspring_size = offspring.len();
        assert!(
            population_size <= offspring_size,
            "The population size ({}) should have a smaller or equal size with respect to the amount of offspring ({})",
            population_size,
            offspring_size
        );

        let mut offspring = offspring;
        offspring.sort_by(|a, b| b.compare_fitness(a));
        population
            .iter_mut()
            .zip(offspring.iter_mut())
            .for_each(|(a, b)| swap(a, b));
    }
}
