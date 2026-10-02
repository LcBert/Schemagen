//! `schemagen` is a fast and flexible fake/mock data generator library for Rust.
//!
//! It provides schema definition capabilities, various value generators (integers, floats,
//! patterns, choices, booleans, sequential counters), and serialization to JSON, JSONL, CSV, and SQL formats.
//!
//! # Quick Start
//!
//! ```no_run
//! use schemagen::{field::Field, schema::Schema};
//!
//! let mut schema = Schema::new()
//!     .with_seed(12345)
//!     .add_field("id", Field::sequential(1, 1))
//!     .add_field("name", Field::pattern("USR-####").add_entry('#', vec!['0'..='9']).unwrap())
//!     .add_field("price", Field::range_float(1.0, 100.0, Some(2)));
//!
//! // Generate JSON
//! let json = schema.generate_json(10)?;
//!
//! // Or export directly to files
//! schema.write_jsonl("output", 100)?;
//! schema.write_csv("output", 100, true)?;
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//!
//! # Features
//!
//! - **Schema Builder**: Define schemas with chainable `.add_field()` methods
//! - **Multiple Data Types**:
//!   - `range_int`: Uniform random integers within `[min, max]`
//!   - `range_float`: Uniform random floats within `[min, max]`, with optional decimal rounding
//!   - `pattern`: Template-based string generation (`#` for digits, `?` for uppercase letters)
//!   - `choice`: Random pick from a predefined collection of options
//!   - `check`: Random boolean flag
//!   - `sequential` / `sequential_float`: Deterministic counters with custom step increments
//!   - `uuid`: UUID v4 (random) or v7 (time-ordered) generation
//!   - `datetime` / `date` / `time`: Random date/time generation with custom formats
//!   - `optional_null`: Wrap any generator to produce NULL values with configurable probability
//! - **Streaming & Exporting**:
//!   - In-memory batch generation (`generate_batch`, `generate_json`)
//!   - Streaming iterator (`iter`)
//!   - Direct file export (`write_jsonl`, `write_csv`, `write_sql`)
//! - **Reproducibility**: Set seeds via `.with_seed(seed)` for deterministic output

pub mod field;
pub mod r#gen;
pub mod schema;
pub mod value;

