//! Random discrete choice generator.

use rand::seq::IndexedRandom;

use crate::value::{Generator, Value};

/// Generator selecting a random string from a list of predefined options.
///
/// This generator randomly selects one value from a provided list of options with uniform probability.
/// Each option has an equal chance of being selected on each generation. It is particularly useful
/// for generating categorical data such as status values, product categories, priorities, or any
/// enumerated type represented as strings.
///
/// # Common Use Cases
///
/// - **Status fields**: Active, Inactive, Pending, Completed
/// - **Categories**: Electronics, Clothing, Books, Home
/// - **Priorities**: Low, Medium, High, Critical
/// - **Roles**: Admin, User, Guest, Moderator
/// - **Countries/Regions**: IT, US, UK, DE, FR
///
/// # Examples
///
/// ## Using with Field (recommended)
///
/// ```no_run
/// use schemagen::{field::Field, schema::Schema};
///
/// let mut schema = Schema::new()
///     .add_field("status", Field::choice(vec!["Active", "Inactive", "Pending"]))
///     .add_field("category", Field::choice(vec!["Electronics", "Clothing", "Books"]))
///     .add_field("priority", Field::choice(vec!["Low", "Medium", "High", "Critical"]));
///
/// // Generate 10 rows
/// let rows = schema.generate_batch(10);
/// ```
///
/// ## Using the generator directly
///
/// ```no_run
/// use schemagen::r#generators::choice::Choice;
/// use schemagen::value::Generator;
/// use rand::SeedableRng;
/// use rand::rngs::StdRng;
///
/// let mut rng = StdRng::seed_from_u64(42);
/// let mut generator = Choice::new(vec![
///     "Option A".to_string(),
///     "Option B".to_string(),
///     "Option C".to_string(),
/// ]);
///
/// for _ in 0..5 {
///     let value = generator.next_value(&mut rng);
///     println!("{:?}", value); // Randomly one of the three options
/// }
/// ```
///
/// # Distribution
///
/// The generator uses uniform distribution, meaning each option has an equal probability:
/// - With 3 options: each has ~33.3% chance
/// - With 4 options: each has 25% chance
/// - With N options: each has 1/N chance
///
/// For weighted distributions (e.g., 80% Active, 20% Inactive), consider using a custom generator
/// or repeating options to simulate weights.
pub struct Choice {
    options: Vec<String>,
}

impl Choice {
    /// Creates a new [`Choice`] generator with the provided options.
    ///
    /// # Arguments
    ///
    /// * `options` - A vector of string options to choose from. Must not be empty.
    ///
    /// # Returns
    ///
    /// A new [`Choice`] instance that will randomly select from the provided options.
    ///
    /// # Panics
    ///
    /// Panics if `options` is empty.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use schemagen::r#generators::choice::Choice;
    ///
    /// let generator = Choice::new(vec![
    ///     "Option A".to_string(),
    ///     "Option B".to_string(),
    ///     "Option C".to_string(),
    /// ]);
    /// ```
    pub fn new(options: Vec<String>) -> Self {
        assert!(!options.is_empty(), "Options cannot be empty");
        Self { options }
    }
}


impl Generator for Choice {
    fn next_value(&mut self, rng: &mut dyn rand::prelude::Rng) -> Value {
        Value::Text(self.options.choose(rng).unwrap().clone())
    }
}

#[cfg(test)]
mod tests {
    use rand::{SeedableRng, rngs::StdRng};

    use super::*;

    #[test]
    fn test_choice() {
        let mut rng = StdRng::seed_from_u64(45);
        let options: Vec<String> = vec![
            "test1".to_string(),
            "test2".to_string(),
            "test3".to_string(),
        ];
        let mut generator = Choice::new(options.clone());

        for _ in 0..10000 {
            match generator.next_value(&mut rng) {
                Value::Text(v) => assert!(options.contains(&v), "Value {v} not in options"),
                _ => panic!("Unexpected type"),
            }
        }
    }

    #[test]
    fn test_single_option() {
        let mut rng = StdRng::seed_from_u64(45);
        let mut generator = Choice::new(vec!["only_one".to_string()]);

        for _ in 0..100 {
            assert_eq!(
                generator.next_value(&mut rng),
                Value::Text("only_one".to_string())
            );
        }
    }

    #[test]
    #[should_panic(expected = "Options cannot be empty")]
    fn test_empty_options_panics() {
        Choice::new(Vec::new());
    }
}

