use super::{
    integer::{BitFlip, RandomResetting},
    Mutator,
};

macro_rules! float_bit_mutator_impl {
    ($($float:ty; $int:ty)+) => {
        $(
            impl Mutator<$float> for RandomResetting<$float> {
                fn mutate<'a, R: rand::Rng + ?Sized>(
                    &self,
                    rng: &mut R,
                    parameter: &'a mut $float,
                ) -> &'a mut $float {
                    let mut bits = parameter.to_bits();

                    let mutator = RandomResetting::new(self.get_probability(), <$int>::MIN, <$int>::MAX);
                    mutator.mutate(rng, &mut bits);

                    *parameter = <$float>::from_bits(bits).clamp(*self.get_min_value(), *self.get_max_value());

                    parameter
                }
            }

            impl Mutator<$float> for BitFlip<$float> {
                fn mutate<'a, R: rand::Rng + ?Sized>(
                    &self,
                    rng: &mut R,
                    parameter: &'a mut $float,
                ) -> &'a mut $float {
                    let mut bits = parameter.to_bits();

                    let mutator = BitFlip::new(
                        self.get_probability(),
                        self.get_max_bit_count(),
                        <$int>::MIN,
                        <$int>::MAX,
                    );
                    mutator.mutate(rng, &mut bits);

                    *parameter = <$float>::from_bits(bits).clamp(*self.get_min_value(), *self.get_max_value());

                    parameter
                }
            }

        )+
    };
}

float_bit_mutator_impl!(f32;u32 f64;u64);
