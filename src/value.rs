//! Core data types and generator abstraction.

use rand::Rng;
use serde::Serialize;
use uuid::Uuid;

/// Represents a generated value of a supported primitive or text type.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(untagged)]
pub enum Value {
    /// 64-bit signed integer.
    Int(i64),
    /// 64-bit floating point number.
    Float(f64),
    /// UTF-8 text string.
    Text(String),
    /// Boolean value.
    Boolean(bool),
    /// Uuid value.
    Uuid(Uuid),
}

impl Value {
    pub fn to_sql(&self) -> String {
        match self {
            Value::Int(v) => v.to_string(),
            Value::Float(v) => v.to_string(),
            Value::Text(v) => format!("'{}'", v.replace('\'', "''")),
            Value::Boolean(v) => {
                if *v {
                    "TRUE".to_string()
                } else {
                    "FALSE".to_string()
                }
            }
            Value::Uuid(v) => v.to_string(),
        }
    }
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Int(v) => write!(f, "{v}"),
            Value::Float(v) => write!(f, "{v}"),
            Value::Text(v) => write!(f, "{v}"),
            Value::Boolean(v) => write!(f, "{v}"),
            Value::Uuid(v) => write!(f, "{v}"),
        }
    }
}

/// Trait implemented by all data generators.
pub trait Generator {
    /// Produces the next generated [`Value`] using the provided random number generator.
    fn next_value(&mut self, rng: &mut dyn Rng) -> Value;
}
