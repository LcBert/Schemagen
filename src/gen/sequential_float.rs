//! Sequential floating-point generator.

use crate::value::{Generator, Value};

/// Generator producing floating-point sequences advancing by a fixed step on each generation,
/// optionally rounded to a specific number of decimal places.
pub struct SequentialFloat {
    current: f64,
    step: f64,
    decimals: Option<u32>,
}

impl SequentialFloat {
    /// Creates a new [`SequentialFloat`] generator starting at `current` with increment `step`,
    /// optionally rounding to `decimals` decimal places.
    pub fn new(current: f64, step: f64, decimals: Option<u32>) -> Self {
        Self {
            current,
            step,
            decimals,
        }
    }
}


impl Generator for SequentialFloat {
    fn next_value(&mut self, _rng: &mut dyn rand::prelude::Rng) -> Value {
        let raw_val = self.current;
        self.current += self.step;

        let final_val = match self.decimals {
            Some(places)=>{
                let factor =10_f64.powi(places as i32);
                (raw_val*factor).round()/factor
            }
            None=>raw_val,
        };
        Value::Float(final_val)
    }
}

#[cfg(test)]
mod tests {
    use rand::{SeedableRng, rngs::StdRng};

    use super::*;

    #[test]
    fn test() {
        let mut rng = StdRng::seed_from_u64(45);
        let mut generator = SequentialFloat::new(1.0, 0.5, Some(2));

        for step in 0..9998 {
            let expected = 1.0 + (step as f64) * 0.5;
            match generator.next_value(&mut rng) {
                Value::Float(v) => assert_eq!(
                    v, expected,
                    "sequence goes out | expected: {expected}, current: {v}"
                ),
                _ => panic!("Unexpected type"),
            }
        }
    }
}
