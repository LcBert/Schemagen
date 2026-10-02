//! Array value generator combining multiple generators.

use crate::value::{Generator, Value};

/// Generator producing array values by combining multiple generators.
///
/// This generator creates arrays by running multiple generators in sequence and collecting
/// their outputs into a single [`Value::Array`]. Each call generates one array containing
/// values from all registered generators in the order they were added. This is useful for
/// generating PostgreSQL ARRAY columns, multi-value fields, or composite structures where
/// multiple values need to be generated together.
///
/// # Common Use Cases
///
/// - **PostgreSQL ARRAY columns**: Storing multiple values in a single column
/// - **Multi-value fields**: Tags, categories, or lists of related items
/// - **Composite data**: Multiple related values generated together
/// - **Array-based storage**: Denormalized data with arrays
///
/// # Examples
///
/// ## Using with Field (recommended)
///
/// ```no_run
/// use schemagen::{field::Field, schema::Schema};
///
/// let mut schema = Schema::new()
///     .add_field("tags", Field::array()
///         .add_generator(Field::choice(vec!["tag1", "tag2", "tag3"]))
///         .add_generator(Field::choice(vec!["tag4", "tag5", "tag6"]))
///         .add_generator(Field::choice(vec!["tag7", "tag8", "tag9"])))
///     .add_field("scores", Field::array()
///         .add_generator(Field::range_int(1, 10))
///         .add_generator(Field::range_int(1, 10))
///         .add_generator(Field::range_int(1, 10)));
///
/// // Generate 10 rows
/// let rows = schema.generate_batch(10);
/// ```
///
/// ## Using the generator directly
///
/// ```no_run
/// use schemagen::r#generators::array::Array;
/// use schemagen::r#generators::range_int::RangeInt;
/// use schemagen::r#generators::choice::Choice;
/// use schemagen::value::Generator;
/// use rand::SeedableRng;
/// use rand::rngs::StdRng;
///
/// let mut rng = StdRng::seed_from_u64(42);
/// let mut generator = Array::new()
///     .add_generator(RangeInt::new(1, 100))
///     .add_generator(Choice::new(vec!["A".to_string(), "B".to_string()]));
///
/// for _ in 0..5 {
///     let value = generator.next_value(&mut rng);
///     println!("{:?}", value); // e.g., Array([42, "A"], [78, "B"], ...)
/// }
/// ```
///
/// # SQL Serialization
///
/// Arrays are serialized as PostgreSQL ARRAY syntax, for example:
/// `ARRAY[42, 78, 'A']` or `ARRAY[1, 2, 3, 4, 5]`
pub struct Array {
    generators: Vec<Box<dyn Generator>>,
}

impl Array {
    /// Creates a new empty [`Array`] generator.
    ///
    /// # Returns
    ///
    /// A new [`Array`] instance with no generators. Use [`add_generator`](Self::add_generator)
    /// to add generators to the array.
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
    /// * `generator` - The generator to add to the array. Its output will be included in each
    ///   generated array in the order added.
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
    ///     .add_generator(Field::range_int(100, 200))
    ///     .add_generator(Field::choice(vec!["A", "B", "C"]));
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
