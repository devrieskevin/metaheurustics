use itertools::Itertools;
use rand::{seq::index, Rng};
use rand_distr::{Bernoulli, Uniform, WeightedIndex};

use crate::individual::Individual;

pub trait ParentSelector<F>
where
    F: PartialOrd,
{
    fn select<'a, R, I, C>(&self, rng: &mut R, individuals: &'a [I], number_children: usize) -> C
    where
        R: Rng + ?Sized,
        I: Individual<F>,
        C: FromIterator<&'a I>;
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

impl<F> ParentSelector<F> for UniformSelector
where
    F: PartialOrd,
{
    fn select<'a, R, I, C>(&self, rng: &mut R, individuals: &'a [I], number_children: usize) -> C
    where
        R: Rng + ?Sized,
        I: Individual<F>,
        C: FromIterator<&'a I>,
    {
        let population_size = individuals.len();

        (0..number_children)
            .map(|_| rng.gen_range(0..population_size))
            .map(|a| &individuals[a])
            .collect()
    }
}

pub struct FitnessProportionate;

impl ParentSelector<f64> for FitnessProportionate {
    fn select<'a, R, I, C>(&self, rng: &mut R, individuals: &'a [I], number_children: usize) -> C
    where
        R: Rng + ?Sized,
        I: Individual<f64>,
        C: FromIterator<&'a I>,
    {
        let minimum_fitness = individuals
            .iter()
            .map(|x| x.fitness())
            .fold(None, |acc, x| match acc {
                Some(v) => Some(f64::min(v, x)),
                None => Some(x),
            })
            .unwrap();

        let effective_fitnesses = individuals
            .iter()
            // Add 1 to prevent `sum_fitnesses == 0`
            .map(|x| x.fitness() - minimum_fitness + 1.0);

        let mut sum_fitnesses = 0.0;
        let mut probabilities = Vec::new();
        for fitness in effective_fitnesses {
            probabilities.push(fitness);
            sum_fitnesses += fitness;
        }

        probabilities.iter_mut().for_each(|fitness| {
            *fitness /= sum_fitnesses;
        });

        stochastic_universal_sampling(rng, individuals, number_children, &probabilities)
    }
}

pub struct LinearRanking {
    s: f64,
}

impl LinearRanking {
    pub fn new(s: f64) -> Self {
        Self { s }
    }
}

impl<F> ParentSelector<F> for LinearRanking
where
    F: PartialOrd,
{
    fn select<'a, R, I, C>(&self, rng: &mut R, individuals: &'a [I], number_children: usize) -> C
    where
        R: Rng + ?Sized,
        I: Individual<F>,
        C: FromIterator<&'a I>,
    {
        let length = individuals.len();
        let mu: f64 = length as f64;

        let mut fitnesses: Vec<_> = individuals
            .iter()
            .map(|x| x.fitness())
            .enumerate()
            .collect();

        // Sort group based on fitness for ranking
        fitnesses.sort_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap());

        // Compute probabilities from ranking
        let mut probabilities = vec![0.0; length];
        (0..length)
            .map(|i| (2.0 - self.s) / mu + 2.0 * (i as f64) * (self.s - 1.0) / (mu * (mu - 1.0)))
            .zip(fitnesses.into_iter().map(|(i, _)| i))
            .for_each(|(p, i)| probabilities[i] = p);

        stochastic_universal_sampling(rng, individuals, number_children, &probabilities)
    }
}

pub struct ExponentialRanking;

impl<F> ParentSelector<F> for ExponentialRanking
where
    F: PartialOrd,
{
    fn select<'a, R, I, C>(&self, rng: &mut R, individuals: &'a [I], number_children: usize) -> C
    where
        R: Rng + ?Sized,
        I: Individual<F>,
        C: FromIterator<&'a I>,
    {
        let length = individuals.len();

        let mut fitnesses: Vec<_> = individuals
            .iter()
            .map(|x| x.fitness())
            .enumerate()
            .collect();

        fitnesses.sort_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap());

        let mut sum_probabilities = 0.0;
        let mut probabilities = vec![0.0; length];
        (0..length)
            .map(|i| 1.0 - f64::exp(-(i as f64)))
            .zip(fitnesses.into_iter().map(|(i, _)| i))
            .for_each(|(p, i)| {
                probabilities[i] = p;
                sum_probabilities += p;
            });

        probabilities.iter_mut().for_each(|probability| {
            *probability /= sum_probabilities;
        });

        stochastic_universal_sampling(rng, individuals, number_children, &probabilities)
    }
}

pub enum TournamentSampleMethod {
    WithReplacement,
    WithoutReplacement,
}

pub struct Tournament {
    tournament_size: usize,
    sample_method: TournamentSampleMethod,
    acceptance_probability: f64,
}

impl Tournament {
    pub fn new(
        tournament_size: usize,
        sample_method: TournamentSampleMethod,
        acceptance_probability: f64,
    ) -> Self {
        assert!(
            (0.0..=1.0).contains(&acceptance_probability),
            "The stochastic tournament result type value should be a valid probability."
        );

        Self {
            tournament_size,
            sample_method,
            acceptance_probability,
        }
    }

    fn sample_candidates<R>(&self, rng: &mut R, length: usize) -> Vec<usize>
    where
        R: Rng + ?Sized,
    {
        match self.sample_method {
            TournamentSampleMethod::WithReplacement => (0..self.tournament_size)
                .map(|_x| rng.gen_range(0..length))
                .collect(),
            TournamentSampleMethod::WithoutReplacement => {
                index::sample(rng, length, self.tournament_size).into_vec()
            }
        }
    }

    fn play_deterministic_tournament<'a, R, I, F, C>(
        &self,
        rng: &mut R,
        individuals: &'a [I],
        number_children: usize,
    ) -> C
    where
        R: Rng + ?Sized,
        I: Individual<F>,
        F: PartialOrd,
        C: FromIterator<&'a I>,
    {
        let length = individuals.len();

        let play_tournament = |_x| {
            self.sample_candidates(rng, length)
                .into_iter()
                .map(|i| &individuals[i])
                .max_by(|a, b| a.compare_fitness(b))
                .unwrap()
        };

        (0..number_children).map(play_tournament).collect()
    }

    fn play_stochastic_tournament<'a, R, I, F, C>(
        &self,
        rng: &mut R,
        individuals: &'a [I],
        number_children: usize,
    ) -> C
    where
        R: Rng + ?Sized,
        I: Individual<F>,
        F: PartialOrd,
        C: FromIterator<&'a I>,
    {
        let length = individuals.len();
        let distribution = Bernoulli::new(self.acceptance_probability).unwrap();

        let play_tournament = |_x| {
            let mut candidates: Vec<_> = self
                .sample_candidates(rng, length)
                .into_iter()
                .map(|i| &individuals[i])
                .collect();

            candidates.sort_by(|a, b| b.compare_fitness(a));

            candidates
                .into_iter()
                .find_or_last(|_x| rng.sample(distribution))
                .unwrap()
        };

        (0..number_children).map(play_tournament).collect()
    }
}

impl<F> ParentSelector<F> for Tournament
where
    F: PartialOrd,
{
    fn select<'a, R, I, C>(&self, rng: &mut R, individuals: &'a [I], number_children: usize) -> C
    where
        R: Rng + ?Sized,
        I: Individual<F>,
        C: FromIterator<&'a I>,
    {
        if self.acceptance_probability == 1.0 {
            self.play_deterministic_tournament(rng, individuals, number_children)
        } else {
            self.play_stochastic_tournament(rng, individuals, number_children)
        }
    }
}

pub fn roulette_wheel<'a, R, I, F, C>(
    rng: &mut R,
    individuals: &'a [I],
    number_children: usize,
    weights: &[f64],
) -> C
where
    R: Rng + ?Sized,
    I: Individual<F>,
    F: PartialOrd,
    C: FromIterator<&'a I>,
{
    let dist = WeightedIndex::new(weights).unwrap();
    rng.sample_iter(dist)
        .take(number_children)
        .map(|i| &individuals[i])
        .collect()
}

pub fn stochastic_universal_sampling<'a, R, I, F, C>(
    rng: &mut R,
    individuals: &'a [I],
    number_children: usize,
    probabilities: &[f64],
) -> C
where
    R: Rng + ?Sized,
    I: Individual<F>,
    F: PartialOrd,
    C: FromIterator<&'a I>,
{
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
            selection.push(i);
            r += 1.0 / individuals.len() as f64;
        }
    }

    selection.iter().map(|i| &individuals[*i]).collect()
}
