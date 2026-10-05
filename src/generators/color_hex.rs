use crate::{
    generators::pattern::Pattern,
    value::{Generator, Value},
};

pub struct ColorHex {}

impl ColorHex {
    pub fn new() -> Self {
        Self {}
    }
}

impl Generator for ColorHex {
    fn next_value(&mut self, rng: &mut dyn rand::Rng) -> Value {
        Pattern::new("#??????")
            .add_entry('?', vec!['0'..='9', 'A'..='F'])
            .next_value(rng)
    }
}
