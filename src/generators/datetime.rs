//! Flexible date, time, and datetime value generator.

use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime, Timelike};
use rand::RngExt;

use crate::value::{Generator, Value};

#[derive(Debug, Clone)]
enum Mode {
    /// Full date and time based on Unix epoch seconds.
    DateTime { min_ts: i64, max_ts: i64 },
    /// Date only based on Unix epoch seconds at 00:00:00 UTC.
    Date { min_ts: i64, max_ts: i64 },
    /// Time only based on seconds from midnight (0..=86399).
    Time { min_secs: u32, max_secs: u32 },
}

/// Generator producing random date, time, or datetime strings within a specified range.
///
/// This generator automatically detects whether the input corresponds to a full datetime,
/// a date only, or a time only based on the provided format string. It uses the chrono
/// library for parsing and formatting, supporting a wide variety of date/time formats.
/// This is ideal for generating timestamps, birth dates, appointment times, or any temporal data.
///
/// # Common Use Cases
///
/// - **Timestamps**: Created/updated timestamps for records
/// - **Birth dates**: Random dates within a realistic range
/// - **Appointment times**: Business hours or specific time slots
/// - **Event dates**: Historical or future events
/// - **Schedule data**: Recurring events or time-based data
///
/// # Examples
///
/// ## Using with Field (recommended)
///
/// ```no_run
/// use schemagen::{field::Field, schema::Schema};
///
/// let mut schema = Schema::new()
///     .add_field("created_at", Field::datetime(
///         "2024-01-01 00:00:00",
///         "2024-12-31 23:59:59",
///         "%Y-%m-%d %H:%M:%S"
///     ))
///     .add_field("birth_date", Field::date("1990-01-01", "2000-12-31"))
///     .add_field("appointment_time", Field::time("09:00:00", "17:00:00"));
///
/// // Generate 10 rows
/// let rows = schema.generate_batch(10);
/// ```
///
/// ## Using the generator directly
///
/// ```no_run
/// use schemagen::r#generators::datetime::Datetime;
/// use schemagen::value::Generator;
/// use rand::SeedableRng;
/// use rand::rngs::StdRng;
///
/// let mut rng = StdRng::seed_from_u64(42);
///
/// // Full datetime
/// let mut generator = Datetime::new(
///     "2024-01-01 00:00:00",
///     "2024-12-31 23:59:59",
///     "%Y-%m-%d %H:%M:%S"
/// );
///
/// for _ in 0..5 {
///     let value = generator.next_value(&mut rng);
///     println!("{:?}", value); // e.g., "2024-06-15 14:32:45"
/// }
/// ```
///
/// # Date/Time Formats
///
/// The generator automatically detects the type based on successful parsing:
/// - **Full datetime**: `"2024-01-01 00:00:00"` with format `"%Y-%m-%d %H:%M:%S"`
/// - **Date only**: `"2024-01-01"` with format `"%Y-%m-%d"`
/// - **Time only**: `"08:00:00"` with format `"%H:%M:%S"`
///
/// See [chrono format documentation](https://docs.rs/chrono/latest/chrono/format/strftime/index.html)
/// for all supported format specifiers.
///
/// # Panics
///
/// Panics if:
/// - `start` or `end` cannot be parsed with `date_format`
/// - `start > end`
pub struct Datetime {
    mode: Mode,
    date_format: String,
}

impl Datetime {
    /// Creates a new [`Datetime`] generator by parsing `start` and `end` with `date_format`.
    ///
    /// # Arguments
    ///
    /// * `start` - The start boundary as a string (inclusive).
    /// * `end` - The end boundary as a string (inclusive).
    /// * `date_format` - The chrono format string to use for parsing and output.
    ///
    /// # Returns
    ///
    /// A new [`Datetime`] instance that generates values in the range `[start, end]`.
    ///
    /// # Panics
    ///
    /// Panics if `start` or `end` cannot be parsed with `date_format`, or if `start > end`.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use schemagen::r#generators::datetime::Datetime;
    ///
    /// // Full datetime
    /// let generator = Datetime::new(
    ///     "2024-01-01 00:00:00",
    ///     "2024-12-31 23:59:59",
    ///     "%Y-%m-%d %H:%M:%S"
    /// );
    ///
    /// // Date only
    /// let generator = Datetime::new("2024-01-01", "2024-12-31", "%Y-%m-%d");
    ///
    /// // Time only
    /// let generator = Datetime::new("08:00:00", "18:00:00", "%H:%M:%S");
    /// ```
    pub fn new(
        start: impl Into<String>,
        end: impl Into<String>,
        date_format: impl Into<String>,
    ) -> Self {
        let start_str = start.into();
        let end_str = end.into();
        let date_format = date_format.into();

        // 1. Try parsing as full NaiveDateTime
        if let (Ok(s), Ok(e)) = (
            NaiveDateTime::parse_from_str(&start_str, &date_format),
            NaiveDateTime::parse_from_str(&end_str, &date_format),
        ) {
            let min_ts = s.and_utc().timestamp();
            let max_ts = e.and_utc().timestamp();
            assert!(min_ts <= max_ts, "start datetime must be <= end datetime");
            return Self {
                mode: Mode::DateTime { min_ts, max_ts },
                date_format,
            };
        }

        // 2. Try parsing as NaiveDate
        if let (Ok(s), Ok(e)) = (
            NaiveDate::parse_from_str(&start_str, &date_format),
            NaiveDate::parse_from_str(&end_str, &date_format),
        ) {
            let min_ts = s
                .and_hms_opt(0, 0, 0)
                .expect("valid time")
                .and_utc()
                .timestamp();
            let max_ts = e
                .and_hms_opt(0, 0, 0)
                .expect("valid time")
                .and_utc()
                .timestamp();
            assert!(min_ts <= max_ts, "start date must be <= end date");
            return Self {
                mode: Mode::Date { min_ts, max_ts },
                date_format,
            };
        }

        // 3. Try parsing as NaiveTime
        if let (Ok(s), Ok(e)) = (
            NaiveTime::parse_from_str(&start_str, &date_format),
            NaiveTime::parse_from_str(&end_str, &date_format),
        ) {
            let min_secs = s.num_seconds_from_midnight();
            let max_secs = e.num_seconds_from_midnight();
            assert!(min_secs <= max_secs, "start time must be <= end time");
            return Self {
                mode: Mode::Time { min_secs, max_secs },
                date_format,
            };
        }

        panic!(
            "Failed to parse datetime range ['{start_str}', '{end_str}'] with format '{date_format}'"
        );
    }
}

impl Generator for Datetime {
    fn next_value(&mut self, rng: &mut dyn rand::prelude::Rng) -> Value {
        match self.mode {
            Mode::DateTime { min_ts, max_ts } => {
                let ts = rng.random_range(min_ts..=max_ts);
                if let Some(dt) = DateTime::from_timestamp(ts, 0) {
                    Value::Text(dt.format(&self.date_format).to_string())
                } else {
                    Value::None
                }
            }
            Mode::Date { min_ts, max_ts } => {
                let ts = rng.random_range(min_ts..=max_ts);
                if let Some(dt) = DateTime::from_timestamp(ts, 0) {
                    Value::Text(dt.date_naive().format(&self.date_format).to_string())
                } else {
                    Value::None
                }
            }
            Mode::Time { min_secs, max_secs } => {
                let secs = rng.random_range(min_secs..=max_secs);
                if let Some(time) = NaiveTime::from_num_seconds_from_midnight_opt(secs, 0) {
                    Value::Text(time.format(&self.date_format).to_string())
                } else {
                    Value::None
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use rand::{SeedableRng, rngs::StdRng};

    use super::*;

    #[test]
    fn test_datetime_range() {
        let mut rng = StdRng::seed_from_u64(42);
        let format = "%Y-%m-%d %H:%M:%S";
        let mut generator = Datetime::new("2024-01-01 00:00:00", "2024-01-02 00:00:00", format);

        for _ in 0..1000 {
            match generator.next_value(&mut rng) {
                Value::Text(s) => {
                    let parsed = NaiveDateTime::parse_from_str(&s, format);
                    assert!(parsed.is_ok(), "Failed to parse generated datetime: {s}");
                }
                _ => panic!("Expected Value::Text"),
            }
        }
    }

    #[test]
    fn test_date_only() {
        let mut rng = StdRng::seed_from_u64(42);
        let format = "%Y-%m-%d";
        let mut generator = Datetime::new("2024-01-01", "2024-12-31", format);

        for _ in 0..1000 {
            match generator.next_value(&mut rng) {
                Value::Text(s) => {
                    let parsed = NaiveDate::parse_from_str(&s, format);
                    assert!(parsed.is_ok(), "Failed to parse generated date: {s}");
                }
                _ => panic!("Expected Value::Text"),
            }
        }
    }

    #[test]
    fn test_time_only() {
        let mut rng = StdRng::seed_from_u64(42);
        let format = "%H:%M:%S";
        let mut generator = Datetime::new("08:00:00", "18:00:00", format);

        for _ in 0..1000 {
            match generator.next_value(&mut rng) {
                Value::Text(s) => {
                    let parsed = NaiveTime::parse_from_str(&s, format);
                    assert!(parsed.is_ok(), "Failed to parse generated time: {s}");
                }
                _ => panic!("Expected Value::Text"),
            }
        }
    }

    #[test]
    #[should_panic(expected = "start datetime must be <= end datetime")]
    fn test_invalid_range_panics() {
        Datetime::new(
            "2024-01-02 00:00:00",
            "2024-01-01 00:00:00",
            "%Y-%m-%d %H:%M:%S",
        );
    }
}
