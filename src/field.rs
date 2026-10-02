//! Field factory functions for instantiating generators.

use crate::{
    r#gen::{
        check::Check,
        choice::Choice,
        datetime::Datetime,
        optional_null::OptionalNull,
        pattern::Pattern,
        range_float::RangeFloat,
        range_int::RangeInt,
        sequential::Sequential,
        sequential_float::SequentialFloat,
        uuid_gen::{UuidGen, UuidVersion},
    },
    value::Generator,
};

/// Factory utility for constructing data generators.
pub struct Field;

impl Field {
    pub fn optiona_null<G: Generator + 'static>(generator: G, chance: f32) -> OptionalNull<G> {
        OptionalNull::new(generator, chance)
    }

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

    /// Creates a customizable template-based string generator.
    ///
    /// Placeholders and their allowed character ranges can be configured using
    /// [`Pattern::add_entry`].
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

    /// Creates a UUID generator for the specified [`UuidVersion`].
    pub fn uuid(version: UuidVersion) -> UuidGen {
        UuidGen::new(version)
    }

    /// Creates a random UUID v4 generator.
    pub fn uuid_v4() -> UuidGen {
        UuidGen::new(UuidVersion::V4)
    }

    /// Creates a time-ordered UUID v7 generator.
    pub fn uuid_v7() -> UuidGen {
        UuidGen::new(UuidVersion::V7)
    }

    /// Creates a random date, time, or datetime generator matching the provided format string.
    ///
    /// Automatically detects whether the input corresponds to:
    /// - Date and time (e.g. `start: "2024-01-01 00:00:00"`, `end: "2024-12-31 23:59:59"`, `format: "%Y-%m-%d %H:%M:%S"`)
    /// - Date only (e.g. `start: "2024-01-01"`, `end: "2024-12-31"`, `format: "%Y-%m-%d"`)
    /// - Time only (e.g. `start: "08:00:00"`, `end: "18:00:00"`, `format: "%H:%M:%S"`)
    ///
    /// # Panics
    ///
    /// Panics if `start` or `end` cannot be parsed with `date_format`, or if `start > end`.
    pub fn datetime(
        start: impl Into<String>,
        end: impl Into<String>,
        date_format: impl Into<String>,
    ) -> Datetime {
        Datetime::new(start, end, date_format)
    }

    /// Creates a random date generator using the default format `"%Y-%m-%d"`.
    pub fn date(start: impl Into<String>, end: impl Into<String>) -> Datetime {
        Datetime::new(start, end, "%Y-%m-%d")
    }

    /// Creates a random time generator using the default format `"%H:%M:%S"`.
    pub fn time(start: impl Into<String>, end: impl Into<String>) -> Datetime {
        Datetime::new(start, end, "%H:%M:%S")
    }
}
