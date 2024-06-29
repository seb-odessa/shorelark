use rand::RngCore;

use crate::{CrossoverMethod, Individual, MutationMethod, SelectionMethod};

pub struct GeneticAlgorithm<S, C, M> {
    selection_method: S,
    crossover_method: C,
    mutation_method: M,
}

impl<S, C, M> GeneticAlgorithm<S, C, M>
where
    S: SelectionMethod,
    C: CrossoverMethod,
    M: MutationMethod,
{
    pub fn new(selection_method: S, crossover_method: C, mutation_method: M) -> Self {
        Self {
            selection_method,
            crossover_method,
            mutation_method,
        }
    }

    pub fn evolve<R, I>(&self, rng: &mut R, population: &[I]) -> Vec<I>
    where
        R: RngCore,
        I: Individual,
    {
        assert!(!population.is_empty());

        (0..population.len())
            .map(|_| {
                let parent_a = self.selection_method.select(rng, population);
                let parent_b = self.selection_method.select(rng, population);
                let mut child = self.crossover_method.crossover(
                    rng,
                    parent_a.chromosome(),
                    parent_b.chromosome(),
                );
                self.mutation_method.mutate(rng, &mut child);
                I::create(child)
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        crossover::UniformCrossover, mutation::GaussianMutation, Chromosome, RouletteWheelSelection,
    };
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;

    #[derive(Clone, Debug, PartialEq)]
    enum TestIndividual {
        /// Для тестов, которым нужен доступ к хромосоме
        WithChromosome { chromosome: Chromosome },

        /// Для тестов, которым не нужен доступ к хромосоме
        #[allow(dead_code)]
        WithFitness { fitness: f32 },
    }

    impl PartialEq for Chromosome {
        fn eq(&self, other: &Self) -> bool {
            approx::relative_eq!(self.genes.as_slice(), other.genes.as_slice())
        }
    }

    impl Individual for TestIndividual {
        fn create(chromosome: Chromosome) -> Self {
            Self::WithChromosome { chromosome }
        }

        fn chromosome(&self) -> &Chromosome {
            match self {
                Self::WithChromosome { chromosome } => chromosome,

                Self::WithFitness { .. } => {
                    panic!("не поддерживается для TestIndividual::WithFitness")
                }
            }
        }

        fn fitness(&self) -> f32 {
            match self {
                Self::WithChromosome { chromosome } => {
                    chromosome.iter().sum()
                    // ^ самая простая на свете фитнес-функция:
                    // мы просто складываем все гены вместе
                }

                Self::WithFitness { fitness } => *fitness,
            }
        }
    }

    #[test]
    fn genetic_algorithm() {
        fn individual(genes: &[f32]) -> TestIndividual {
            TestIndividual::create(genes.iter().cloned().collect())
        }

        let mut rng = ChaCha8Rng::from_seed(Default::default());

        let ga = GeneticAlgorithm::new(
            RouletteWheelSelection,
            UniformCrossover,
            GaussianMutation::new(0.5, 0.5),
        );

        let mut population = vec![
            individual(&[0.0, 0.0, 0.0]),
            individual(&[1.0, 1.0, 1.0]),
            individual(&[1.0, 2.0, 1.0]),
            individual(&[1.0, 2.0, 4.0]),
        ];

        // Мы запускаем `.evolve()` несколько раз, чтобы легче было заметить разницу
        // между входной и выходной популяциями.
        //
        // 10 взято с потолка, с тем же успехом мы можем взять 5, 20 или даже
        // 1000 поколений: единственное, что будет меняться - магнитуда
        // разницы между популяциями.
        for _ in 0..10 {
            population = ga.evolve(&mut rng, &population);
        }

        let expected_population = vec![
            individual(&[0.44769490, 2.0648358, 4.3058133]),
            individual(&[1.21268670, 1.5538777, 2.8869110]),
            individual(&[1.06176780, 2.2657390, 4.4287640]),
            individual(&[0.95909685, 2.4618788, 4.0247330]),
        ];

        assert_eq!(population, expected_population);
    }
}
