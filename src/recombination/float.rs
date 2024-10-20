use rand::Rng;

use super::{
    integer::{NPoint, OnePoint, Uniform},
    Recombinator,
};

macro_rules! float_bits_recombination_impl {
    ($($float:ty)+) => {
        $(
            impl Recombinator<$float, 2> for OnePoint {
                fn recombine<R: Rng + ?Sized>(&self, rng: &mut R, parents: &[&$float; 2]) -> [$float; 2] {
                    let bit_parents = [parents[0].to_bits(), parents[1].to_bits()];
                    let bit_parent_references = [&bit_parents[0], &bit_parents[1]];
                    let bit_children = self.recombine(rng, &bit_parent_references);

                    [
                        <$float>::from_bits(bit_children[0]),
                        <$float>::from_bits(bit_children[1]),
                    ]
                }
            }

            impl Recombinator<$float, 2> for NPoint {
                fn recombine<R: Rng + ?Sized>(&self, rng: &mut R, parents: &[&$float; 2]) -> [$float; 2] {
                    let bit_parents = [parents[0].to_bits(), parents[1].to_bits()];
                    let bit_parent_references = [&bit_parents[0], &bit_parents[1]];
                    let bit_children = self.recombine(rng, &bit_parent_references);
                    [
                        <$float>::from_bits(bit_children[0]),
                        <$float>::from_bits(bit_children[1]),
                    ]
                }
            }

            impl Recombinator<$float, 2> for Uniform {
                fn recombine<R: Rng + ?Sized>(&self, rng: &mut R, parents: &[&$float; 2]) -> [$float; 2] {
                    let bit_parents = [parents[0].to_bits(), parents[1].to_bits()];
                    let bit_parent_references = [&bit_parents[0], &bit_parents[1]];
                    let bit_children = self.recombine(rng, &bit_parent_references);
                    [
                        <$float>::from_bits(bit_children[0]),
                        <$float>::from_bits(bit_children[1]),
                    ]
                }
            }
        )+
    };
}

float_bits_recombination_impl!(f32 f64);
