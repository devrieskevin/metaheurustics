use std::{cmp::Ordering, marker::PhantomData};

use crate::{mutation::Mutator, parameter::BoundedVector, recombination::Recombinator};

pub trait Individual<F>
where
    F: PartialOrd,
{
    fn fitness(&self) -> F;

    fn set_fitness(&mut self, fitness: F) -> &mut Self;

    fn compare_fitness(&self, other: &Self) -> Ordering {
        F::partial_cmp(&self.fitness(), &other.fitness()).unwrap()
    }

    fn age(&self) -> u32;

    fn set_age(&mut self, age: u32) -> &mut Self;

    fn compare_age(&self, other: &Self) -> Ordering {
        u32::cmp(&self.age(), &other.age())
    }
}

#[derive(Debug, Clone)]
/// Individual for numerical values
pub struct BasicIndividual<T> {
    pub min_value: T,
    pub max_value: T,
    pub value: Vec<T>,
    pub std_dev: Vec<T>,
    pub age: u32,
    pub fitness: T,
    pub wins: u32,
}

/// NumericIndividual implementation for f64
impl BasicIndividual<f64> {
    /// Creates a new [`Individual<f64>`].
    pub fn new(
        min_value: f64,
        max_value: f64,
        value: Vec<f64>,
        std_dev: Vec<f64>,
    ) -> BasicIndividual<f64> {
        BasicIndividual {
            min_value,
            max_value,
            value,
            std_dev,
            fitness: min_value,
            age: 0,
            wins: 0,
        }
    }

    /// Creates a new empty [`Individual<f64>`].
    pub fn new_empty(min_value: f64, max_value: f64, length: usize) -> BasicIndividual<f64> {
        BasicIndividual {
            min_value,
            max_value,
            value: vec![0.0; length],
            std_dev: vec![0.0; length],
            fitness: min_value,
            age: 0,
            wins: 0,
        }
    }

    pub fn compete(&mut self, other: &Self) -> &mut Self {
        self.wins += match self.fitness > other.fitness {
            true => 1,
            false => 0,
        };

        self
    }

    /// Compares the wins of this [`Individual<f64>`] with another [`Individual<f64>`].
    pub fn compare_wins(&self, other: &BasicIndividual<f64>) -> Ordering {
        self.wins.cmp(&other.wins)
    }
}

impl PartialEq for BasicIndividual<f64> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl Individual<f64> for BasicIndividual<f64> {
    fn fitness(&self) -> f64 {
        self.fitness
    }

    /// Sets the fitness of this [`Individual<f64>`].
    fn set_fitness(&mut self, fitness: f64) -> &mut Self {
        self.fitness = fitness;
        self
    }

    fn age(&self) -> u32 {
        self.age
    }

    fn set_age(&mut self, age: u32) -> &mut Self {
        self.age = age;
        self
    }
}

pub struct BoundedVectorIndividual<T, F>
where
    T: PartialOrd,
    F: PartialOrd,
{
    vector: BoundedVector<T>,
    fitness: F,
    age: u32,
}

impl<T, F> Individual<F> for BoundedVectorIndividual<T, F>
where
    T: PartialOrd,
    F: PartialOrd + Copy,
{
    fn fitness(&self) -> F {
        self.fitness
    }

    fn set_fitness(&mut self, fitness: F) -> &mut Self {
        self.fitness = fitness;
        self
    }

    fn age(&self) -> u32 {
        self.age
    }

    fn set_age(&mut self, age: u32) -> &mut Self {
        self.age = age;
        self
    }
}

impl<T, F> BoundedVectorIndividual<T, F>
where
    T: PartialOrd,
    F: PartialOrd,
{
    pub fn vector(&self) -> &BoundedVector<T> {
        &self.vector
    }
}

impl<T, F> BoundedVectorIndividual<T, F>
where
    T: PartialOrd,
    F: PartialOrd + Default,
{
    pub fn new(vector: BoundedVector<T>) -> Self {
        Self {
            vector,
            fitness: Default::default(),
            age: 0,
        }
    }
}

pub struct BoundedVectorIndividualMutator<T, M>
where
    T: PartialOrd,
    M: Mutator<BoundedVector<T>>,
{
    vector_mutator: M,
    _phantom: PhantomData<T>,
}

impl<T, M> BoundedVectorIndividualMutator<T, M>
where
    T: PartialOrd,
    M: Mutator<BoundedVector<T>>,
{
    pub fn new(vector_mutator: M) -> Self {
        Self {
            vector_mutator,
            _phantom: PhantomData,
        }
    }
}

impl<T, M, F> Mutator<BoundedVectorIndividual<T, F>> for BoundedVectorIndividualMutator<T, M>
where
    T: PartialOrd,
    M: Mutator<BoundedVector<T>>,
    F: PartialOrd + Copy + Default,
{
    fn mutate<'a, R: rand::Rng + ?Sized>(
        &self,
        rng: &mut R,
        parameter: &'a mut BoundedVectorIndividual<T, F>,
    ) -> &'a mut BoundedVectorIndividual<T, F> {
        self.vector_mutator.mutate(rng, &mut parameter.vector);
        parameter
    }
}

pub struct BoundedVectorIndividualRecombinator<T, R, const N: usize>
where
    T: PartialOrd,
    R: Recombinator<BoundedVector<T>, N>,
{
    vector_recombinator: R,
    _phantom: PhantomData<T>,
}

impl<T, R, const N: usize> BoundedVectorIndividualRecombinator<T, R, N>
where
    T: PartialOrd,
    R: Recombinator<BoundedVector<T>, N>,
{
    pub fn new(vector_recombinator: R) -> Self {
        Self {
            vector_recombinator,
            _phantom: PhantomData,
        }
    }
}

impl<C, T, F, const N: usize> Recombinator<BoundedVectorIndividual<T, F>, N>
    for BoundedVectorIndividualRecombinator<T, C, N>
where
    T: PartialOrd,
    C: Recombinator<BoundedVector<T>, N>,
    F: PartialOrd + Copy + Default,
{
    fn recombine<R: rand::Rng + ?Sized>(
        &self,
        rng: &mut R,
        parents: &[&BoundedVectorIndividual<T, F>; N],
    ) -> [BoundedVectorIndividual<T, F>; N] {
        let values: Vec<_> = parents.iter().map(|parent| parent.vector()).collect();
        let values_array = values[..].try_into().unwrap();
        let recombined_individuals: Vec<_> = self
            .vector_recombinator
            .recombine(rng, values_array)
            .into_iter()
            .map(|value| BoundedVectorIndividual::new(value))
            .collect();

        match recombined_individuals.try_into() {
            Ok(x) => x,
            Err(_) => panic!("Recombination resulted in an unexpected amount of offspring."),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_individual_creation() {
        let individual = BasicIndividual::new(0.0, 1.0, vec![0.5], vec![0.1]);
        let test_individual = BasicIndividual {
            min_value: 0.0,
            max_value: 1.0,
            value: vec![0.5],
            std_dev: vec![0.1],
            age: 0,
            fitness: 0.0,
            wins: 0,
        };
        assert_eq!(individual, test_individual);
    }

    #[test]
    fn test_individual_set_fitness() {
        let mut individual = BasicIndividual::new(0.0, 1.0, vec![0.5], vec![0.1]);
        individual.set_fitness(0.5).set_fitness(1.0);
        assert_eq!(individual.fitness, 1.0);
    }
}
