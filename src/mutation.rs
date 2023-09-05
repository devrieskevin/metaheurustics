use rand::Rng;
use rand_distr::{uniform::SampleUniform, Normal, Uniform};

use crate::{individual::BasicIndividual, parameter::BoundedVector};

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

/// Mutate the given children using the uniform mutation method.
pub fn uniform<R: Rng + ?Sized>(
    rng: &mut R,
    probability: f64,
    children: &mut [BasicIndividual<f64>],
) {
    children.iter_mut().for_each(|child| {
        child.value.iter_mut().for_each(|value| {
            let random_value = rng.sample(Uniform::new(0.0, 1.0));
            if random_value <= probability {
                let distribution = Uniform::new_inclusive(child.min_value, child.max_value);
                *value = rng.sample(distribution);
            }
        });
    });
}

/// Mutate the given children using simple Gaussian perturbation mutation method using individual variance(s).
pub fn simple_gaussian<R: Rng + ?Sized>(rng: &mut R, children: &mut [BasicIndividual<f64>]) {
    children.iter_mut().for_each(|child| {
        child.value.iter_mut().enumerate().for_each(|(n, value)| {
            let distribution = Normal::new(0.0, child.std_dev[n]).unwrap();
            *value += rng.sample(distribution);

            // Clamp the value to the min/max value.
            if *value < child.min_value {
                *value = child.min_value;
            } else if *value > child.max_value {
                *value = child.max_value;
            }
        });
    });
}

/// Mutate the given children using uncorrelated adaptive Gaussian perturbation with changing step sizes.
pub fn uncorrelated_adaptive_gaussian<R: Rng + ?Sized>(
    rng: &mut R,
    children: &mut [BasicIndividual<f64>],
    lr1: f64,
    lr2: f64,
    eps: f64,
) {
    let mut base_mutation: f64;
    let mut step_factor: f64;
    let distribution = Normal::new(0.0, 1.0).unwrap();

    // Apply mutation to std values
    for child in children.iter_mut() {
        base_mutation = lr1 * rng.sample(distribution);
        for j in 0..child.std_dev.len() {
            step_factor = f64::exp(base_mutation + lr2 * rng.sample(distribution));
            child.std_dev[j] *= step_factor;

            if child.std_dev[j] < eps {
                child.std_dev[j] = eps;
            }
        }
    }

    // Apply Gaussian perturbation
    simple_gaussian(rng, children);
}
