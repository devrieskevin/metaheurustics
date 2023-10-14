use rand::Rng;
use rand_distr::Uniform;

#[allow(deprecated)]
use crate::{individual::BasicIndividual, parameter::BoundedVector, population::BasicPopulation};

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

#[allow(deprecated)]
#[deprecated(note = "Uses `BasicIndividual` struct prototype. Use `SingleArithmetic` instead.")]
/// Applies single arithmetic recombination on a [`Vec<Individual<f64>>`].
pub fn single_arithmetic<R: Rng + ?Sized>(
    rng: &mut R,
    mating_pool: Vec<BasicIndividual<f64>>,
    alpha: f64,
) -> BasicPopulation<f64> {
    let mut offspring = Vec::with_capacity(mating_pool.len());

    let min_val = mating_pool[0].min_value;
    let max_val = mating_pool[0].max_value;
    let length = mating_pool[0].value.len();

    let mut cross_val;
    let mut cross_dev;
    let mut allele;

    for i in (0..mating_pool.len()).step_by(2) {
        let mut child_1 = BasicIndividual::new_empty(min_val, max_val, length);
        let mut child_2 = BasicIndividual::new_empty(min_val, max_val, length);

        // Copy values into offspring
        for j in 0..length {
            child_1.value[j] = mating_pool[i].value[j];
            child_2.value[j] = mating_pool[i + 1].value[j];

            child_1.std_dev[j] = mating_pool[i].std_dev[j];
            child_2.std_dev[j] = mating_pool[i + 1].std_dev[j];
        }

        // Apply arithmetic average on allele of parents for allele of offspring
        allele = rng.gen_range(0..length);

        // Child 1
        cross_val =
            alpha * mating_pool[i + 1].value[allele] + (1.0 - alpha) * mating_pool[i].value[allele];

        child_1.value[allele] = cross_val;

        cross_dev = alpha * mating_pool[i + 1].std_dev[allele]
            + (1.0 - alpha) * mating_pool[i].std_dev[allele];

        child_1.std_dev[allele] = cross_dev;

        // Child 2
        cross_val =
            alpha * mating_pool[i].value[allele] + (1.0 - alpha) * mating_pool[i + 1].value[allele];

        child_2.value[allele] = cross_val;

        cross_dev = alpha * mating_pool[i].std_dev[allele]
            + (1.0 - alpha) * mating_pool[i + 1].std_dev[allele];

        child_2.std_dev[allele] = cross_dev;

        offspring.push(child_1);
        offspring.push(child_2);
    }

    BasicPopulation::new_from_individuals(offspring)
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

#[allow(deprecated)]
#[deprecated(note = "Uses `BasicIndividual` struct prototype. Use `SimpleArithmetic` instead.")]
/// Applies simple arithmetic recombination on a [`&[Individual<f64>]`].
pub fn simple_arithmetic(
    mating_pool: &[BasicIndividual<f64>],
    alpha: f64,
    cross_point: usize,
) -> BasicPopulation<f64> {
    let min_value = mating_pool[0].min_value;
    let max_value = mating_pool[0].max_value;
    let value_length = mating_pool[0].value.len();
    let pool_size = mating_pool.len();

    let mut offspring =
        vec![BasicIndividual::new_empty(min_value, max_value, value_length); pool_size];
    for n in (0..pool_size).step_by(2) {
        offspring[n].value[..cross_point].copy_from_slice(&mating_pool[n].value[..cross_point]);
        offspring[n + 1].value[..cross_point]
            .copy_from_slice(&mating_pool[n + 1].value[..cross_point]);

        offspring[n].std_dev[..cross_point].copy_from_slice(&mating_pool[n].std_dev[..cross_point]);
        offspring[n + 1].std_dev[..cross_point]
            .copy_from_slice(&mating_pool[n + 1].std_dev[..cross_point]);

        for m in cross_point..pool_size {
            offspring[n].value[m] =
                alpha * mating_pool[n + 1].value[m] * (1.0 - alpha) * mating_pool[n].value[m];
            offspring[n].std_dev[m] =
                alpha * mating_pool[n + 1].std_dev[m] * (1.0 - alpha) * mating_pool[n].std_dev[m];

            offspring[n + 1].value[m] =
                alpha * mating_pool[n].value[m] * (1.0 - alpha) * mating_pool[n + 1].value[m];
            offspring[n + 1].std_dev[m] =
                alpha * mating_pool[n].std_dev[m] * (1.0 - alpha) * mating_pool[n + 1].std_dev[m];
        }
    }

    BasicPopulation::new_from_individuals(offspring)
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

#[allow(deprecated)]
#[deprecated(note = "Uses `BasicIndividual` struct prototype. Use `BlendCrossover` instead.")]
/// Applies blend crossover on a [`&[Individual<f64>]`].
pub fn blend_crossover<R: Rng + ?Sized>(
    rng: &mut R,
    mating_pool: Vec<BasicIndividual<f64>>,
    alpha: f64,
) -> BasicPopulation<f64> {
    let min_value = mating_pool[0].min_value;
    let max_value = mating_pool[0].max_value;
    let value_length = mating_pool[0].value.len();
    let pool_size = mating_pool.len();
    let distribution = Uniform::new(0.0, 1.0);

    let mut offspring =
        vec![BasicIndividual::new_empty(min_value, max_value, value_length); pool_size];
    let mut gamma;
    for n in (0..pool_size).step_by(2) {
        // Sample blend parameter
        gamma = (1.0 + 2.0 * alpha) * rng.sample(distribution) - alpha;

        for m in 0..value_length {
            // Child 1
            offspring[n].value[m] =
                gamma * mating_pool[n + 1].value[m] + (1.0 - gamma) * mating_pool[n].value[m];
            offspring[n].std_dev[m] =
                gamma * mating_pool[n + 1].std_dev[m] + (1.0 - gamma) * mating_pool[n].std_dev[m];

            // Child 2
            offspring[n + 1].value[m] =
                gamma * mating_pool[n].value[m] + (1.0 - gamma) * mating_pool[n + 1].value[m];
            offspring[n + 1].std_dev[m] =
                gamma * mating_pool[n].std_dev[m] + (1.0 - gamma) * mating_pool[n + 1].std_dev[m];
        }
    }

    BasicPopulation::new_from_individuals(offspring)
}
