use crate::{Individual, SelectionMethod};
use rand::{seq::SliceRandom, RngCore};

pub struct RouletteWheelSelection;
impl SelectionMethod for RouletteWheelSelection {
    fn select<'a, R, I>(&self, rng: &mut R, population: &'a [I]) -> &'a I
    where
        R: RngCore,
        I: Individual,
    {
        population
            .choose_weighted(rng, |individual| individual.fitness())
            .expect("получена пустая популяция")
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use crate::Chromosome;

    use super::*;
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;

    #[derive(Clone, Debug)]
    struct TestIndividual {
        fitness: f32,
    }

    impl TestIndividual {
        fn new(fitness: f32) -> Self {
            Self { fitness }
        }
    }

    impl Individual for TestIndividual {
        fn create(_: Chromosome) -> Self {
            todo!()
         }
        fn fitness(&self) -> f32 {
            self.fitness
        }
        fn chromosome(&self) -> &Chromosome {
            panic!("не поддерживается для TestIndividual")
        }
    }

    #[test]
    fn roulette_wheel_selection() {
        let mut rng = ChaCha8Rng::from_seed(Default::default());

        let population = vec![
            TestIndividual::new(2.0),
            TestIndividual::new(1.0),
            TestIndividual::new(4.0),
            TestIndividual::new(3.0),
        ];

        let mut actual_histogram = BTreeMap::new();

        for _ in 0..1000 {
            let fitness = RouletteWheelSelection
                .select(&mut rng, &population)
                .fitness() as i32;

            *actual_histogram.entry(fitness).or_insert(0) += 1;
        }

        let expected_histogram = BTreeMap::from_iter([(1, 98), (2, 202), (3, 278), (4, 422)]);

        assert_eq!(actual_histogram, expected_histogram);
    }
}
