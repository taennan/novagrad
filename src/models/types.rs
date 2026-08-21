use crate::utils::{
    Logger,
    events::AppEvent,
    metrics::{Metric, MetricTag},
};
use std::{collections::HashMap, fmt::Debug, io, path::PathBuf, sync::mpsc::Sender};

pub enum CategorisedModel {
    F32(Box<dyn Model<f32>>),
    F64(Box<dyn Model<f64>>),
}

impl CategorisedModel {
    pub fn category(&self) -> ValueCategory {
        match self {
            Self::F32(_) => ValueCategory::F32,
            Self::F64(_) => ValueCategory::F64,
        }
    }
}

pub enum CategorisedDataset {
    F32(Box<dyn Dataset<f32>>),
    F64(Box<dyn Dataset<f64>>),
}

impl CategorisedDataset {
    pub fn category(&self) -> ValueCategory {
        match self {
            Self::F32(_) => ValueCategory::F32,
            Self::F64(_) => ValueCategory::F64,
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum ValueCategory {
    F32,
    F64,
}

pub trait Model<N> {
    //fn new(hyperparams: &HashMap<String, HyperParam>) -> Self;
    fn name(&self) -> &'static str;
    fn hyperparam_config(&self) -> HashMap<String, HyperParamConfig>;
    fn train(
        &self,
        hyperparams: &HashMap<String, HyperParam>,
        dataset: &dyn Dataset<N>,
        app_sender: Sender<AppEvent>,
        logger: &Logger,
    ) -> Result<(), ()>;
    fn test(&self, dataset: &dyn Dataset<N>, logger: &Logger) -> Result<(), ()>;
    fn save(&self, filepath: PathBuf) -> Result<(), io::Error>;
    fn load(&self, filepath: PathBuf) -> Result<(), io::Error>;
}

#[derive(Debug)]
pub enum HyperParamConfig {
    Bool {
        default: bool,
    },
    String {
        default: String,
    },
    Float {
        default: f32,
        min: Option<f32>,
        max: Option<f32>,
    },
    Int {
        default: i32,
        min: Option<i32>,
        max: Option<i32>,
    },
}

impl HyperParamConfig {
    pub fn epochs() -> Self {
        let min_epochs = 1;
        Self::Int {
            default: min_epochs,
            min: Some(min_epochs),
            max: None,
        }
    }
}

pub enum HyperParam {
    Bool(bool),
    String(String),
    Float(f32),
    Int(i32),
}

pub trait Dataset<I> {
    fn name(&self) -> &'static str;

    fn train_len(&self) -> usize;
    fn validation_len(&self) -> usize;
    fn test_len(&self) -> usize;

    fn iter_train(&self) -> Box<dyn Iterator<Item = DatasetItem<I>>>;
    fn iter_validation(&self) -> Box<dyn Iterator<Item = DatasetItem<I>>>;
    fn iter_test(&self) -> Box<dyn Iterator<Item = DatasetItem<I>>>;
}

pub struct DatasetItem<I> {
    pub input: I,
    pub expected: I,
}
