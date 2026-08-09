use crate::models::{Dataset, DatasetItem};
use rand::{RngExt, SeedableRng, distr::StandardUniform, rngs::StdRng};

pub struct ExponentialDataset {
    length: usize,
    seed: u64,
}

impl ExponentialDataset {
    pub fn new(length: usize, seed: u64) -> Self {
        if length <= 0 {
            panic!("ExponentialDataset len must be greater than 0");
        }
        Self { length, seed }
    }

    fn make_iterator(length: usize, seed: u64) -> Box<dyn Iterator<Item = DatasetItem<f32>>> {
        let rng = StdRng::seed_from_u64(seed);
        let range = rng
            .sample_iter::<f32, StandardUniform>(StandardUniform)
            .take(length)
            .map(|v| DatasetItem {
                input: v,
                expected: v.powi(2),
            });

        Box::new(range)
    }
}

impl Dataset<f32> for ExponentialDataset {
    fn name(&self) -> &'static str {
        "Exponential"
    }

    fn train_len(&self) -> usize {
        self.length
    }

    fn validation_len(&self) -> usize {
        self.train_len() / 10
    }

    fn test_len(&self) -> usize {
        self.train_len() / 3
    }

    fn iter_train(&self) -> Box<dyn Iterator<Item = DatasetItem<f32>>> {
        Self::make_iterator(self.train_len(), self.seed)
    }

    fn iter_validation(&self) -> Box<dyn Iterator<Item = DatasetItem<f32>>> {
        Self::make_iterator(self.validation_len(), self.seed * 2)
    }

    fn iter_test(&self) -> Box<dyn Iterator<Item = DatasetItem<f32>>> {
        Self::make_iterator(self.test_len(), self.seed * 4)
    }
}
