use rand::{distributions::Slice, Rng};
use rand_distr::Uniform;

use super::Mutator;

pub struct RandomResetting<T> {
    probability: f64,
    min_value: T,
    max_value: T,
}

impl<T> RandomResetting<T> {
    pub fn new(probability: f64, min_value: T, max_value: T) -> Self {
        Self {
            probability,
            min_value,
            max_value,
        }
    }
}

impl Mutator<i32> for RandomResetting<i32> {
    fn mutate<'a, R: Rng + ?Sized>(&self, rng: &mut R, parameter: &'a mut i32) -> &'a mut i32 {
        let random_value = rng.sample(Uniform::new(0.0, 1.0));
        if random_value <= self.probability {
            let distribution = Uniform::new_inclusive(self.min_value, self.max_value);
            *parameter = rng.sample(distribution);
        }
        parameter
    }
}

pub struct SimpleCreep<T> {
    probability: f64,
    min_value: T,
    max_value: T,
    step_size: T,
}

impl SimpleCreep<i32> {
    pub fn new(probability: f64, min_value: i32, max_value: i32, step_size: i32) -> Self {
        Self {
            probability,
            min_value,
            max_value,
            step_size,
        }
    }
}

impl Mutator<i32> for SimpleCreep<i32> {
    fn mutate<'a, R: Rng + ?Sized>(&self, rng: &mut R, parameter: &'a mut i32) -> &'a mut i32 {
        let step_choices = [-self.step_size, self.step_size];
        let random_value = rng.sample(Uniform::new(0.0, 1.0));
        if random_value <= self.probability {
            let distribution = Slice::new(&step_choices).unwrap();
            let random_value = rng.sample(distribution);
            *parameter = (*parameter + random_value).clamp(self.min_value, self.max_value);
        }
        parameter
    }
}

#[cfg(test)]
mod tests {
    use rand::{rngs::StdRng, SeedableRng};

    use super::*;

    #[test]
    fn test_random_resetting() {
        let mut rng = StdRng::seed_from_u64(1234);
        let mut parameter = 0;
        let mutator = RandomResetting::new(1.0, 0, 10);
        mutator.mutate(&mut rng, &mut parameter);
        assert!((0..=10).contains(&parameter));
    }

    #[test]
    fn test_simple_creep() {
        let mut rng = StdRng::seed_from_u64(1234);
        let mut parameter = 0;
        let mutator = SimpleCreep::new(1.0, 0, 10, 1);
        mutator.mutate(&mut rng, &mut parameter);
        assert!((0..=10).contains(&parameter));
    }
}
