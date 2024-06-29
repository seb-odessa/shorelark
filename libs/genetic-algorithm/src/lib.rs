use rand::RngCore;

mod chromosome;
mod crossover;
mod genetic_algorithm;
mod mutation;
mod selection;

pub use chromosome::Chromosome;
pub use crossover::UniformCrossover;
pub use genetic_algorithm::GeneticAlgorithm;
pub use mutation::GaussianMutation;
pub use selection::RouletteWheelSelection;

pub trait Individual {
    fn create(chromosome: Chromosome) -> Self;
    fn fitness(&self) -> f32;
    fn chromosome(&self) -> &Chromosome;
}

pub trait SelectionMethod {
    fn select<'a, R, I>(&self, rng: &mut R, population: &'a [I]) -> &'a I
    where
        R: RngCore,
        I: Individual;
}

pub trait CrossoverMethod {
    fn crossover<R>(&self, rng: &mut R, parent_a: &Chromosome, parent_b: &Chromosome) -> Chromosome
    where
        R: RngCore;
}

pub trait MutationMethod {
    fn mutate<R>(&self, rng: &mut R, child: &mut Chromosome)
    where
        R: RngCore;
}
