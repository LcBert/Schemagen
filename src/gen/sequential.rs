//! Sequential integer generator.

use crate::value::{Generator, Value};

/// Generator producing integer sequences advancing by a fixed step on each generation.
pub struct Sequential {
    current: i64,
    step: i64,
}

impl Sequential {
    /// Creates a new [`Sequential`] generator starting at `current` with increment `step`.
    pub fn new(current: i64, step: i64) -> Self {
        Self { current, step }
    }
}


impl Generator for Sequential {
    fn next_value(&mut self, _rng: &mut dyn rand::prelude::Rng) -> Value {
        let val = self.current;
        self.current += self.step;
        Value::Int(val)
    }
}

#[cfg(test)]
mod tests {
    use rand::{SeedableRng, rngs::StdRng};

    use super::*;

    #[test]
    fn test_sequential_positive_step() {
        let mut rng = StdRng::seed_from_u64(45);
        let mut generator = Sequential::new(1, 1);

        for i in 1..1000 {
            match generator.next_value(&mut rng) {
                Value::Int(v) => {
                    assert_eq!(v, i, "sequence goes out | expected: {i}, current: {v}")
                }
                _ => panic!("Unexpected type"),
            }
        }
    }

    #[test]
    fn test_sequential_negative_step() {
        let mut rng = StdRng::seed_from_u64(45);
        let mut generator = Sequential::new(100, -5);

        assert_eq!(generator.next_value(&mut rng), Value::Int(100));
        assert_eq!(generator.next_value(&mut rng), Value::Int(95));
        assert_eq!(generator.next_value(&mut rng), Value::Int(90));
    }
}

