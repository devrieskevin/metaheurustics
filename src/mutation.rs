use rand::Rng;
use rand_distr::{uniform::SampleUniform, Uniform};

use crate::parameter::BoundedVector;

pub trait Mutator<T> {
    fn mutate<'a, R: Rng + ?Sized>(&self, rng: &mut R, parameter: &'a mut T) -> &'a mut T;
}

pub struct UniformMutator {
    probability: f64,
}

impl UniformMutator {
    pub fn new(probability: f64) -> Self {
        UniformMutator {
            probability: probability.clamp(0.0, 1.0),
        }
    }
}

impl<T> Mutator<BoundedVector<T>> for UniformMutator
where
    T: PartialOrd + SampleUniform + Copy,
{
    fn mutate<'a, R: Rng + ?Sized>(
        &self,
        rng: &mut R,
        parameter: &'a mut BoundedVector<T>,
    ) -> &'a mut BoundedVector<T> {
        let distribution = Uniform::new_inclusive(parameter.min_value, parameter.max_value);
        parameter.value.iter_mut().for_each(|value| {
            let random_value = rng.sample(Uniform::new(0.0, 1.0));
            if random_value <= self.probability {
                *value = rng.sample(&distribution);
            }
        });
        parameter
    }
}
