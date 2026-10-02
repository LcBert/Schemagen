//! Boolean check generator.

use rand::RngExt;

use crate::value::{Generator, Value};

/// Generator producing random boolean values with uniform distribution.
///
/// This generator creates boolean values where `true` and `false` have equal probability (50/50).
/// It is ideal for generating binary flags, checkboxes, status indicators, or any field that
/// requires a simple true/false state.
///
/// # Common Use Cases
///
/// - **Active/Inactive flags**: Mark records as active or inactive
/// - **Permission checks**: Generate random permission states
/// - **Boolean toggles**: Feature flags, settings, or preferences
/// - **Validation states**: Success/failure indicators
///
/// # Examples
///
/// ## Using with Field (recommended)
///
/// ```no_run
/// use schemagen::{field::Field, schema::Schema};
///
/// let mut schema = Schema::new()
///     .add_field("is_active", Field::check())
///     .add_field("has_permission", Field::check())
///     .add_field("is_verified", Field::check());
///
/// // Generate 10 rows
/// let rows = schema.generate_batch(10);
/// ```
///
/// ## Using the generator directly
///
/// ```no_run
/// use schemagen::r#generators::check::Check;
/// use schemagen::value::Generator;
/// use rand::SeedableRng;
/// use rand::rngs::StdRng;
///
/// let mut rng = StdRng::seed_from_u64(42);
/// let mut generator = Check::new();
///
/// for _ in 0..5 {
///     let value = generator.next_value(&mut rng);
///     println!("{:?}", value); // Either true or false
/// }
/// ```
///
/// # Distribution
///
/// The generator uses a uniform distribution, meaning over a large number of generations:
/// - Approximately 50% will be `true`
/// - Approximately 50% will be `false`
///
/// For weighted distributions (e.g., 80% true, 20% false), consider using [`Choice`](crate::generators::choice::Choice)
/// with weighted options or a custom generator.
#[derive(Default)]
pub struct Check {}

impl Check {
    /// Creates a new [`Check`] generator.
    ///
    /// # Returns
    ///
    /// A new [`Check`] instance configured for 50/50 boolean distribution.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use schemagen::r#generators::check::Check;
    /// use schemagen::value::Generator;
    ///
    /// let generator = Check::new();
    /// ```
    pub fn new() -> Self {
        Self {}
    }
}


impl Generator for Check {
    fn next_value(&mut self, rng: &mut dyn rand::prelude::Rng) -> Value {
        Value::Boolean(rng.random_ratio(1, 2))
    }
}

#[cfg(test)]
mod tests {
    use rand::{SeedableRng, rngs::StdRng};

    use super::*;

    #[test]
    fn test_check_distribution() {
        let mut rng = StdRng::seed_from_u64(45);
        let mut generator = Check::new();

        let mut true_count = 0;
        let mut false_count = 0;

        for _ in 0..10000 {
            match generator.next_value(&mut rng) {
                Value::Boolean(true) => true_count += 1,
                Value::Boolean(false) => false_count += 1,
                _ => panic!("Unexpected type"),
            }
        }

        // With 10,000 iterations and 50/50 probability, both should be well-represented
        assert!(true_count > 4000, "Too few true values: {true_count}");
        assert!(false_count > 4000, "Too few false values: {false_count}");
    }
}

