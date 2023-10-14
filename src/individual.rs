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
