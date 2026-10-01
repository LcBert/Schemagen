use crate::value::{Generator, Value};

pub struct Sequential {
    current: i64,
    step: i64,
}

impl Sequential {
    pub fn new(current: i64, step: i64) -> Self {
        Self { current, step }
    }
}

impl Generator for Sequential {
    fn next_value(&mut self, _rng: &mut dyn rand::prelude::Rng) -> Value {
        let val = self.current;
        self.current += self.step;
        Value::Int(val)
    }
}

#[cfg(test)]
mod tests {
    use rand::{SeedableRng, rngs::StdRng};

    use super::*;

    #[test]
    fn test() {
        let mut rng = StdRng::seed_from_u64(45);
        let mut generator = Sequential::new(1, 1);

        for i in 1..10000 {
            match generator.next_value(&mut rng) {
                Value::Int(v) => {
                    assert_eq!(v, i, "sequence goes out | expected: {i}, current: {v}")
                }
                _ => panic!("Unexpected type"),
            }
        }
    }
}
