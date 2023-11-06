use std::num::NonZeroUsize;

use rand::{seq::IteratorRandom, Rng};
use rand_distr::Bernoulli;

use super::Recombinator;

pub struct OnePoint;

impl Recombinator<i32, 2> for OnePoint {
    fn recombine<R: Rng + ?Sized>(&self, rng: &mut R, parents: &[&i32; 2]) -> [i32; 2] {
        const ALL_BITS: i32 = !0;

        let [&parent_1, &parent_2] = parents;
        let mask = rng.gen_range(1..i32::BITS);

        let child_1 = (parent_1 & (ALL_BITS << mask)) | (parent_2 & !(ALL_BITS << mask));
        let child_2 = (parent_2 & (ALL_BITS << mask)) | (parent_1 & !(ALL_BITS << mask));

        [child_1, child_2]
    }
}

pub struct NPoint {
    points: NonZeroUsize,
}

impl NPoint {
    pub fn new(points: NonZeroUsize) -> Self {
        Self { points }
    }
}

impl Recombinator<i32, 2> for NPoint {
    fn recombine<R: Rng + ?Sized>(&self, rng: &mut R, parents: &[&i32; 2]) -> [i32; 2] {
        const ALL_BITS: i32 = !0;

        let [&parent_1, &parent_2] = parents;

        let mut points = (1..i32::BITS).choose_multiple(rng, self.points.get());
        points.push(0);
        points.push(i32::BITS);
        points.sort_unstable();

        let mask = points.windows(2).step_by(2).fold(0, |acc, slice| {
            let [left_bound, right_bound]: [_; 2] = slice.try_into().unwrap();
            let one_bits = right_bound - left_bound;
            acc | (!(ALL_BITS << one_bits) << left_bound)
        });

        let child_1 = (parent_1 & mask) | (parent_2 & !mask);
        let child_2 = (parent_2 & mask) | (parent_1 & !mask);

        [child_1, child_2]
    }
}

pub struct Uniform;

impl Recombinator<i32, 2> for Uniform {
    fn recombine<R: Rng + ?Sized>(&self, rng: &mut R, parents: &[&i32; 2]) -> [i32; 2] {
        let [&parent_1, &parent_2] = parents;

        let distribution = Bernoulli::new(0.5).unwrap();

        let mask = (0..i32::BITS)
            .zip(rng.sample_iter(distribution))
            .filter(|(_, sample)| *sample)
            .fold(0, |acc, (bit, _)| acc | 1 << bit);

        let child_1 = (parent_1 & mask) | (parent_2 & !mask);
        let child_2 = (parent_2 & mask) | (parent_1 & !mask);

        [child_1, child_2]
    }
}
