//! Integer range generator.

use rand::RngExt;

use crate::value::{Generator, Value};

/// Generator producing uniform random integers in the range `[min, max]`.
///
/// Each call generates a random integer where both endpoints are inclusive.
/// Useful for generating IDs, ages, quantities, or any discrete numeric values.
///
/// # Example
///
/// ```no_run
/// use schemagen::{field::Field, schema::Schema};
///
/// let mut schema = Schema::new()
///     .add_field("age", Field::range_int(18, 100))
///     .add_field("quantity", Field::range_int(1, 1000));
/// ```
pub struct RangeInt {
    min: i64,
    max: i64,
}

impl RangeInt {
    /// Creates a new [`RangeInt`] generator.
    ///
    /// # Arguments
    ///
    /// * `min` - Minimum value (inclusive).
    /// * `max` - Maximum value (inclusive).
    ///
    /// # Panics
    ///
    /// Panics if `min > max`.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use schemagen::r#gen::range_int::RangeInt;
    ///
    /// let generator = RangeInt::new(1, 100);
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

