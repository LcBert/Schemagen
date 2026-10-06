//! Floating-point range generator.

use rand::RngExt;

use crate::{
    utils::round_float,
    value::{Generator, Value},
};

/// Generator producing uniform random floating-point numbers within a specified range.
///
/// This generator creates random floating-point numbers where both the minimum and maximum values
/// are inclusive. Optionally, you can specify the number of decimal places for rounding to ensure
/// consistent formatting. It is ideal for generating prices, coordinates, percentages, ratings, or
/// any continuous numeric data.
///
/// # Common Use Cases
///
/// - **Prices**: Generate prices with 2 decimal places (e.g., $1.99, $50.00)
/// - **Ratings**: Star ratings with 1 decimal place (e.g., 4.5, 3.8)
/// - **Percentages**: Values between 0.0 and 100.0
/// - **Coordinates**: Latitude/longitude values
/// - **Weights/Measures**: Physical measurements
/// - **Probabilities**: Values between 0.0 and 1.0
///
/// # Examples
///
/// ## Using with Field (recommended)
///
/// ```no_run
/// use schemagen::{field::Field, schema::Schema};
///
/// let mut schema = Schema::new()
///     .add_field("price", Field::range_float(1.0, 100.0, Some(2)))
///     .add_field("rating", Field::range_float(0.0, 5.0, Some(1)))
///     .add_field("discount", Field::range_float(0.0, 1.0, None));
///
/// // Generate 10 rows
/// let rows = schema.generate_batch(10);
/// ```
///
/// ## Using the generator directly
///
/// ```no_run
/// use schemagen::r#generators::range_float::RangeFloat;
/// use schemagen::value::Generator;
/// use rand::SeedableRng;
/// use rand::rngs::StdRng;
///
/// let mut rng = StdRng::seed_from_u64(42);
///
/// // Generate prices with 2 decimal places
/// let mut generator = RangeFloat::new(1.0, 100.0, Some(2));
///
/// for _ in 0..5 {
///     let value = generator.next_value(&mut rng);
///     println!("{:?}", value); // e.g., 42.57, 15.99, etc.
/// }
/// ```
///
/// ## Without rounding
///
/// ```no_run
/// use schemagen::r#generators::range_float::RangeFloat;
///
/// // Generate unrounded floats for maximum precision
/// let generator = RangeFloat::new(0.0, 1.0, None);
/// ```
///
/// # Decimal Places
///
/// The `decimals` parameter controls rounding:
/// - `Some(2)`: Rounds to 2 decimal places (e.g., 42.57) - ideal for currency
/// - `Some(1)`: Rounds to 1 decimal place (e.g., 4.5) - ideal for ratings
/// - `None`: No rounding applied - maximum precision
///
/// # Distribution
///
/// The generator uses uniform distribution across the entire range `[min, max]`:
/// - Each value in the continuous range has equal probability density
/// - Both endpoints are inclusive
/// - Rounding is applied after random selection
///
/// # Panics
///
/// Panics if `min > max`.
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
    /// * `min` - Minimum value (inclusive). Can be negative.
    /// * `max` - Maximum value (inclusive). Must be greater than or equal to `min`.
    /// * `decimals` - Optional number of decimal places to round to:
    ///   - `Some(n)`: Rounds to n decimal places
    ///   - `None`: No rounding (maximum precision)
    ///
    /// # Returns
    ///
    /// A new [`RangeFloat`] instance that generates floats in the range `[min, max]`.
    ///
    /// # Panics
    ///
    /// Panics if `min > max`.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use schemagen::r#generators::range_float::RangeFloat;
    ///
    /// // Generate prices with 2 decimal places (e.g., 42.57)
    /// let generator = RangeFloat::new(1.0, 100.0, Some(2));
    ///
    /// // Generate ratings with 1 decimal place (e.g., 4.5)
    /// let generator = RangeFloat::new(0.0, 5.0, Some(1));
    ///
    /// // Generate unrounded floats for maximum precision
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
        Value::Float(round_float(raw_val, self.decimals))
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
