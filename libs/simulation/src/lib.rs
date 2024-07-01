use nalgebra as na;

mod animal;
mod animal_individual;
mod brain;
mod eye;
mod food;
mod simulation;
mod word;

pub use animal::Animal;
pub use animal_individual::AnimalIndividual;
pub use brain::Brain;
pub use eye::Eye;
pub use food::Food;
pub use simulation::Simulation;
pub use word::World;

pub type Point = na::Point2<f32>;
pub type Rotation = na::Rotation2<f32>;
