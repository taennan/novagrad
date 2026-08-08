use std::{collections::HashMap, io, path::PathBuf};

pub trait Model {
    fn name(&self) -> &'static str;
    fn hyperparam_config(&self) -> HashMap<String, HyperParamConfig>;
    fn train(&self, hyperparams: &HashMap<String, HyperParam>) -> Result<(), ()>;
    fn test(&self, hyperparams: &HashMap<String, HyperParam>) -> Result<(), ()>;
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

struct Test {}

impl Model for Test {
    fn name(&self) -> &'static str {
        "Test Model"
    }

    fn hyperparam_config(&self) -> HashMap<String, HyperParamConfig> {
        let mut hyperparams = HashMap::new();
        hyperparams.insert("epochs".into(), HyperParamConfig::epochs());

        hyperparams
    }

    fn train(&self, hyperparams: &HashMap<String, HyperParam>) -> Result<(), ()> {
        Ok(())
    }

    fn test(&self, hyperparams: &HashMap<String, HyperParam>) -> Result<(), ()> {
        Ok(())
    }

    fn save(&self, filepath: PathBuf) -> Result<(), io::Error> {
        todo!()
    }

    fn load(&self, filepath: PathBuf) -> Result<(), io::Error> {
        todo!()
    }
}
