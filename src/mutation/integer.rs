use rand::Rng;
use rand_distr::Bernoulli;
use rand_distr::Uniform;

use super::Mutator;

pub struct BitFlip<T>
where
    T: PartialOrd,
{
    probability: f64,
    max_bit_count: u32,
    min_value: T,
    max_value: T,
}

impl<T> BitFlip<T>
where
    T: PartialOrd,
{
    pub fn new(probability: f64, max_bit_count: u32, min_value: T, max_value: T) -> Self {
        Self {
            probability: probability.clamp(0.0, 1.0),
            max_bit_count,
            min_value,
            max_value,
        }
    }
}

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

pub struct SimpleCreep<T> {
    probability: f64,
    min_value: T,
    max_value: T,
    step_size: T,
}

impl<T> SimpleCreep<T> {
    pub fn new(probability: f64, min_value: T, max_value: T, step_size: T) -> Self {
        Self {
            probability,
            min_value,
            max_value,
            step_size,
        }
    }
}

macro_rules! int_mutator_impl {
    ($($Int:ty)+) => {
        $(
            impl Mutator<$Int> for BitFlip<$Int> {
                fn mutate<'a, R: Rng + ?Sized>(&self, rng: &mut R, parameter: &'a mut $Int) -> &'a mut $Int {
                    let bitmask = rng
                        .sample_iter(Uniform::new(0.0, 1.0))
                        .zip(0..<$Int>::BITS)
                        .take_while(|(_, i)| *i < self.max_bit_count)
                        .filter(|(prob, _)| *prob <= self.probability)
                        .map(|(_, i)| 1 << i)
                        .fold(0, |acc, val| acc | val);

                    *parameter = (*parameter ^ bitmask).clamp(self.min_value, self.max_value);

                    parameter
                }
            }

            impl Mutator<$Int> for RandomResetting<$Int> {
                fn mutate<'a, R: Rng + ?Sized>(&self, rng: &mut R, parameter: &'a mut $Int) -> &'a mut $Int {
                    let random_value = rng.sample(Uniform::new(0.0, 1.0));
                    if random_value <= self.probability {
                        let distribution = Uniform::new_inclusive(self.min_value, self.max_value);
                        *parameter = rng.sample(distribution);
                    }
                    parameter
                }
            }

            impl Mutator<$Int> for SimpleCreep<$Int> {
                fn mutate<'a, R: Rng + ?Sized>(&self, rng: &mut R, parameter: &'a mut $Int) -> &'a mut $Int {
                    if rng.sample(Bernoulli::new(self.probability).unwrap()) {
                        let distribution = Bernoulli::new(0.5).unwrap();
                        let parameter_raw = if rng.sample(distribution) {
                            parameter.checked_add(self.step_size).unwrap_or(self.max_value)
                        } else {
                            parameter.checked_sub(self.step_size).unwrap_or(self.min_value)
                        };
                        *parameter = parameter_raw.clamp(self.min_value, self.max_value);
                    }
                    parameter
                }
            }
        )+
    };
}

int_mutator_impl!(u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize);

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
