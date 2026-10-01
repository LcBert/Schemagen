use rand::Rng;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(untagged)]
pub enum Value {
    Int(i64),
    Float(f64),
    Text(String),
    Boolean(bool),
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Int(v) => write!(f, "{v}"),
            Value::Float(v) => write!(f, "{v}"),
            Value::Text(v) => write!(f, "{v}"),
            Value::Boolean(v) => write!(f, "{v}"),
        }
    }
}

pub trait Generator {
    fn next_value(&mut self, rng: &mut dyn Rng) -> Value;
}
