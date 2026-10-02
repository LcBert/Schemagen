//! Sequential floating-point generator.

use crate::value::{Generator, Value};

/// Generator producing floating-point sequences advancing by a fixed step on each generation,
/// optionally rounded to a specific number of decimal places.
///
/// This generator is deterministic and ignores the random number generator, making it useful
/// for creating sequential version numbers, coordinates, or ordered floating-point sequences.
///
/// # Example
///
/// ```no_run
/// use schemagen::{field::Field, schema::Schema};
///
/// let mut schema = Schema::new()
///     .add_field("version", Field::sequential_float(1.0, 0.1, Some(1)))     // 1.0, 1.1, 1.2, ...
///     .add_field("coordinate", Field::sequential_float(0.0, 0.5, Some(2))); // 0.00, 0.50, 1.00, ...
/// ```
pub struct SequentialFloat {
    current: f64,
    step: f64,
    decimals: Option<u32>,
}

impl SequentialFloat {
    /// Creates a new [`SequentialFloat`] generator starting at `current` with increment `step`,
    /// optionally rounding to `decimals` decimal places.
    ///
    /// # Arguments
    ///
    /// * `current` - The starting value.
    /// * `step` - The increment to apply on each generation. Can be negative for descending sequences.
    /// * `decimals` - Optional number of decimal places to round to. If `None`, no rounding is applied.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use schemagen::r#gen::sequential_float::SequentialFloat;
    ///
    /// // Ascending sequence with 2 decimal places: 1.00, 1.50, 2.00, ...
    /// let generator = SequentialFloat::new(1.0, 0.5, Some(2));
    ///
    /// // Descending sequence with 1 decimal place: 10.0, 9.5, 9.0, ...
    /// let generator = SequentialFloat::new(10.0, -0.5, Some(1));
    /// ```
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
    fn test_sequential_float_with_decimals() {
        let mut rng = StdRng::seed_from_u64(45);
        let mut generator = SequentialFloat::new(1.0, 0.5, Some(2));

        for step in 0..1000 {
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

    #[test]
    fn test_sequential_float_negative_step() {
        let mut rng = StdRng::seed_from_u64(45);
        let mut generator = SequentialFloat::new(10.0, -0.5, Some(1));

        assert_eq!(generator.next_value(&mut rng), Value::Float(10.0));
        assert_eq!(generator.next_value(&mut rng), Value::Float(9.5));
        assert_eq!(generator.next_value(&mut rng), Value::Float(9.0));
    }
}

