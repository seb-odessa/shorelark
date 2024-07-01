use rand::RngCore;

mod chromosome;
mod crossover;
mod genetic_algorithm;
mod mutation;
mod selection;
mod statistics;

pub use chromosome::Chromosome;
pub use crossover::UniformCrossover;
pub use genetic_algorithm::GeneticAlgorithm;
pub use mutation::GaussianMutation;
pub use selection::RouletteWheelSelection;
pub use statistics::Statistics;

pub trait Individual {
    fn create(chromosome: Chromosome) -> Self;
    fn fitness(&self) -> f32;
    fn chromosome(&self) -> &Chromosome;
}

pub trait SelectionMethod {
    fn select<'a, I>(&self, rng: &mut dyn RngCore, population: &'a [I]) -> &'a I
    where
        I: Individual;
}

pub trait CrossoverMethod {
    fn crossover(
        &self,
        rng: &mut dyn RngCore,
        parent_a: &Chromosome,
        parent_b: &Chromosome,
    ) -> Chromosome;
}

pub trait MutationMethod {
    fn mutate(&self, rng: &mut dyn RngCore, child: &mut Chromosome);
}
