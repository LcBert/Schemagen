//! Sequential floating-point generator.

use crate::{
    utils::round_float,
    value::{Generator, Value},
};

/// Generator producing deterministic floating-point sequences with a fixed step.
///
/// This generator creates sequential floating-point values by starting at a specified value
/// and incrementing by a fixed step on each generation. Optionally, you can specify the
/// number of decimal places for rounding. Like the integer sequential generator, it is
/// deterministic and ignores the random number generator, making it ideal for version
/// numbers, coordinates, or any ordered floating-point sequence where predictability is required.
///
/// # Common Use Cases
///
/// - **Version numbers**: Semantic versioning (1.0, 1.1, 1.2, ...)
/// - **Coordinates**: Grid coordinates or lat/lon sequences
/// - **Percentages**: Incremental percentage values
/// - **Time intervals**: Sequential time measurements
/// - **Test data**: Predictable float sequences for testing
///
/// # Examples
///
/// ## Using with Field (recommended)
///
/// ```no_run
/// use schemagen::{field::Field, schema::Schema};
///
/// let mut schema = Schema::new()
///     .add_field("version", Field::sequential_float(1.0, 0.1, Some(1)))     // 1.0, 1.1, 1.2, ...
///     .add_field("coordinate", Field::sequential_float(0.0, 0.5, Some(2))); // 0.00, 0.50, 1.00, ...
///
/// // Generate 10 rows
/// let rows = schema.generate_batch(10);
/// ```
///
/// ## Using the generator directly
///
/// ```no_run
/// use schemagen::r#generators::sequential_float::SequentialFloat;
/// use schemagen::value::Generator;
/// use rand::SeedableRng;
/// use rand::rngs::StdRng;
///
/// let mut rng = StdRng::seed_from_u64(42);
/// let mut generator = SequentialFloat::new(1.0, 0.5, Some(2));
///
/// for _ in 0..5 {
///     let value = generator.next_value(&mut rng);
///     println!("{:?}", value); // 1.00, 1.50, 2.00, 2.50, 3.00
/// }
/// ```
///
/// ## Descending sequences
///
/// ```no_run
/// use schemagen::r#generators::sequential_float::SequentialFloat;
///
/// // Countdown: 10.0, 9.5, 9.0, 8.5, ...
/// let generator = SequentialFloat::new(10.0, -0.5, Some(1));
/// ```
///
/// # Deterministic Behavior
///
/// This generator is **deterministic** and ignores the random number generator:
/// - Each call produces the next value in the sequence
/// - The sequence is entirely predictable
/// - Useful for reproducible test data
///
/// # Decimal Places
///
/// The `decimals` parameter controls rounding:
/// - `Some(2)`: Rounds to 2 decimal places (e.g., 1.50)
/// - `Some(1)`: Rounds to 1 decimal place (e.g., 1.5)
/// - `None`: No rounding applied - maximum precision
pub struct SequentialFloat {
    current: f64,
    step: f64,
    decimals: Option<u32>,
}

impl SequentialFloat {
    /// Creates a new [`SequentialFloat`] generator starting at `current` with increment `step`.
    ///
    /// # Arguments
    ///
    /// * `current` - The starting value of the sequence.
    /// * `step` - The increment to apply on each generation:
    ///   - Positive values create ascending sequences
    ///   - Negative values create descending sequences
    ///   - Zero creates a constant sequence (always returns the same value)
    /// * `decimals` - Optional number of decimal places to round to:
    ///   - `Some(n)`: Rounds to n decimal places
    ///   - `None`: No rounding (maximum precision)
    ///
    /// # Returns
    ///
    /// A new [`SequentialFloat`] instance that generates values starting at `current`.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use schemagen::r#generators::sequential_float::SequentialFloat;
    ///
    /// // Version numbers: 1.0, 1.1, 1.2, ...
    /// let generator = SequentialFloat::new(1.0, 0.1, Some(1));
    ///
    /// // Coordinates: 0.00, 0.50, 1.00, 1.50, ...
    /// let generator = SequentialFloat::new(0.0, 0.5, Some(2));
    ///
    /// // Descending: 10.0, 9.5, 9.0, 8.5, ...
    /// let generator = SequentialFloat::new(10.0, -0.5, Some(1));
    ///
    /// // Unrounded: 1.0, 1.5, 2.0, 2.5, ...
    /// let generator = SequentialFloat::new(1.0, 0.5, None);
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
        Value::Float(round_float(raw_val, self.decimals))
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
