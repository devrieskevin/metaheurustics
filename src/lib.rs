pub mod benchmark;
pub mod individual;
pub mod mutation;
pub mod parameter;
pub mod population;
pub mod recombination;
pub mod samplers;
pub mod selection;

#[cfg(test)]
mod tests {
    use rand::{rngs::StdRng, SeedableRng};

    use crate::{
        benchmark::bent_cigar, mutation, population::Population, recombination, selection,
    };

    #[test]
    fn test_improvement_per_epoch() {
        let mut rng: StdRng = SeedableRng::seed_from_u64(1234);
        let evaluation_func = |x: &[f64]| -1.0 * bent_cigar(x);
        let alpha = 0.5;
        let mutation_probability = 0.01;
        let replacement_rate = 0.9;

        // Initialize population
        let mut population = Population::new(&mut rng, -100.0, 100.0, 10, 1000);

        // Set fitness
        population.individuals.iter_mut().for_each(|individual| {
            individual.set_fitness(evaluation_func(&individual.value));
        });

        let mut last_max_fitness = population
            .individuals
            .iter()
            .map(|individual| individual.fitness)
            .max_by(|a, b| a.total_cmp(b))
            .unwrap();
        for _ in 0..100 {
            // Parent selection
            let mating_pool = selection::parent::fitness_proportionate_selection(
                &mut rng,
                &population,
                population.individuals.len(),
            );

            // Generate offspring
            let mut offspring =
                recombination::single_arithmetic(&mut rng, mating_pool.individuals, alpha);
            mutation::uniform(&mut rng, mutation_probability, &mut offspring.individuals);

            // Set offspring fitness
            offspring.individuals.iter_mut().for_each(|individual| {
                individual.set_fitness(evaluation_func(&individual.value));
            });

            // Select survivors
            selection::survivor::replace_worst_selection(
                &mut population,
                &mut offspring,
                replacement_rate,
            );

            let max_fitness = population
                .individuals
                .iter()
                .map(|individual| individual.fitness)
                .max_by(|a, b| a.total_cmp(b))
                .unwrap();

            assert!(max_fitness >= last_max_fitness);
            last_max_fitness = max_fitness;
        }
    }
}
