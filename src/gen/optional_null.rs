//! Optional null wrapper generator.

use rand::RngExt;

use crate::value::{Generator, Value};

/// Generator wrapper that produces `NULL` values with a configurable probability.
///
/// This wraps any generator and occasionally returns [`Value::None`] instead of
/// the generated value, based on the specified chance.
///
/// # Type Parameters
///
/// * `G` - The inner generator type, must implement [`Generator`] and have a `'static` lifetime.
pub struct OptionalNull<G: Generator + 'static> {
    generator: G,
    chance: f32,
}

impl<G: 'static + Generator> OptionalNull<G> {
    /// Creates a new [`OptionalNull`] wrapper.
    ///
    /// # Arguments
    ///
    /// * `generator` - The inner generator to wrap.
    /// * `chance` - Probability (0.0 to 1.0) of returning the generated value instead of `NULL`.
    ///   For example, `0.9` means 90% chance of generating a value, 10% chance of `NULL`.
    ///
    /// # Panics
    ///
    /// Panics if `chance` is not in the range `[0.0, 1.0]`.
    pub fn new(generator: G, chance: f32) -> Self {
        assert!(
            (0.0 <= chance && chance <= 1.0),
            "chance not valid (0.0 <= chance <= 1.0)"
        );
        Self { generator, chance }
    }
}

impl<G: 'static + Generator> Generator for OptionalNull<G> {
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
    use crate::r#gen::range_int::RangeInt;

    #[test]
    fn test_optional_null_always_value() {
        let mut rng = StdRng::seed_from_u64(42);
        let inner = RangeInt::new(1, 100);
        let mut generator = OptionalNull::new(inner, 1.0);

        for _ in 0..100 {
            match generator.next_value(&mut rng) {
                Value::Int(_) => {}
                Value::None => panic!("Should never return None with chance=1.0"),
                _ => panic!("Unexpected type"),
            }
        }
    }

    #[test]
    fn test_optional_null_always_none() {
        let mut rng = StdRng::seed_from_u64(42);
        let inner = RangeInt::new(1, 100);
        let mut generator = OptionalNull::new(inner, 0.0);

        for _ in 0..100 {
            match generator.next_value(&mut rng) {
                Value::None => {}
                _ => panic!("Should always return None with chance=0.0"),
            }
        }
    }

    #[test]
    fn test_optional_null_mixed() {
        let mut rng = StdRng::seed_from_u64(42);
        let inner = RangeInt::new(1, 100);
        let mut generator = OptionalNull::new(inner, 0.5);

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
        OptionalNull::new(inner, 1.5);
    }
}
