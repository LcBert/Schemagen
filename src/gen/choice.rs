//! Random discrete choice generator.

use rand::seq::IndexedRandom;

use crate::value::{Generator, Value};

/// Generator selecting a random string from a list of predefined options.
pub struct Choice {
    options: Vec<String>,
}

impl Choice {
    /// Creates a new [`Choice`] generator with the provided options.
    ///
    /// # Panics
    ///
    /// Panics if `options` is empty.
    pub fn new(options: Vec<String>) -> Self {
        assert!(!options.is_empty(), "Options cannot be empty");
        Self { options }
    }
}


impl Generator for Choice {
    fn next_value(&mut self, rng: &mut dyn rand::prelude::Rng) -> Value {
        Value::Text(self.options.choose(rng).unwrap().clone())
    }
}

#[cfg(test)]
mod tests {
    use rand::{SeedableRng, rngs::StdRng};

    use super::*;

    #[test]
    fn test_choice() {
        let mut rng = StdRng::seed_from_u64(45);
        let options: Vec<String> = vec![
            "test1".to_string(),
            "test2".to_string(),
            "test3".to_string(),
        ];
        let mut generator = Choice::new(options.clone());

        for _ in 0..10000 {
            match generator.next_value(&mut rng) {
                Value::Text(v) => assert!(options.contains(&v), "Value {v} not in options"),
                _ => panic!("Unexpected type"),
            }
        }
    }

    #[test]
    fn test_single_option() {
        let mut rng = StdRng::seed_from_u64(45);
        let mut generator = Choice::new(vec!["only_one".to_string()]);

        for _ in 0..100 {
            assert_eq!(
                generator.next_value(&mut rng),
                Value::Text("only_one".to_string())
            );
        }
    }

    #[test]
    #[should_panic(expected = "Options cannot be empty")]
    fn test_empty_options_panics() {
        Choice::new(Vec::new());
    }
}

