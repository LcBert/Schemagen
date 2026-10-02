//! Array value generator combining multiple generators.

use crate::value::{Generator, Value};

/// Generator producing array values by combining multiple generators.
///
/// Each call generates a [`Value::Array`] containing values from all registered generators
/// in the order they were added. This is useful for generating multi-column data or composite
/// structures where multiple values need to be generated together.
///
/// # Example
///
/// ```no_run
/// use schemagen::field::Field;
///
/// let array = Field::array()
///     .add_generator(Field::range_int(1, 100))
///     .add_generator(Field::choice(vec!["A".to_string(), "B".to_string()]));
/// ```
pub struct Array {
    generators: Vec<Box<dyn Generator>>,
}

impl Array {
    /// Creates a new empty [`Array`] generator.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use schemagen::field::Field;
    ///
    /// let array = Field::array();
    /// ```
    pub fn new() -> Self {
        Self {
            generators: Vec::new(),
        }
    }

    /// Adds a generator to the array.
    ///
    /// # Type Parameters
    ///
    /// * `G` - The generator type to add, must implement [`Generator`] and have a `'static` lifetime.
    ///
    /// # Arguments
    ///
    /// * `generator` - The generator to add to the array.
    ///
    /// # Returns
    ///
    /// Returns `self` to allow method chaining.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use schemagen::field::Field;
    ///
    /// let array = Field::array()
    ///     .add_generator(Field::range_int(1, 10))
    ///     .add_generator(Field::range_int(100, 200));
    /// ```
    pub fn add_generator<G: Generator + 'static>(mut self, generator: G) -> Self {
        self.generators.push(Box::new(generator));
        self
    }
}

impl Generator for Array {
    fn next_value(&mut self, rng: &mut dyn rand::prelude::Rng) -> Value {
        let mut values: Vec<Value> = Vec::with_capacity(self.generators.len());

        for generator in &mut self.generators {
            values.push(generator.next_value(rng))
        }

        Value::Array(values)
    }
}
