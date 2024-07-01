use rand::RngCore;
use crate::{Animal, Food};

#[derive(Debug)]
pub struct World {
    pub(crate) animals: Vec<Animal>,
    pub(crate) foods: Vec<Food>,
}
impl World {
    pub fn random(rng: &mut dyn RngCore) -> Self {
        let animals = (0..10)
            .map(|_| Animal::random(rng))
            .collect();

        let foods = (0..60)
            .map(|_| Food::random(rng))
            .collect();

        // ^ Наш алгоритм позволяет животным и еде накладываться друг на друга,
        // | это не идеально, но для наших целей сойдет.
        // |
        // | Более сложное решение может быть основано, например, на
        // | избыточной выборке сглаживания:
        // |
        // | https://en.wikipedia.org/wiki/Supersampling
        // ---

        Self { animals, foods }
    }

    pub fn animals(&self) -> &[Animal] {
        &self.animals
    }

    pub fn foods(&self) -> &[Food] {
        &self.foods
    }

}