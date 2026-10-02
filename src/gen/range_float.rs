//! Floating-point range generator.

use rand::RngExt;

use crate::value::{Generator, Value};

/// Generator producing uniform random floating-point numbers in the range `[min, max]`,
/// optionally rounded to a specific number of decimal places.
///
/// Each call generates a random float where both endpoints are inclusive.
/// Useful for generating prices, coordinates, percentages, or any continuous numeric values.
///
/// # Example
///
/// ```no_run
/// use schemagen::{field::Field, schema::Schema};
///
/// let mut schema = Schema::new()
///     .add_field("price", Field::range_float(1.0, 100.0, Some(2)))
///     .add_field("rating", Field::range_float(0.0, 5.0, Some(1)));
/// ```
pub struct RangeFloat {
    min: f64,
    max: f64,
    decimals: Option<u32>,
}

impl RangeFloat {
    /// Creates a new [`RangeFloat`] generator.
    ///
    /// # Arguments
    ///
    /// * `min` - Minimum value (inclusive).
    /// * `max` - Maximum value (inclusive).
    /// * `decimals` - Optional number of decimal places to round to. If `None`, no rounding is applied.
    ///
    /// # Panics
    ///
    /// Panics if `min > max`.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use schemagen::r#gen::range_float::RangeFloat;
    ///
    /// // Generate prices with 2 decimal places
    /// let generator = RangeFloat::new(1.0, 100.0, Some(2));
    ///
    /// // Generate unrounded floats
    /// let generator = RangeFloat::new(0.0, 1.0, None);
    /// ```
    pub fn new(min: f64, max: f64, decimals: Option<u32>) -> Self {
        assert!(min <= max, "min must be <= max");
        Self { min, max, decimals }
    }
}

impl Generator for RangeFloat {
    fn next_value(&mut self, rng: &mut dyn rand::prelude::Rng) -> Value {
        let raw_val = rng.random_range(self.min..=self.max);
        let final_val = match self.decimals {
            Some(places) => {
                let factor = 10_f64.powi(places as i32);
                (raw_val * factor).round() / factor
            }
            None => raw_val,
        };
        Value::Float(final_val)
    }
}


#[cfg(test)]
mod tests {
    use rand::{SeedableRng, rngs::StdRng};

    use super::*;

    #[test]
    fn test_range_float() {
        let mut rng = StdRng::seed_from_u64(45);
        let min = 5.0;
        let max = 15.0;
        let decimals = 2;
        let mut generator = RangeFloat::new(min, max, Some(decimals));

        for _ in 0..10000 {
            match generator.next_value(&mut rng) {
                Value::Float(v) => assert!(
                    (min..=max).contains(&v),
                    "Value {v} out of range [{min}, {max}]"
                ),
                _ => panic!("Unexpected type"),
            }
        }
    }

    #[test]
    fn test_without_decimals() {
        let mut rng = StdRng::seed_from_u64(45);
        let min = 1.0;
        let max = 10.0;
        let mut generator = RangeFloat::new(min, max, None);

        for _ in 0..1000 {
            match generator.next_value(&mut rng) {
                Value::Float(v) => assert!((min..=max).contains(&v)),
                _ => panic!("Unexpected type"),
            }
        }
    }

    #[test]
    #[should_panic(expected = "min must be <= max")]
    fn test_invalid_range_panics() {
        RangeFloat::new(10.0, 1.0, None);
    }
}

