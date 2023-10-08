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
        benchmark::bent_cigar,
        individual::{
            BoundedVectorIndividualMutator, BoundedVectorIndividualRecombinator, Individual,
        },
        mutation::{Mutator, UniformMutator},
        population::Population,
        recombination::{Recombinator, SingleArithmetic},
        selection::{
            parent::{ParentSelector, UniformSelector},
            survivor::{ReplaceWorstSelector, SurvivorSelector},
        },
    };

    #[test]
    fn test_improvement_per_epoch() {
        let mut rng: StdRng = SeedableRng::seed_from_u64(1234);
        let evaluation_func = |x: &[f64]| -1.0 * bent_cigar(x);
        let alpha = 0.5;
        let mutation_probability = 0.01;
        let replacement_rate = 0.9;

        let parent_selector = UniformSelector::new();
        let recombinator = BoundedVectorIndividualRecombinator::new(SingleArithmetic::new(alpha));
        let mutator =
            BoundedVectorIndividualMutator::new(UniformMutator::new(mutation_probability));
        let survivor_selector = ReplaceWorstSelector::new(replacement_rate);

        // Initialize population
        let mut population = Population::new(&mut rng, -100.0, 100.0, 10, 1000);

        // Set fitness
        population
            .individuals_mut()
            .iter_mut()
            .for_each(|individual| {
                individual.set_fitness(evaluation_func(&individual.vector().value));
            });

        let mut last_max_fitness = population
            .individuals()
            .iter()
            .map(|individual| individual.fitness())
            .max_by(|a, b| a.total_cmp(b))
            .unwrap();
        for _ in 0..100 {
            // Parent selection
            let mating_pool: Vec<_> = parent_selector.select(
                &mut rng,
                population.individuals(),
                population.individuals().len(),
            );

            // Generate offspring
            let mut offspring: Vec<_> = mating_pool
                .chunks(2)
                .map(|x| x.iter().map(|y| *y).collect())
                .flat_map(|x: Vec<_>| recombinator.recombine(&mut rng, x[..].try_into().unwrap()))
                .collect();

            // Mutate offspring and set offspring fitness
            offspring.iter_mut().for_each(|individual| {
                mutator.mutate(&mut rng, individual);
                individual.set_fitness(evaluation_func(&individual.vector().value));
            });

            // Select survivors
            survivor_selector.select(&mut rng, population.individuals_mut(), offspring);

            let max_fitness = population
                .individuals()
                .iter()
                .map(|individual| individual.fitness())
                .max_by(|a, b| a.total_cmp(b))
                .unwrap();

            assert!(max_fitness >= last_max_fitness);
            last_max_fitness = max_fitness;
        }
    }
}
