//! Boolean check generator.

use rand::RngExt;

use crate::value::{Generator, Value};

/// Generator producing random boolean values (`true` or `false`) with a 50/50 distribution.
#[derive(Default)]
pub struct Check {}

impl Check {
    /// Creates a new [`Check`] generator.
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

