use std::fmt::Debug;

#[derive(Clone, Copy, Debug)]
pub enum Datasets {
    ExponentialF32,
}

impl Datasets {
    pub fn meta(&self) -> DatasetMeta {
        match self {
            Self::ExponentialF32 => DatasetMeta {
                name: "Exponential",
                description: "f32's raised to the power of 2",
                samples: 10_000,
            },
        }
    }
}

pub trait Dataset<I> {
    fn train_len(&self) -> usize;
    fn validation_len(&self) -> usize;
    fn test_len(&self) -> usize;

    fn iter_train(&self) -> Box<dyn Iterator<Item = DatasetItem<I>>>;
    fn iter_validation(&self) -> Box<dyn Iterator<Item = DatasetItem<I>>>;
    fn iter_test(&self) -> Box<dyn Iterator<Item = DatasetItem<I>>>;
}

#[derive(Debug)]
pub struct DatasetMeta {
    pub name: &'static str,
    pub description: &'static str,
    pub samples: usize,
}

pub struct DatasetItem<I> {
    pub input: I,
    pub expected: I,
}

pub enum CategorisedDataset {
    F32(Box<dyn Dataset<f32>>),
    F64(Box<dyn Dataset<f64>>),
}

impl Debug for CategorisedDataset {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "CategorisedModel::{}",
            match self {
                Self::F32(_) => "F32",
                Self::F64(_) => "F64",
            }
        )
    }
}
