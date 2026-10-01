//! Customizable template-based string pattern generator.

use std::{collections::HashMap, ops::RangeInclusive, sync::Arc};

use rand::{Rng, seq::IndexedRandom};

use crate::value::{Generator, Value};

enum Step {
    Literal(char),
    Pool(Arc<[char]>),
}

/// Generator producing strings by substituting configurable placeholder characters in a template.
///
/// Placeholders are configured using [`Pattern::add_entry`]. Any character without a configured
/// entry is preserved verbatim.
pub struct Pattern {
    template: String,
    rules: HashMap<char, Vec<char>>,
    compiled: Option<Vec<Step>>,
}

impl Pattern {
    /// Creates a new [`Pattern`] generator with the specified template string and no placeholder rules.
    pub fn new(template: impl Into<String>) -> Self {
        Self {
            template: template.into(),
            rules: HashMap::new(),
            compiled: None,
        }
    }

    /// Registers allowed character ranges for a specific placeholder character.
    ///
    /// # Errors
    ///
    /// Returns `Err(String)` if `ranges` is empty or contains a wrong `RangeInclusive<char>`.
    pub fn add_entry(
        mut self,
        placeholder: char,
        ranges: Vec<RangeInclusive<char>>,
    ) -> Result<Self, String> {
        if ranges.is_empty() {
            return Err(format!(
                "Ranges for placeholder '{placeholder}' cannot be empty"
            ));
        }

        let mut char_pool = Vec::new();
        for r in ranges {
            if r.is_empty() {
                return Err(format!(
                    "Range not valid for '{placeholder}': '{}'..='{}'",
                    r.start(),
                    r.end()
                ));
            }
            for ch in r {
                char_pool.push(ch);
            }
        }

        self.rules.insert(placeholder, char_pool);
        self.compiled = None;
        Ok(self)
    }

    fn compile(&mut self) {
        let pools: HashMap<char, Arc<[char]>> = self
            .rules
            .iter()
            .map(|(&k, v)| (k, Arc::from(v.as_slice())))
            .collect();

        let steps: Vec<Step> = self
            .template
            .chars()
            .map(|ch| match pools.get(&ch) {
                Some(pool) => Step::Pool(Arc::clone(pool)),
                None => Step::Literal(ch),
            })
            .collect();

        self.compiled = Some(steps);
    }
}

impl Generator for Pattern {
    fn next_value(&mut self, rng: &mut dyn Rng) -> Value {
        if self.compiled.is_none() {
            self.compile();
        }

        let steps = self.compiled.as_ref().unwrap();
        let mut result = String::with_capacity(self.template.len());

        for step in steps {
            match step {
                Step::Literal(c) => result.push(*c),
                Step::Pool(pool) => {
                    if let Some(&ch) = pool.choose(rng) {
                        result.push(ch);
                    }
                }
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
        let mut generator = Pattern::new(template)
            .add_entry('#', vec!['0'..='9'])
            .unwrap()
            .add_entry('?', vec!['A'..='Z'])
            .unwrap();

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
