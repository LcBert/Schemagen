//! UUID value generator.

use uuid::Uuid;

use crate::value::{Generator, Value};

/// Supported UUID standard versions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum UuidVersion {
    /// Random UUID v4 (RFC 4122 / RFC 9562).
    #[default]
    V4,
    /// Time-ordered UUID v7 with millisecond precision (RFC 9562).
    V7,
}

/// Generator producing UUID values of the specified version.
pub struct UuidGen {
    version: UuidVersion,
}

impl UuidGen {
    /// Creates a new [`UuidGen`] generator for the specified [`UuidVersion`].
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
