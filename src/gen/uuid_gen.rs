use uuid::Uuid;

use crate::value::{Generator, Value};
pub enum UuidVersion {
    V4,
    V7,
}

pub struct UuidGen {
    version: UuidVersion,
}

impl UuidGen {
    pub fn new(version: UuidVersion) -> Self {
        Self { version }
    }
}

impl Generator for UuidGen {
    fn next_value(&mut self, _rng: &mut dyn rand::prelude::Rng) -> Value {
        match self.version {
            UuidVersion::V4 => Value::Uuid(Uuid::new_v4()),
            UuidVersion::V7 => Value::Uuid(Uuid::now_v7()),
        }
    }
}
