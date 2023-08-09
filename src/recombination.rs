use ndarray_rand::rand::{thread_rng, Rng};

use crate::{individual::Individual, population::Population};

/// Applies single arithmetic recombination on a [`Vec<Individual<f64>>`].
pub fn single_arithmetic(mating_pool: Vec<Individual<f64>>, alpha: f64) -> Population<f64> {
    let mut offspring = Vec::with_capacity(mating_pool.len());

    let min_val = mating_pool[0].min_value;
    let max_val = mating_pool[0].max_value;
    let length = mating_pool[0].value.len();

    let mut cross_val;
    let mut cross_dev;
    let mut allele;
    let mut rng = thread_rng();

    for i in (0..mating_pool.len()).step_by(2) {
        let mut child_1 = Individual::new_empty(min_val, max_val, length);
        let mut child_2 = Individual::new_empty(min_val, max_val, length);

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

    Population::new_from_individuals(offspring)
}
