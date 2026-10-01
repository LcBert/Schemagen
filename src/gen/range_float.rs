use rand::RngExt;

use crate::value::{Generator, Value};

pub struct RangeFloat {
    min: f64,
    max: f64,
    decimals: Option<u32>,
}

impl RangeFloat {
    pub fn new(min: f64, max: f64, decimals: Option<u32>) -> Self {
        assert!(min <= max, "min must be <= max");
        Self { min, max, decimals }
    }
}

impl Generator for RangeFloat {
    fn next_value(&mut self, rng: &mut dyn rand::prelude::Rng) -> Value {
        let raw_val = rng.random_range(self.min..=self.max);
        let final_val = match self.decimals {
            Some(places) => {
                let factor = 10_f64.powi(places as i32);
                (raw_val * factor).round() / factor
            }
            None => raw_val,
        };
        Value::Float(final_val)
    }
}

#[cfg(test)]
mod tests {
    use rand::{SeedableRng, rngs::StdRng};

    use super::*;

    #[test]
    fn test() {
        let mut rng = StdRng::seed_from_u64(45);
        let min = 5.0;
        let max = 15.0;
        let decimals = 2;
        let mut generator = RangeFloat::new(min, max, Some(decimals));

        for _ in 0..10000 {
            match generator.next_value(&mut rng) {
                Value::Float(v) => assert!(
                    (min..=max).contains(&v),
                    "Value {v} out of range [{min}, {max}]"
                ),
                _ => panic!("Unexpected type"),
            }
        }
    }
}
