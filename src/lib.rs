//! `schemagen` is a fast and flexible fake/mock data generator library for Rust.
//!
//! It provides schema definition capabilities, various value generators (integers, floats,
//! patterns, choices, booleans, sequential counters), and serialization to JSON, JSONL, and CSV formats.

pub mod field;
pub mod r#gen;
pub mod schema;
pub mod value;

