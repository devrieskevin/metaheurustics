use std::rc::Rc;

use rand::Rng;

use crate::{mutation::Mutator, recombination::Recombinator};

pub trait Parameter<const N: usize> {
    fn mutate<R: Rng + ?Sized>(&mut self, rng: &mut R) -> &mut Self;

    fn recombine<R: Rng + ?Sized>(&self, rng: &mut R, parents: &[&Self; N]) -> [Self; N]
    where
        Self: Sized;
}

pub struct ParameterWrapper<T, R, const N: usize, M>
where
    R: Recombinator<T, N>,
    M: Mutator<T>,
{
    value: T,
    recombinator: Rc<R>,
    mutator: Rc<M>,
}

impl<T, C, const N: usize, M> ParameterWrapper<T, C, N, M>
where
    C: Recombinator<T, N>,
    M: Mutator<T>,
{
    pub fn new(value: T, recombinator: Rc<C>, mutator: Rc<M>) -> Self {
        Self {
            value,
            recombinator,
            mutator,
        }
    }
}

impl<T, C, const N: usize, M> Parameter<N> for ParameterWrapper<T, C, N, M>
where
    C: Recombinator<T, N>,
    M: Mutator<T>,
{
    fn mutate<R: Rng + ?Sized>(&mut self, rng: &mut R) -> &mut Self {
        self.mutator.mutate(rng, &mut self.value);
        self
    }

    fn recombine<R: Rng + ?Sized>(&self, rng: &mut R, parents: &[&Self; N]) -> [Self; N]
    where
        Self: Sized,
    {
        let values: Vec<_> = parents.iter().map(|parent| &parent.value).collect();
        let values_array: &[&T; N] = values[..].try_into().unwrap();

        let recombined: Vec<_> = self
            .recombinator
            .recombine(rng, values_array)
            .into_iter()
            .map(|value| {
                Self::new(
                    value,
                    Rc::clone(&self.recombinator),
                    Rc::clone(&self.mutator),
                )
            })
            .collect();

        match recombined.try_into() {
            Ok(x) => x,
            Err(_) => panic!("Recombination resulted in an unexpected amount of offspring."),
        }
    }
}

#[derive(Clone)]
pub struct BoundedVector<T>
where
    T: PartialOrd,
{
    pub min_value: T,
    pub max_value: T,
    pub value: Vec<T>,
}
