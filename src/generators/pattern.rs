//! Customizable template-based string pattern generator.

use std::{collections::HashMap, ops::RangeInclusive, sync::Arc};

use rand::seq::IndexedRandom;

use crate::value::{Generator, Value};

enum Step {
    Literal(char),
    Pool(Arc<[char]>),
}

/// Generator producing strings by substituting configurable placeholder characters in a template.
///
/// This generator allows you to define a template string with placeholder characters that get
/// replaced with random characters from specified ranges. Any character without a configured
/// entry is preserved verbatim. This is extremely useful for generating formatted IDs, codes,
/// phone numbers, license plates, or any structured string pattern.
///
/// # Common Use Cases
///
/// - **Product IDs**: PRD-1234-ABCD, SKU-5678
/// - **User IDs**: USR-12345, EMP-0001
/// - **Phone numbers**: +39 312 345 6789, (555) 123-4567
/// - **License plates**: AB-123-CD, XYZ-9876
/// - **Order codes**: ORD-2024-001, INV-5678
/// - **Serial numbers**: SN-XXXX-YYYY
///
/// # Examples
///
/// ## Using with Field (recommended)
///
/// ```no_run
/// use schemagen::{field::Field, schema::Schema};
///
/// let mut schema = Schema::new()
///     .add_field("product_id", Field::pattern("PRD-####-????")
///         .add_entry('#', vec!['0'..='9'])
///         .add_entry('?', vec!['A'..='Z']))
///     .add_field("user_code", Field::pattern("USR-#####")
///         .add_entry('#', vec!['0'..='9']));
///
/// // Generate 10 rows
/// let rows = schema.generate_batch(10);
/// ```
///
/// ## Using the generator directly
///
/// ```no_run
/// use schemagen::r#generators::pattern::Pattern;
/// use schemagen::value::Generator;
/// use rand::SeedableRng;
/// use rand::rngs::StdRng;
///
/// let mut rng = StdRng::seed_from_u64(42);
/// let mut generator = Pattern::new("ID-##-??")
///     .add_entry('#', vec!['0'..='9'])
///     .add_entry('?', vec!['A'..='Z']);
///
/// for _ in 0..5 {
///     let value = generator.next_value(&mut rng);
///     println!("{:?}", value); // e.g., "ID-42-AB", "ID-78-XY"
/// }
/// ```
///
/// ## Phone number patterns
///
/// ```no_run
/// use schemagen::r#generators::pattern::Pattern;
///
/// // Italian mobile: +39 3xx xxx xxxx
/// let phone_it = Pattern::new("+39 3## ### ####")
///     .add_entry('#', vec!['0'..='9']);
///
/// // USA format: (xxx) xxx-xxxx
/// let phone_us = Pattern::new("(###) ###-####")
///     .add_entry('#', vec!['0'..='9']);
/// ```
///
/// # Placeholder Configuration
///
/// Placeholders are configured using character ranges:
/// - `vec!['0'..='9']`: Digits only
/// - `vec!['A'..='Z']`: Uppercase letters only
/// - `vec!['a'..='z']`: Lowercase letters only
/// - `vec!['A'..='Z', 'a'..='z']`: Both cases
/// - `vec!['0'..='9', 'A'..='F']`: Hexadecimal
///
/// Any character in the template without a configured entry is preserved as-is.
pub struct Pattern {
    template: String,
    rules: HashMap<char, Vec<char>>,
    compiled: Option<Vec<Step>>,
}

impl Pattern {
    /// Creates a new [`Pattern`] generator with the specified template string.
    ///
    /// # Arguments
    ///
    /// * `template` - The template string with placeholder characters to be substituted.
    ///   Characters without configured rules will be preserved verbatim.
    ///
    /// # Returns
    ///
    /// A new [`Pattern`] instance. Use [`add_entry`](Self::add_entry) to configure placeholders.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use schemagen::r#generators::pattern::Pattern;
    ///
    /// let generator = Pattern::new("USER-####");
    /// ```
    pub fn new(template: impl Into<String>) -> Self {
        Self {
            template: template.into(),
            rules: HashMap::new(),
            compiled: None,
        }
    }

    /// Registers allowed character ranges for a specific placeholder character.
    ///
    /// # Arguments
    ///
    /// * `placeholder` - The character in the template to be replaced.
    /// * `ranges` - A vector of character ranges to randomly select from for this placeholder.
    ///
    /// # Returns
    ///
    /// Returns `self` to allow method chaining.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use schemagen::r#generators::pattern::Pattern;
    ///
    /// let generator = Pattern::new("ID-##-??")
    ///     .add_entry('#', vec!['0'..='9'])             // # becomes a digit
    ///     .add_entry('?', vec!['A'..='Z', 'a'..='z']); // ? becomes a letter
    /// ```
    pub fn add_entry(mut self, placeholder: char, ranges: Vec<RangeInclusive<char>>) -> Self {
        let mut char_pool = Vec::new();
        for r in ranges {
            if r.is_empty() {
                continue;
            }
            for ch in r {
                char_pool.push(ch);
            }
        }

        self.rules.insert(placeholder, char_pool);
        self.compiled = None;
        self
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
    fn next_value(&mut self, rng: &mut dyn rand::prelude::Rng) -> Value {
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
    fn test_pattern_with_placeholders() {
        let mut rng = StdRng::seed_from_u64(45);
        let template = "USR-####-??";
        let mut generator = Pattern::new(template)
            .add_entry('#', vec!['0'..='9'])
            .add_entry('?', vec!['A'..='Z']);

        for _ in 0..1000 {
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

    #[test]
    fn test_literal_only_pattern() {
        let mut rng = StdRng::seed_from_u64(45);
        let mut generator = Pattern::new("CONSTANT_STRING");

        for _ in 0..10 {
            assert_eq!(
                generator.next_value(&mut rng),
                Value::Text("CONSTANT_STRING".to_string())
            );
        }
    }
}
