use rand_distr::{Distribution, Normal};

use crate::{
    utils::round_float,
    value::{Generator, Value},
};

pub struct GaussianFloat {
    mu: f64,
    sigma: f64,
    decimals: Option<u32>,
}

impl GaussianFloat {
    pub fn new(mu: f64, sigma: f64, decimals: Option<u32>) -> Self {
        Self {
            mu,
            sigma,
            decimals,
        }
    }
}

impl Generator for GaussianFloat {
    fn next_value(&mut self, rng: &mut dyn rand::Rng) -> Value {
        let normale = Normal::new(self.mu, self.sigma).unwrap();
        let extracted = normale.sample(rng);
        Value::Float(round_float(extracted, self.decimals))
    }
}
