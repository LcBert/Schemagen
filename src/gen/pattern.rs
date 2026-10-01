use rand::{Rng, RngExt};

use crate::value::{Generator, Value};

pub struct Pattern {
    template: String,
}

impl Pattern {
    pub fn new(template: impl Into<String>) -> Self {
        Self {
            template: template.into(),
        }
    }
}

impl Generator for Pattern {
    fn next_value(&mut self, rng: &mut dyn Rng) -> Value {
        let mut result: String = String::with_capacity(self.template.len());
        for ch in self.template.chars() {
            match ch {
                '#' => {
                    let digit: u8 = rng.random_range(b'0'..=b'9');
                    result.push(digit as char);
                }
                '?' => {
                    let digit: u8 = rng.random_range(b'A'..=b'Z');
                    result.push(digit as char);
                }
                other => result.push(other),
            }
        }
        Value::Text(result)
    }
}

#[cfg(test)]
mod tests {
    use rand::{SeedableRng, rngs::StdRng};

    use super::*;

    #[test]
    fn test() {
        let mut rng = StdRng::seed_from_u64(45);
        let template = "USR-####-??";
        let mut generator = Pattern::new(template);

        for _ in 0..10000 {
            match generator.next_value(&mut rng) {
                Value::Text(v) => {
                    assert_eq!(v.len(), template.len(), "Length mismatch");

                    for (tpl_ch, gen_ch) in template.chars().zip(v.chars()) {
                        match tpl_ch {
                            '#' => assert!(
                                gen_ch.is_ascii_digit(),
                                "Expected digit for '#', got '{gen_ch}'"
                            ),
                            '?' => assert!(
                                gen_ch.is_ascii_uppercase(),
                                "Expected uppercase letter for '?', got '{gen_ch}'"
                            ),
                            other => assert_eq!(
                                gen_ch, other,
                                "Expected literal '{other}', got '{gen_ch}'"
                            ),
                        }
                    }
                }
                _ => panic!("Unexpected type"),
            }
        }
    }
}
