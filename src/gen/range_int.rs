use rand::RngExt;

use crate::value::{Generator, Value};

pub struct RangeInt {
    min: i64,
    max: i64,
}

impl RangeInt {
    pub fn new(min: i64, max: i64) -> Self {
        assert!(min <= max, "min must be <= max");
        Self { min, max }
    }
}

impl Generator for RangeInt {
    fn next_value(&mut self, rng: &mut dyn rand::prelude::Rng) -> Value {
        Value::Int(rng.random_range(self.min..=self.max))
    }
}

#[cfg(test)]
mod tests {
    use rand::{SeedableRng, rngs::StdRng};

    use super::*;

    #[test]
    fn test() {
        let mut rng = StdRng::seed_from_u64(45);
        let min = 5;
        let max = 15;
        let mut generator = RangeInt::new(min, max);

        for _ in 0..10000 {
            match generator.next_value(&mut rng) {
                Value::Int(v) => assert!(
                    (min..=max).contains(&v),
                    "Value {v} out of range [{min}, {max}]"
                ),
                _ => panic!("Unexpected type"),
            }
        }
    }
}
