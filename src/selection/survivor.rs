use crate::population::Population;

pub fn replace_worst_selection(
    population: &mut Population<f64>,
    offspring: &mut Population<f64>,
    replacement_rate: f64,
) {
    population.individuals.sort_by(|a, b| b.compare_fitness(a));
    offspring.individuals.sort_by(|a, b| a.compare_fitness(b));

    let replacement_count = (replacement_rate * population.individuals.len() as f64) as usize;
    if replacement_count > population.individuals.len() {
        panic!("Replacement count must be less than population size");
    }

    for n in (population.individuals.len() - replacement_count)..population.individuals.len() {
        population.individuals[n] = offspring.individuals[n].clone();
    }
}
