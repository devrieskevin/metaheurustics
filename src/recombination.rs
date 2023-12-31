use std::marker::PhantomData;

use rand::Rng;
use rand_distr::Uniform;

use crate::parameter::{
    BoundedValue, BoundedVector, GaussianStrategyParameter, SelfAdaptiveGaussianVector,
};

pub mod integer;

pub trait Recombinator<T, const N: usize> {
    fn recombine<R: Rng + ?Sized>(&self, rng: &mut R, parents: &[&T; N]) -> [T; N];
}

pub struct Discrete;

impl Recombinator<BoundedVector<f64>, 2> for Discrete {
    fn recombine<R: Rng + ?Sized>(
        &self,
        rng: &mut R,
        parents: &[&BoundedVector<f64>; 2],
    ) -> [BoundedVector<f64>; 2] {
        let [parent_1, parent_2] = parents;

        let mut child_1 = BoundedVector::clone(parent_1);
        let mut child_2 = BoundedVector::clone(parent_2);

        let distribution = Uniform::new_inclusive(0, 1);

        child_1
            .value
            .iter_mut()
            .zip(child_2.value.iter_mut())
            .for_each(|(a, b)| {
                let values = [*a, *b];
                *a = values[rng.sample(distribution)];
                *b = values[rng.sample(distribution)];
            });

        [child_1, child_2]
    }
}

pub struct SingleArithmetic {
    alpha: f64,
}

impl SingleArithmetic {
    pub fn new(alpha: f64) -> Self {
        SingleArithmetic {
            alpha: alpha.clamp(0.0, 1.0),
        }
    }
}

impl Recombinator<BoundedVector<f64>, 2> for SingleArithmetic {
    fn recombine<R: Rng + ?Sized>(
        &self,
        rng: &mut R,
        parents: &[&BoundedVector<f64>; 2],
    ) -> [BoundedVector<f64>; 2] {
        let [parent_1, parent_2] = parents;
        let length = usize::min(parent_1.value.len(), parent_2.value.len());

        let mut cross_val;

        let mut child_1 = BoundedVector::clone(parent_1);
        let mut child_2 = BoundedVector::clone(parent_2);

        // Apply arithmetic average on allele of parents for allele of offspring
        let allele = rng.gen_range(0..length);

        // Child 1
        cross_val =
            self.alpha * parent_2.value[allele] + (1.0 - self.alpha) * parent_1.value[allele];

        child_1.value[allele] = cross_val;

        // Child 2
        cross_val =
            self.alpha * parent_1.value[allele] + (1.0 - self.alpha) * parent_2.value[allele];

        child_2.value[allele] = cross_val;

        [child_1, child_2]
    }
}

pub struct SimpleArithmetic {
    alpha: f64,
    cross_point: usize,
}

impl SimpleArithmetic {
    pub fn new(alpha: f64, cross_point: usize) -> Self {
        Self { alpha, cross_point }
    }
}

impl Recombinator<BoundedVector<f64>, 2> for SimpleArithmetic {
    fn recombine<R: Rng + ?Sized>(
        &self,
        _rng: &mut R,
        parents: &[&BoundedVector<f64>; 2],
    ) -> [BoundedVector<f64>; 2] {
        let [parent_1, parent_2] = parents;
        let mut child_1 = BoundedVector::clone(parent_1);
        let mut child_2 = BoundedVector::clone(parent_2);

        child_1
            .value
            .iter_mut()
            .zip(child_2.value.iter_mut())
            .skip(self.cross_point)
            .for_each(|(ref_1, ref_2)| {
                let value_1 = *ref_1;
                let value_2 = *ref_2;
                *ref_1 = (1.0 - self.alpha) * value_1 + self.alpha * value_2;
                *ref_2 = self.alpha * value_1 + (1.0 - self.alpha) * value_2;
            });

        [child_1, child_2]
    }
}

pub struct WholeArithmetic {
    alpha: f64,
}

impl WholeArithmetic {
    pub fn new(alpha: f64) -> Self {
        Self { alpha }
    }
}

impl Recombinator<BoundedVector<f64>, 2> for WholeArithmetic {
    fn recombine<R: Rng + ?Sized>(
        &self,
        rng: &mut R,
        parents: &[&BoundedVector<f64>; 2],
    ) -> [BoundedVector<f64>; 2] {
        SimpleArithmetic::new(self.alpha, 0).recombine(rng, parents)
    }
}

impl Recombinator<BoundedValue<f64>, 2> for WholeArithmetic {
    fn recombine<R: Rng + ?Sized>(
        &self,
        _rng: &mut R,
        parents: &[&BoundedValue<f64>; 2],
    ) -> [BoundedValue<f64>; 2] {
        let [parent_1, parent_2] = parents;

        let mut child_1 = BoundedValue::clone(parent_1);
        let mut child_2 = BoundedValue::clone(parent_2);

        let x = child_1.value;
        let y = child_2.value;

        child_1.value = self.alpha * x + (1.0 - self.alpha) * y;
        child_2.value = self.alpha * y + (1.0 - self.alpha) * x;

        [child_1, child_2]
    }
}

pub struct BlendCrossover {
    alpha: f64,
}

impl BlendCrossover {
    pub fn new(alpha: f64) -> Self {
        Self { alpha }
    }
}

impl Recombinator<BoundedVector<f64>, 2> for BlendCrossover {
    fn recombine<R: Rng + ?Sized>(
        &self,
        rng: &mut R,
        parents: &[&BoundedVector<f64>; 2],
    ) -> [BoundedVector<f64>; 2] {
        let [parent_1, parent_2] = parents;
        let mut child_1 = BoundedVector::clone(parent_1);
        let mut child_2 = BoundedVector::clone(parent_2);

        child_1
            .value
            .iter_mut()
            .zip(child_2.value.iter_mut())
            .for_each(|(ref_1, ref_2)| {
                let value_1 = *ref_1;
                let value_2 = *ref_2;
                let distance = f64::abs(value_1 - value_2);
                let min = f64::min(value_1, value_2) - self.alpha * distance;
                let max = f64::max(value_1, value_2) + self.alpha * distance;
                let distribution = Uniform::new(min, max);
                *ref_1 = rng
                    .sample(distribution)
                    .clamp(child_1.min_value, child_1.max_value);
                *ref_2 = rng
                    .sample(distribution)
                    .clamp(child_2.min_value, child_2.max_value);
            });

        [child_1, child_2]
    }
}

pub struct SelfAdaptiveGaussianVectorRecombinator<TR, T, SR, S, const N: usize>
where
    TR: Recombinator<BoundedVector<T>, N>,
    T: PartialOrd,
    SR: Recombinator<S, N>,
    S: GaussianStrategyParameter,
{
    value_recombinator: TR,
    strategy_parameter_recombinator: SR,
    _markers: PhantomData<(T, S)>,
}

impl<TR, SR> Recombinator<SelfAdaptiveGaussianVector<f64, BoundedValue<f64>>, 2>
    for SelfAdaptiveGaussianVectorRecombinator<TR, f64, SR, BoundedValue<f64>, 2>
where
    TR: Recombinator<BoundedVector<f64>, 2>,
    SR: Recombinator<BoundedValue<f64>, 2>,
{
    fn recombine<R: Rng + ?Sized>(
        &self,
        rng: &mut R,
        parents: &[&SelfAdaptiveGaussianVector<f64, BoundedValue<f64>>; 2],
    ) -> [SelfAdaptiveGaussianVector<f64, BoundedValue<f64>>; 2] {
        let [parent_1, parent_2] = parents;

        let parent_values = [&parent_1.value, &parent_2.value];
        let parent_strategy_parameters =
            [&parent_1.strategy_parameter, &parent_2.strategy_parameter];

        let recombined_values = self.value_recombinator.recombine(rng, &parent_values);
        let recombined_strategy_parameters = self
            .strategy_parameter_recombinator
            .recombine(rng, &parent_strategy_parameters);

        let children_vector: Vec<_> = recombined_values
            .into_iter()
            .zip(recombined_strategy_parameters)
            .map(|(value, strategy_parameter)| SelfAdaptiveGaussianVector {
                value,
                strategy_parameter,
            })
            .collect();

        match children_vector.try_into() {
            Ok(children) => children,
            _ => panic!("Fatal error. Unexpected amount of children created."),
        }
    }
}
