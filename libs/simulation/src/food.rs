use rand::{Rng, RngCore};
use crate::Point;

#[derive(Debug)]
pub struct Food {
    pub(crate) position: Point,
}
impl Food {
    pub fn random(rng: &mut dyn RngCore) -> Self {
        Self {
            position: rng.gen(),
        }
    }

    pub fn position(&self) -> Point {
        self.position
    }
}