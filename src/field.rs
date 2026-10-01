//! Field factory functions for instantiating generators.

use crate::r#gen::{
    check::Check, choice::Choice, pattern::Pattern, range_float::RangeFloat, range_int::RangeInt,
    sequential::Sequential, sequential_float::SequentialFloat,
};

/// Factory utility for constructing data generators.
pub struct Field;

impl Field {
    /// Creates a generator that produces uniform random integers within `[min, max]`.
    ///
    /// # Panics
    ///
    /// Panics if `min > max`.
    pub fn range_int(min: i64, max: i64) -> RangeInt {
        RangeInt::new(min, max)
    }

    /// Creates a generator that produces uniform random floats within `[min, max]`,
    /// optionally rounded to the specified number of decimal places.
    ///
    /// # Panics
    ///
    /// Panics if `min > max`.
    pub fn range_float(min: f64, max: f64, decimals: Option<u32>) -> RangeFloat {
        RangeFloat::new(min, max, decimals)
    }

    /// Creates a template-based string generator.
    ///
    /// - `'#'` is replaced with a random digit (`'0'..='9'`).
    /// - `'?'` is replaced with a random uppercase letter (`'A'..='Z'`).
    /// - All other characters are preserved literally.
    pub fn pattern(template: impl Into<String>) -> Pattern {
        Pattern::new(template)
    }

    /// Creates a generator that randomly selects an element from a non-empty set of options.
    ///
    /// # Panics
    ///
    /// Panics if `options` is empty.
    pub fn choice<I, S>(options: I) -> Choice
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        Choice::new(options.into_iter().map(Into::into).collect())
    }

    /// Creates a generator that produces random boolean values (`true` or `false`) with equal probability.
    pub fn check() -> Check {
        Check::new()
    }

    /// Creates a sequential integer generator starting at `current` and incrementing by `step` on each call.
    pub fn sequential(current: i64, step: i64) -> Sequential {
        Sequential::new(current, step)
    }

    /// Creates a sequential floating-point generator starting at `current` and incrementing by `step` on each call,
    /// optionally rounded to `decimals` decimal places.
    pub fn sequential_float(current: f64, step: f64, decimals: Option<u32>) -> SequentialFloat {
        SequentialFloat::new(current, step, decimals)
    }
}

