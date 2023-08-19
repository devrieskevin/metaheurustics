use ndarray_rand::{
    rand::Rng,
    rand_distr::{Normal, Uniform},
};

use crate::individual::Individual;

/// Mutate the given children using the uniform mutation method.
pub fn uniform<R: Rng + ?Sized>(rng: &mut R, probability: f64, children: &mut [Individual<f64>]) {
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
pub fn simple_gaussian<R: Rng + ?Sized>(rng: &mut R, children: &mut [Individual<f64>]) {
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
