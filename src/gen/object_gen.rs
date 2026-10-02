//! Object generator producing structured JSON-like objects.

use crate::value::{Generator, Value};
use indexmap::IndexMap;

/// Generator producing structured objects with named fields.
///
/// Each call generates a [`Value::Object`] containing field names mapped to values from
/// their respective generators. Field order is preserved using `IndexMap`. This is useful
/// for generating JSON documents, nested structures, or any data requiring named fields.
///
/// # Example
///
/// ```no_run
/// use schemagen::field::Field;
///
/// let object = Field::object()
///     .add_field("id", Field::range_int(1, 100))
///     .add_field("status", Field::choice(vec!["active", "inactive"]));
/// ```
#[derive(Default)]
pub struct ObjectGen {
    fields: Vec<(String, Box<dyn Generator>)>,
}

impl ObjectGen {
    /// Creates a new empty [`ObjectGen`] generator.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use schemagen::field::Field;
    ///
    /// let object = Field::object();
    /// ```
    pub fn new() -> Self {
        Self { fields: Vec::new() }
    }

    /// Adds a named field to the object generator.
    ///
    /// # Type Parameters
    ///
    /// * `G` - The generator type for this field, must implement [`Generator`] and have a `'static` lifetime.
    ///
    /// # Arguments
    ///
    /// * `name` - The field name (will be used as the key in the generated object).
    /// * `generator` - The generator to produce values for this field.
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
    /// let object = Field::object()
    ///     .add_field("id", Field::range_int(1, 1000))
    ///     .add_field("status", Field::choice(vec!["active".to_string(), "inactive".to_string()]));
    /// ```
    pub fn add_field<G: Generator + 'static>(
        mut self,
        name: impl Into<String>,
        generator: G,
    ) -> Self {
        self.fields.push((name.into(), Box::new(generator)));
        self
    }
}

impl Generator for ObjectGen {
    fn next_value(&mut self, rng: &mut dyn rand::Rng) -> Value {
        let mut map = IndexMap::with_capacity(self.fields.len());
        for (name, generator) in &mut self.fields {
            map.insert(name.clone(), generator.next_value(rng));
        }
        Value::Object(map)
    }
}
