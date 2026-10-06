//! Field factory functions for instantiating generators.

use crate::{
    generators::{
        array::Array,
        check::Check,
        choice::Choice,
        color_hex::ColorHex,
        color_rgb::ColorRgb,
        datetime::Datetime,
        object_gen::ObjectGen,
        optional_value::OptionalValue,
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
///
/// This struct provides static methods for creating various generator types,
/// which can then be added to a [`Schema`](crate::schema::Schema) using
/// [`Schema::add_field`](crate::schema::Schema::add_field).
///
/// # Example
///
/// ```no_run
/// use schemagen::{field::Field, schema::Schema};
///
/// let mut schema = Schema::new()
///     .add_field("id", Field::sequential(1, 1))
///     .add_field("name", Field::pattern("USR-####").add_entry('#', vec!['0'..='9']))
///     .add_field("price", Field::range_float(1.0, 100.0, Some(2)));
/// ```
pub struct Field;

impl Field {
    /// Creates a generator that produces uniform random integers within `[min, max]`.
    ///
    /// # Arguments
    ///
    /// * `min` - Minimum value (inclusive).
    /// * `max` - Maximum value (inclusive).
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
    /// # Arguments
    ///
    /// * `min` - Minimum value (inclusive).
    /// * `max` - Maximum value (inclusive).
    /// * `decimals` - Optional number of decimal places to round to.
    ///
    /// # Panics
    ///
    /// Panics if `min > max`.
    pub fn range_float(min: f64, max: f64, decimals: Option<u32>) -> RangeFloat {
        RangeFloat::new(min, max, decimals)
    }

    /// Creates a customizable template-based string generator.
    ///
    /// # Arguments
    ///
    /// * `template` - The template string with placeholder characters.
    ///
    /// Placeholders and their allowed character ranges can be configured using
    /// [`Pattern::add_entry`](crate::generators::pattern::Pattern::add_entry).
    pub fn pattern(template: impl Into<String>) -> Pattern {
        Pattern::new(template)
    }

    /// Creates a generator that randomly selects an element from a non-empty set of options.
    ///
    /// # Arguments
    ///
    /// * `options` - An iterable of string options to choose from.
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
    ///
    /// # Arguments
    ///
    /// * `current` - The starting value.
    /// * `step` - The increment to apply on each generation.
    pub fn sequential(current: i64, step: i64) -> Sequential {
        Sequential::new(current, step)
    }

    /// Creates a sequential floating-point generator starting at `current` and incrementing by `step` on each call,
    /// optionally rounded to `decimals` decimal places.
    ///
    /// # Arguments
    ///
    /// * `current` - The starting value.
    /// * `step` - The increment to apply on each generation.
    /// * `decimals` - Optional number of decimal places to round to.
    pub fn sequential_float(current: f64, step: f64, decimals: Option<u32>) -> SequentialFloat {
        SequentialFloat::new(current, step, decimals)
    }

    /// Creates a UUID generator for the specified [`UuidVersion`].
    ///
    /// # Arguments
    ///
    /// * `version` - The UUID version to generate (V4 or V7).
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
    /// # Arguments
    ///
    /// * `start` - The start boundary as a string (inclusive).
    /// * `end` - The end boundary as a string (inclusive).
    /// * `date_format` - The chrono format string to use for parsing and output.
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
    ///
    /// # Arguments
    ///
    /// * `start` - The start date as a string (inclusive).
    /// * `end` - The end date as a string (inclusive).
    ///
    /// # Panics
    ///
    /// Panics if `start` or `end` cannot be parsed, or if `start > end`.
    pub fn date(start: impl Into<String>, end: impl Into<String>) -> Datetime {
        Field::datetime(start, end, "%Y-%m-%d")
    }

    /// Creates a random time generator using the default format `"%H:%M:%S"`.
    ///
    /// # Arguments
    ///
    /// * `start` - The start time as a string (inclusive).
    /// * `end` - The end time as a string (inclusive).
    ///
    /// # Panics
    ///
    /// Panics if `start` or `end` cannot be parsed, or if `start > end`.
    pub fn time(start: impl Into<String>, end: impl Into<String>) -> Datetime {
        Field::datetime(start, end, "%H:%M:%S")
    }

    /// Creates a generator that wraps another generator and produces `NULL` values with a configurable probability.
    ///
    /// # Arguments
    ///
    /// * `generator` - The inner generator to wrap.
    /// * `chance` - Probability (0.0 to 1.0) of returning the generated value instead of `NULL`.
    ///   For example, `0.9` means 90% chance of generating a value, 10% chance of `NULL`.
    ///
    /// # Panics
    ///
    /// Panics if `chance` is not in the range `[0.0, 1.0]`.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use schemagen::{field::Field, schema::Schema};
    ///
    /// let mut schema = Schema::new()
    ///     .add_field("optional_id", Field::optional_value(Field::range_int(1, 100), 0.8));
    /// ```
    pub fn optional_value<G: Generator + 'static>(generator: G, chance: f32) -> OptionalValue<G> {
        OptionalValue::new(generator, chance)
    }

    /// Creates an array generator that combines multiple generators.
    ///
    /// The array generator runs multiple generators in sequence and collects their outputs
    /// into a single array. Use [`add_generator`](crate::generators::array::Array::add_generator)
    /// to add generators to the array.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use schemagen::{field::Field, schema::Schema};
    ///
    /// let mut schema = Schema::new()
    ///     .add_field("tags", Field::array()
    ///         .add_generator(Field::choice(vec!["tag1", "tag2", "tag3"]))
    ///         .add_generator(Field::choice(vec!["tag4", "tag5", "tag6"])));
    /// ```
    pub fn array() -> Array {
        Array::new()
    }

    /// Creates an object generator for nested structured data.
    ///
    /// The object generator creates JSON-like objects with named fields. Use
    /// [`add_field`](crate::generators::object_gen::ObjectGen::add_field) to add fields
    /// to the object.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use schemagen::{field::Field, schema::Schema};
    ///
    /// let mut schema = Schema::new()
    ///     .add_field("metadata", Field::object()
    ///         .add_field("id", Field::range_int(1, 1000))
    ///         .add_field("status", Field::choice(vec!["active", "inactive"])));
    /// ```
    pub fn object() -> ObjectGen {
        ObjectGen::new()
    }

    /// Creates a generator that produces random RGB color values.
    ///
    /// Each color consists of red, green, and blue components ranging from 0 to 255.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use schemagen::{field::Field, schema::Schema};
    ///
    /// let mut schema = Schema::new()
    ///     .add_field("theme_color", Field::color_rgb())
    ///     .add_field("background", Field::color_rgb());
    /// ```
    pub fn color_rgb() -> ColorRgb {
        ColorRgb::new()
    }

    /// Creates a generator that produces random hexadecimal color codes.
    ///
    /// Each color code is in the format `#RRGGBB` where each component is a hexadecimal digit.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use schemagen::{field::Field, schema::Schema};
    ///
    /// let mut schema = Schema::new()
    ///     .add_field("primary_color", Field::color_hex())
    ///     .add_field("accent_color", Field::color_hex());
    /// ```
    pub fn color_hex() -> ColorHex {
        ColorHex::new()
    }
}
