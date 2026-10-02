//! Sequential integer generator.

use crate::value::{Generator, Value};

/// Generator producing deterministic integer sequences with a fixed step.
///
/// This generator creates sequential numeric values by starting at a specified value and
/// incrementing by a fixed step on each generation. It is deterministic and ignores the
/// random number generator, making it ideal for creating sequential IDs, counters, batch
/// numbers, or any ordered numeric sequence where predictability is required.
///
/// # Common Use Cases
///
/// - **Primary keys**: Sequential IDs (1, 2, 3, 4, ...)
/// - **Batch numbers**: Grouping records by batch (1000, 1010, 1020, ...)
/// - **Version numbers**: Incrementing version identifiers
/// - **Order numbers**: Sequential order or invoice numbers
/// - **Test data**: Predictable sequences for testing
///
/// # Examples
///
/// ## Using with Field (recommended)
///
/// ```no_run
/// use schemagen::{field::Field, schema::Schema};
///
/// let mut schema = Schema::new()
///     .add_field("id", Field::sequential(1, 1))           // 1, 2, 3, 4, ...
///     .add_field("batch_id", Field::sequential(1000, 10)); // 1000, 1010, 1020, ...
///
/// // Generate 10 rows
/// let rows = schema.generate_batch(10);
/// ```
///
/// ## Using the generator directly
///
/// ```no_run
/// use schemagen::r#generators::sequential::Sequential;
/// use schemagen::value::Generator;
/// use rand::SeedableRng;
/// use rand::rngs::StdRng;
///
/// let mut rng = StdRng::seed_from_u64(42);
/// let mut generator = Sequential::new(1, 1);
///
/// for _ in 0..5 {
///     let value = generator.next_value(&mut rng);
///     println!("{:?}", value); // 1, 2, 3, 4, 5
/// }
/// ```
///
/// ## Descending sequences
///
/// ```no_run
/// use schemagen::r#generators::sequential::Sequential;
///
/// // Countdown: 100, 95, 90, 85, ...
/// let generator = Sequential::new(100, -5);
/// ```
///
/// # Deterministic Behavior
///
/// This generator is **deterministic** and ignores the random number generator:
/// - Each call produces the next value in the sequence
/// - The sequence is entirely predictable
/// - Useful for reproducible test data
///
/// # Sequences
///
/// Common sequence patterns:
/// - `Sequential::new(1, 1)`: 1, 2, 3, 4, 5, ... (standard increment)
/// - `Sequential::new(0, 10)`: 0, 10, 20, 30, 40, ... (by tens)
/// - `Sequential::new(1000, 100)`: 1000, 1100, 1200, 1300, ... (by hundreds)
/// - `Sequential::new(100, -5)`: 100, 95, 90, 85, 80, ... (descending)
pub struct Sequential {
    current: i64,
    step: i64,
}

impl Sequential {
    /// Creates a new [`Sequential`] generator starting at `current` with increment `step`.
    ///
    /// # Arguments
    ///
    /// * `current` - The starting value of the sequence.
    /// * `step` - The increment to apply on each generation:
    ///   - Positive values create ascending sequences
    ///   - Negative values create descending sequences
    ///   - Zero creates a constant sequence (always returns the same value)
    ///
    /// # Returns
    ///
    /// A new [`Sequential`] instance that generates values starting at `current`.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use schemagen::r#generators::sequential::Sequential;
    ///
    /// // Ascending sequence: 1, 2, 3, 4, ...
    /// let generator = Sequential::new(1, 1);
    ///
    /// // Descending sequence: 100, 95, 90, 85, ...
    /// let generator = Sequential::new(100, -5);
    ///
    /// // By tens: 0, 10, 20, 30, ...
    /// let generator = Sequential::new(0, 10);
    ///
    /// // Constant value: always returns 42
    /// let generator = Sequential::new(42, 0);
    /// ```
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

