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
/// session tokens, or any scenario requiring unique identifiers.
///
/// # Example
///
/// ```no_run
/// use schemagen::{field::Field, schema::Schema};
/// use schemagen::r#gen::uuid_gen::UuidVersion;
///
/// let mut schema = Schema::new()
///     .add_field("id", Field::uuid(UuidVersion::V4))
///     .add_field("time_ordered_id", Field::uuid(UuidVersion::V7));
/// ```
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
    /// # Example
    ///
    /// ```no_run
    /// use schemagen::r#gen::uuid_gen::{UuidGen, UuidVersion};
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
