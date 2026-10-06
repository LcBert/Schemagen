use rand_distr::{Distribution, Normal};

use crate::value::{Generator, Value};

pub struct GaussianInt {
    mu: f64,
    sigma: f64,
}

impl GaussianInt {
    pub fn new(mu: f64, sigma: f64) -> Self {
        Self { mu, sigma }
    }
}

impl Generator for GaussianInt {
    fn next_value(&mut self, rng: &mut dyn rand::Rng) -> Value {
        let normale = Normal::new(self.mu, self.sigma).unwrap();
        let extracted = normale.sample(rng);
        Value::Int(extracted as i64)
    }
}
