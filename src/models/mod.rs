pub mod datasets;
pub mod exponential_predictor;
pub mod exponential_predictor_v2;
pub mod sine_predictor;
pub mod types;

pub use types::*;

#[derive(Debug)]
pub enum Models {
    ExpPredictor,
}
