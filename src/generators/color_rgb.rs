use rand::RngExt;

use crate::value::{Generator, Value};

pub struct ColorRgb {}

impl ColorRgb {
    pub fn new() -> Self {
        Self {}
    }
}

impl Generator for ColorRgb {
    fn next_value(&mut self, rng: &mut dyn rand::Rng) -> Value {
        Value::ColorRGB(
            rng.random_range(0..=255),
            rng.random_range(0..=255),
            rng.random_range(0..=255),
        )
    }
}
