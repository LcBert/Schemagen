//! RGB color generator.

use rand::RngExt;

use crate::value::{Generator, Value};

/// Generator producing random RGB color values.
///
/// This generator creates random colors in RGB format with values for red, green, and blue
/// components each ranging from 0 to 255. Useful for generating color data, theme colors,
/// or any application requiring random color values.
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
pub struct ColorRgb {}

impl ColorRgb {
    /// Creates a new [`ColorRgb`] generator.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use schemagen::r#generators::color_rgb::ColorRgb;
    ///
    /// let generator = ColorRgb::new();
    /// ```
    pub fn new() -> Self {
        Self {}
    }
}

impl Generator for ColorRgb {
    fn next_value(&mut self, rng: &mut dyn rand::Rng) -> Value {
        Value::ColorRGB(
            rng.random_range(0..=255),
            rng.random_range(0..=255),
            rng.random_range(0..=255),
        )
    }
}
