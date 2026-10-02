//! Optional null wrapper generator.

use rand::RngExt;

use crate::value::{Generator, Value};

/// Generator wrapper that produces `NULL` values with a configurable probability.
///
/// This generator wraps any other generator and occasionally returns [`Value::None`]
/// instead of the generated value, based on a specified probability. This is useful
/// for simulating optional fields, sparse data, or any scenario where NULL values
/// should appear with a certain frequency.
///
/// # Common Use Cases
///
/// - **Optional fields**: Phone numbers, middle names, optional metadata
/// - **Sparse data**: Fields that are frequently empty (e.g., discount codes)
/// - **Testing NULL handling**: Ensuring your application handles NULL values correctly
/// - **Realistic data**: Most real-world databases have some NULL values
///
/// # Examples
///
/// ## Using with Field (recommended)
///
/// ```no_run
/// use schemagen::{field::Field, schema::Schema};
///
/// let mut schema = Schema::new()
///     .add_field("id", Field::range_int(1, 100))
///     .add_field("phone", Field::optional_value(
///         Field::pattern("+39 3## ### ####").add_entry('#', vec!['0'..='9']),
///         0.7  // 70% chance of a phone number, 30% NULL
///     ))
///     .add_field("discount_code", Field::optional_value(
///         Field::choice(vec!["SAVE10", "SAVE20", "SAVE30"]),
///         0.2  // 20% chance of discount, 80% NULL
///     ));
///
/// // Generate 10 rows
/// let rows = schema.generate_batch(10);
/// ```
///
/// ## Using the generator directly
///
/// ```no_run
/// use schemagen::r#generators::optional_value::OptionalValue;
/// use schemagen::r#generators::range_int::RangeInt;
/// use schemagen::value::Generator;
/// use rand::SeedableRng;
/// use rand::rngs::StdRng;
///
/// let mut rng = StdRng::seed_from_u64(42);
/// let inner = RangeInt::new(1, 100);
/// let mut generator = OptionalValue::new(inner, 0.8); // 80% value, 20% NULL
///
/// for _ in 0..10 {
///     let value = generator.next_value(&mut rng);
///     println!("{:?}", value); // Mostly integers, occasionally None
/// }
/// ```
///
/// # Type Parameters
///
/// * `G` - The inner generator type, must implement [`Generator`] and have a `'static` lifetime.
///
/// # Probability
///
/// The `chance` parameter controls the probability of generating a value (not NULL):
/// - `1.0`: Always generates a value (never NULL)
/// - `0.8`: 80% value, 20% NULL
/// - `0.5`: 50% value, 50% NULL
/// - `0.2`: 20% value, 80% NULL
/// - `0.0`: Always NULL (never generates a value)
///
/// # Panics
///
/// Panics if `chance` is not in the range `[0.0, 1.0]`.
pub struct OptionalValue<G: Generator + 'static> {
    generator: G,
    chance: f32,
}

impl<G: 'static + Generator> OptionalValue<G> {
    /// Creates a new [`OptionalValue`] wrapper.
    ///
    /// # Type Parameters
    ///
    /// * `G` - The inner generator type to wrap.
    ///
    /// # Arguments
    ///
    /// * `generator` - The inner generator to wrap.
    /// * `chance` - Probability (0.0 to 1.0) of returning the generated value instead of `NULL`.
    ///   For example, `0.9` means 90% chance of generating a value, 10% chance of `NULL`.
    ///
    /// # Returns
    ///
    /// A new [`OptionalValue`] instance wrapping the provided generator.
    ///
    /// # Panics
    ///
    /// Panics if `chance` is not in the range `[0.0, 1.0]`.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use schemagen::r#generators::optional_value::OptionalValue;
    /// use schemagen::r#generators::range_int::RangeInt;
    ///
    /// // 80% chance of value, 20% chance of NULL
    /// let generator = OptionalValue::new(RangeInt::new(1, 100), 0.8);
    ///
    /// // Always generates a value (never NULL)
    /// let generator = OptionalValue::new(RangeInt::new(1, 100), 1.0);
    ///
    /// // Always NULL (never generates a value)
    /// let generator = OptionalValue::new(RangeInt::new(1, 100), 0.0);
    /// ```
    pub fn new(generator: G, chance: f32) -> Self {
        assert!(
            (0.0 <= chance && chance <= 1.0),
            "chance not valid (0.0 <= chance <= 1.0)"
        );
        Self { generator, chance }
    }
}

impl<G: 'static + Generator> Generator for OptionalValue<G> {
    fn next_value(&mut self, rng: &mut dyn rand::prelude::Rng) -> Value {
        if rng.random_range(0.0..=1.0) <= self.chance {
            self.generator.next_value(rng)
        } else {
            Value::None
        }
    }
}

#[cfg(test)]
mod tests {
    use rand::{SeedableRng, rngs::StdRng};

    use super::*;
    use crate::r#generators::range_int::RangeInt;

    #[test]
    fn test_optional_value_always_value() {
        let mut rng = StdRng::seed_from_u64(42);
        let inner = RangeInt::new(1, 100);
        let mut generator = OptionalValue::new(inner, 1.0);

        for _ in 0..100 {
            match generator.next_value(&mut rng) {
                Value::Int(_) => {}
                Value::None => panic!("Should never return None with chance=1.0"),
                _ => panic!("Unexpected type"),
            }
        }
    }

    #[test]
    fn test_optional_value_always_none() {
        let mut rng = StdRng::seed_from_u64(42);
        let inner = RangeInt::new(1, 100);
        let mut generator = OptionalValue::new(inner, 0.0);

        for _ in 0..100 {
            match generator.next_value(&mut rng) {
                Value::None => {}
                _ => panic!("Should always return None with chance=0.0"),
            }
        }
    }

    #[test]
    fn test_optional_value_mixed() {
        let mut rng = StdRng::seed_from_u64(42);
        let inner = RangeInt::new(1, 100);
        let mut generator = OptionalValue::new(inner, 0.5);

        let mut value_count = 0;
        let mut none_count = 0;

        for _ in 0..1000 {
            match generator.next_value(&mut rng) {
                Value::Int(_) => value_count += 1,
                Value::None => none_count += 1,
                _ => panic!("Unexpected type"),
            }
        }

        // With 50% chance and 1000 iterations, both should be well-represented
        assert!(value_count > 400, "Too few values: {value_count}");
        assert!(none_count > 400, "Too few None values: {none_count}");
    }

    #[test]
    #[should_panic(expected = "chance not valid")]
    fn test_invalid_chance_panics() {
        let inner = RangeInt::new(1, 100);
        OptionalValue::new(inner, 1.5);
    }
}
