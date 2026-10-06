//! Hexadecimal color generator.

use crate::{
    generators::pattern::Pattern,
    value::{Generator, Value},
};

/// Generator producing random hexadecimal color codes.
///
/// This generator creates random color codes in hexadecimal format (e.g., `#FF5733`, `#A1B2C3`).
/// Each color code consists of a `#` prefix followed by 6 hexadecimal characters representing
/// the red, green, and blue components. Useful for web colors, CSS styles, or any application
/// requiring hex color format.
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
pub struct ColorHex {}

impl ColorHex {
    /// Creates a new [`ColorHex`] generator.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use schemagen::r#generators::color_hex::ColorHex;
    ///
    /// let generator = ColorHex::new();
    /// ```
    pub fn new() -> Self {
        Self {}
    }
}

impl Generator for ColorHex {
    fn next_value(&mut self, rng: &mut dyn rand::Rng) -> Value {
        Pattern::new("#??????")
            .add_entry('?', vec!['0'..='9', 'A'..='F'])
            .next_value(rng)
    }
}
