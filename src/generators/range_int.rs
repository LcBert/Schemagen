//! Integer range generator.

use rand::RngExt;

use crate::value::{Generator, Value};

/// Generator producing uniform random integers within a specified range.
///
/// This generator creates random integers where both the minimum and maximum values are inclusive.
/// Each generation produces a value uniformly distributed across the entire range. It is ideal for
/// generating IDs, ages, quantities, scores, or any discrete numeric data that falls within bounds.
///
/// # Common Use Cases
///
/// - **Age ranges**: Generate ages between 18 and 100
/// - **Quantities**: Order quantities between 1 and 1000
/// - **Scores**: Test scores between 0 and 100
/// - **IDs**: Random IDs within a specific range
/// - **Ratings**: Star ratings between 1 and 5
/// - **Counts**: Item counts, view counts, etc.
///
/// # Examples
///
/// ## Using with Field (recommended)
///
/// ```no_run
/// use schemagen::{field::Field, schema::Schema};
///
/// let mut schema = Schema::new()
///     .add_field("age", Field::range_int(18, 100))
///     .add_field("quantity", Field::range_int(1, 1000))
///     .add_field("rating", Field::range_int(1, 5));
///
/// // Generate 10 rows
/// let rows = schema.generate_batch(10);
/// ```
///
/// ## Using the generator directly
///
/// ```no_run
/// use schemagen::r#generators::range_int::RangeInt;
/// use schemagen::value::Generator;
/// use rand::SeedableRng;
/// use rand::rngs::StdRng;
///
/// let mut rng = StdRng::seed_from_u64(42);
/// let mut generator = RangeInt::new(1, 100);
///
/// for _ in 0..5 {
///     let value = generator.next_value(&mut rng);
///     println!("{:?}", value); // Random integer between 1 and 100 (inclusive)
/// }
/// ```
///
/// ## Negative ranges
///
/// ```no_run
/// use schemagen::r#generators::range_int::RangeInt;
///
/// // Generate temperatures between -50 and 50
/// let generator = RangeInt::new(-50, 50);
/// ```
///
/// # Distribution
///
/// The generator uses uniform distribution across the entire range `[min, max]`:
/// - Each integer in the range has equal probability
/// - Both endpoints are inclusive
/// - Works with negative numbers
///
/// # Panics
///
/// Panics if `min > max`.
pub struct RangeInt {
    min: i64,
    max: i64,
}

impl RangeInt {
    /// Creates a new [`RangeInt`] generator.
    ///
    /// # Arguments
    ///
    /// * `min` - Minimum value (inclusive). Can be negative.
    /// * `max` - Maximum value (inclusive). Must be greater than or equal to `min`.
    ///
    /// # Returns
    ///
    /// A new [`RangeInt`] instance that generates integers in the range `[min, max]`.
    ///
    /// # Panics
    ///
    /// Panics if `min > max`.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use schemagen::r#generators::range_int::RangeInt;
    ///
    /// // Generate ages between 18 and 100
    /// let generator = RangeInt::new(18, 100);
    ///
    /// // Generate temperatures between -50 and 50
    /// let generator = RangeInt::new(-50, 50);
    ///
    /// // Generate a single constant value
    /// let generator = RangeInt::new(42, 42);
    /// ```
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
    fn test_range_int() {
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

    #[test]
    fn test_negative_range() {
        let mut rng = StdRng::seed_from_u64(45);
        let min = -50;
        let max = -10;
        let mut generator = RangeInt::new(min, max);

        for _ in 1000..=2000 {
            match generator.next_value(&mut rng) {
                Value::Int(v) => assert!(
                    (min..=max).contains(&v),
                    "Value {v} out of range [{min}, {max}]"
                ),
                _ => panic!("Unexpected type"),
            }
        }
    }

    #[test]
    fn test_single_value() {
        let mut rng = StdRng::seed_from_u64(45);
        let mut generator = RangeInt::new(42, 42);

        for _ in 0..100 {
            assert_eq!(generator.next_value(&mut rng), Value::Int(42));
        }
    }

    #[test]
    #[should_panic(expected = "min must be <= max")]
    fn test_invalid_range_panics() {
        RangeInt::new(10, 5);
    }
}

