use crate::{Brain, Eye, Point, Rotation};
use lib_genetic_algorithm as ga;
use rand::{Rng, RngCore};

#[derive(Debug)]
pub struct Animal {
    pub(crate) position: Point,
    pub(crate) rotation: Rotation,
    pub(crate) speed: f32,
    pub(crate) eye: Eye,
    pub(crate) brain: Brain,
    pub(crate) satiation: usize,
}
impl Animal {
    pub fn random(rng: &mut dyn RngCore) -> Self {
        let eye = Eye::default();
        let brain = Brain::random(rng, &eye);

        Self {
            position: rng.gen(),
            // ------ ^-------^
            // | Если бы не `rand-no-std`, нам пришлось бы делать
            // | `na::Point2::new(rng.gen(), rng.gen())`
            // ---
            rotation: rng.gen(),
            speed: 0.002,
            eye,
            brain,
            satiation: 0,
        }
    }

    pub(crate) fn from_chromosome(chromosome: ga::Chromosome, rng: &mut dyn RngCore) -> Self {
        let eye = Eye::default();
        let brain = Brain::from_chromosome(chromosome, &eye);

        Self::new(eye, brain, rng)
    }

    pub(crate) fn as_chromosome(&self) -> ga::Chromosome {
        // We evolve only our birds' brains, but technically there's no
        // reason not to simulate e.g. physical properties such as size.
        //
        // If that was to happen, this function could be adjusted to
        // return a longer chromosome that encodes not only the brain,
        // but also, say, birdie's color.

        self.brain.as_chromosome()
    }

    fn new(eye: Eye, brain: Brain, rng: &mut dyn RngCore) -> Self {
        Self {
            position: rng.gen(),
            rotation: rng.gen(),
            speed: 0.002,
            eye,
            brain,
            satiation: 0,
        }
    }

    pub fn position(&self) -> Point {
        // ------------------ ^
        // | Нет необходимости возвращать ссылку, поскольку `na::Point2` является копируемым (реализует типаж `Copy`).
        // |
        // | (он настолько маленький, что клонирование дешевле, чем возня с ссылками)
        // ---

        self.position
    }

    pub fn rotation(&self) -> Rotation {
        self.rotation
    }
}
