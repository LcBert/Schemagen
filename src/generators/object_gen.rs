//! Object generator producing structured JSON-like objects.

use crate::value::{Generator, Value};
use indexmap::IndexMap;

/// Generator producing structured objects with named fields.
///
/// This generator creates JSON-like objects by mapping field names to values produced by
/// their respective generators. Field order is preserved using `IndexMap`, ensuring
/// consistent output ordering. This is useful for generating JSON documents, nested
/// structures, configuration objects, or any data requiring named fields.
///
/// # Common Use Cases
///
/// - **JSON columns**: PostgreSQL JSONB/JSON columns with structured data
/// - **Nested objects**: Complex nested data structures
/// - **Configuration objects**: Settings or metadata stored as JSON
/// - **API responses**: Simulating API response payloads
/// - **Document storage**: NoSQL document databases
///
/// # Examples
///
/// ## Using with Field (recommended)
///
/// ```no_run
/// use schemagen::{field::Field, schema::Schema};
///
/// let mut schema = Schema::new()
///     .add_field("metadata", Field::object()
///         .add_field("id", Field::range_int(1, 1000))
///         .add_field("status", Field::choice(vec!["active", "inactive"]))
///         .add_field("priority", Field::choice(vec!["low", "medium", "high"]))
///         .add_field("tags", Field::array()
///             .add_generator(Field::choice(vec!["tag1", "tag2", "tag3"]))))
///     .add_field("config", Field::object()
///         .add_field("enabled", Field::check())
///         .add_field("max_retries", Field::range_int(1, 5)));
///
/// // Generate 10 rows
/// let rows = schema.generate_batch(10);
/// ```
///
/// ## Using the generator directly
///
/// ```no_run
/// use schemagen::r#generators::object_gen::ObjectGen;
/// use schemagen::r#generators::range_int::RangeInt;
/// use schemagen::r#generators::choice::Choice;
/// use schemagen::value::Generator;
/// use rand::SeedableRng;
/// use rand::rngs::StdRng;
///
/// let mut rng = StdRng::seed_from_u64(42);
/// let mut generator = ObjectGen::new()
///     .add_field("id", RangeInt::new(1, 100))
///     .add_field("status", Choice::new(vec!["active".to_string(), "inactive".to_string()]));
///
/// for _ in 0..5 {
///     let value = generator.next_value(&mut rng);
///     println!("{:?}", value); // e.g., Object({"id": 42, "status": "active"})
/// }
/// ```
///
/// # SQL Serialization
///
/// Objects are serialized as JSON strings for SQL insertion, for example:
/// `'{"id": 42, "status": "active"}'` or `'{"enabled": true, "max_retries": 3}'`
///
/// This format is compatible with PostgreSQL JSONB/JSON columns and similar database types.
#[derive(Default)]
pub struct ObjectGen {
    fields: Vec<(String, Box<dyn Generator>)>,
}

impl ObjectGen {
    /// Creates a new empty [`ObjectGen`] generator.
    ///
    /// # Returns
    ///
    /// A new [`ObjectGen`] instance with no fields. Use [`add_field`](Self::add_field)
    /// to add fields to the object.
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
    ///     .add_field("status", Field::choice(vec!["active", "inactive"]))
    ///     .add_field("enabled", Field::check());
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
