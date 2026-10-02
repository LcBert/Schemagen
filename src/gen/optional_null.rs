use rand::RngExt;

use crate::value::{Generator, Value};

pub struct OptionalNull<G: Generator + 'static> {
    generator: G,
    chance: f32,
}

impl<G: 'static + Generator> OptionalNull<G> {
    pub fn new(generator: G, chance: f32) -> Self {
        assert!(
            (0.0 <= chance && chance <= 1.0),
            "chance not valid (0.0 <= chance <= 1.0)"
        );
        Self { generator, chance }
    }
}

impl<G: 'static + Generator> Generator for OptionalNull<G> {
    fn next_value(&mut self, rng: &mut dyn rand::prelude::Rng) -> Value {
        if rng.random_range(0.0..=1.0) <= self.chance {
            self.generator.next_value(rng)
        } else {
            Value::None
        }
    }
}
