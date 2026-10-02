//! UUID value generator.

use uuid::Uuid;

use crate::value::{Generator, Value};

/// Supported UUID standard versions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum UuidVersion {
    /// Random UUID v4 (RFC 4122 / RFC 9562).
    ///
    /// Universally unique identifiers generated from random numbers.
    /// Suitable for most use cases where uniqueness is required without ordering.
    #[default]
    V4,
    /// Time-ordered UUID v7 with millisecond precision (RFC 9562).
    ///
    /// UUIDs that are sorted by creation time, making them ideal for database indexes
    /// and applications where chronological ordering is beneficial.
    V7,
}

/// Generator producing UUID values of the specified version.
///
/// This generator creates universally unique identifiers that can be used as primary keys,
/// session tokens, or any scenario requiring unique identifiers. It supports both UUID v4
/// (random) and UUID v7 (time-ordered) standards.
///
/// # Common Use Cases
///
/// - **Primary keys**: Unique identifiers for database records
/// - **Session tokens**: User session or authentication tokens
/// - **Request IDs**: Tracking requests across distributed systems
/// - **File identifiers**: Unique names for uploaded files
/// - **Reference codes**: External reference numbers
///
/// # Examples
///
/// ## Using with Field (recommended)
///
/// ```no_run
/// use schemagen::{field::Field, schema::Schema};
/// use schemagen::r#generators::uuid_gen::UuidVersion;
///
/// let mut schema = Schema::new()
///     .add_field("id", Field::uuid(UuidVersion::V4))
///     .add_field("time_ordered_id", Field::uuid(UuidVersion::V7));
///
/// // Generate 10 rows
/// let rows = schema.generate_batch(10);
/// ```
///
/// ## Using the generator directly
///
/// ```no_run
/// use schemagen::r#generators::uuid_gen::{UuidGen, UuidVersion};
/// use schemagen::value::Generator;
/// use rand::SeedableRng;
/// use rand::rngs::StdRng;
///
/// let mut rng = StdRng::seed_from_u64(42);
/// let mut generator = UuidGen::new(UuidVersion::V4);
///
/// for _ in 0..5 {
///     let value = generator.next_value(&mut rng);
///     println!("{:?}", value); // e.g., "550e8400-e29b-41d4-a716-446655440000"
/// }
/// ```
///
/// # UUID Versions
///
/// ## UUID v4 (Random)
///
/// - Generated from random numbers
/// - No ordering guarantees
/// - Suitable for most general-purpose use cases
/// - Example: `550e8400-e29b-41d4-a716-446655440000`
///
/// ## UUID v7 (Time-ordered)
///
/// - Sorted by creation time (RFC 9562)
/// - Better for database indexes (reduces fragmentation)
/// - Includes timestamp for approximate creation time
/// - Example: `0189f4b8-5b5c-7b2a-8c4d-0e1f2a3b4c5d`
///
/// When to use v7:
/// - Primary keys in high-write databases
/// - Need chronological ordering
/// - Want to extract creation time from UUID
pub struct UuidGen {
    version: UuidVersion,
}

impl UuidGen {
    /// Creates a new [`UuidGen`] generator for the specified [`UuidVersion`].
    ///
    /// # Arguments
    ///
    /// * `version` - The UUID version to generate (V4 or V7).
    ///
    /// # Returns
    ///
    /// A new [`UuidGen`] instance that generates UUIDs of the specified version.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use schemagen::r#generators::uuid_gen::{UuidGen, UuidVersion};
    ///
    /// // Random UUID v4
    /// let generator = UuidGen::new(UuidVersion::V4);
    ///
    /// // Time-ordered UUID v7
    /// let generator = UuidGen::new(UuidVersion::V7);
    /// ```
    pub fn new(version: UuidVersion) -> Self {
        Self { version }
    }
}

impl Generator for UuidGen {
    fn next_value(&mut self, _rng: &mut dyn rand::prelude::Rng) -> Value {
        match self.version {
            UuidVersion::V4 => Value::Uuid(Uuid::new_v4()),
            UuidVersion::V7 => Value::Uuid(Uuid::now_v7()),
        }
    }
}

#[cfg(test)]
mod tests {
    use rand::{SeedableRng, rngs::StdRng};

    use super::*;

    #[test]
    fn test_uuid_v4() {
        let mut rng = StdRng::seed_from_u64(42);
        let mut generator = UuidGen::new(UuidVersion::V4);

        for _ in 0..100 {
            match generator.next_value(&mut rng) {
                Value::Uuid(u) => {
                    assert_eq!(u.get_version_num(), 4);
                }
                _ => panic!("Expected Value::Uuid"),
            }
        }
    }

    #[test]
    fn test_uuid_v7() {
        let mut rng = StdRng::seed_from_u64(42);
        let mut generator = UuidGen::new(UuidVersion::V7);

        for _ in 0..100 {
            match generator.next_value(&mut rng) {
                Value::Uuid(u) => {
                    assert_eq!(u.get_version_num(), 7);
                }
                _ => panic!("Expected Value::Uuid"),
            }
        }
    }
}
