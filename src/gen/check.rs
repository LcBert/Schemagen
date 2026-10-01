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
    fn test() {
        let mut rng = StdRng::seed_from_u64(45);
        let mut generator = Check::new();

        for _ in 0..10000 {
            match generator.next_value(&mut rng) {
                Value::Boolean(v) => assert!(v || !v, "Value is not true or false"),
                _ => panic!("Unexpected type"),
            }
        }
    }
}
