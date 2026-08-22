use crate::{
    datasets::Dataset,
    utils::{Logger, events::AppEvent},
};
use std::{
    collections::HashMap,
    fmt::Debug,
    io,
    path::PathBuf,
    sync::{RwLock, mpsc::Sender},
};

#[derive(Clone, Copy, Debug)]
pub enum Models {
    ExpPredictor,
}

impl Models {
    pub fn meta(&self) -> ModelMeta {
        match self {
            Self::ExpPredictor => ModelMeta {
                name: "Exponential Predictor",
                description: "Single node network",
                checkpoint: None,
            },
        }
    }
}

pub enum CategorisedModel {
    F32(Box<dyn Model<f32>>),
    F64(Box<dyn Model<f64>>),
}

impl Debug for CategorisedModel {
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

pub trait Model<N> {
    fn hyperparam_config(&self) -> HashMap<String, HyperParamConfig>;
    fn train(&self, dataset: &dyn Dataset<N>) -> Result<(), ()>;
    fn train_step(&mut self, dataset: &dyn Dataset<N>) -> Result<TrainStepOutcome, ()>;
    fn test(&self, dataset: &dyn Dataset<N>) -> Result<(), ()>;
    fn save(&self, filepath: PathBuf) -> Result<(), io::Error>;
    fn load(&self, filepath: PathBuf) -> Result<(), io::Error>;
}

#[derive(Debug)]
pub enum TrainStepOutcome {
    Continue,
    Done,
}

#[derive(Debug)]
pub struct ModelMeta {
    pub name: &'static str,
    pub description: &'static str,
    pub checkpoint: Option<PathBuf>,
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

#[derive(Debug)]
pub enum HyperParam {
    Bool(bool),
    String(String),
    Float(f32),
    Int(i32),
}
