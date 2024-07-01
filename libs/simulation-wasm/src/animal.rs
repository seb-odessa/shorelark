use lib_simulation as sim;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Debug, Copy)]
pub struct Animal {
    pub x: f32,
    pub y: f32,
    pub rotation: f32,
}
// ^ Эта модель меньше `lib_simulation::Animal`, поскольку
// | позиция птицы - это все, что нам нужно на данный момент
// | на стороне JS, другие поля нам пока не нужны.

impl From<&sim::Animal> for Animal {
    fn from(animal: &sim::Animal) -> Self {
        Self {
            x: animal.position().x,
            y: animal.position().y,
            rotation: animal.rotation().angle(),
        }
    }
}