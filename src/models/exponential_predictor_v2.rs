use crate::{
    engine::{ExpOp, MseOp, NodeRef},
    models::{Dataset, HyperParam, HyperParamConfig, Model},
};
use rand::prelude::*;
use std::{collections::HashMap, io, path::PathBuf};

#[derive(Debug)]
pub struct ExpPredictor {
    pub input: NodeRef,
    pub output: NodeRef,
    pub param: NodeRef,
}

impl ExpPredictor {
    pub fn new() -> Self {
        let input = NodeRef::from(0.0);
        input.set_label("input");

        let exponential = NodeRef::from(1.0);
        exponential.set_label("exponential");

        let output = NodeRef::chained(&[input.clone(), exponential.clone()], &ExpOp);

        Self {
            input,
            output,
            param: exponential,
        }
    }
}

impl Model<f32> for ExpPredictor {
    fn name(&self) -> &'static str {
        "Exponential Predictor"
    }

    fn hyperparam_config(&self) -> HashMap<String, HyperParamConfig> {
        HashMap::new()
    }

    fn train(
        &self,
        hyperparams: &HashMap<String, HyperParam>,
        dataset: &dyn Dataset<f32>,
    ) -> Result<(), ()> {
        let epochs = hyperparams.get("epoch").map_or(4usize, |h| match h {
            HyperParam::Int(e) => *e as usize,
            _ => 4,
        });
        let step = hyperparams.get("step").map_or(1e-6, |h| match h {
            HyperParam::Float(e) => *e as f32,
            _ => 1e-6,
        });

        let batch_size = hyperparams.get("batch_size").map_or(1usize, |h| match h {
            HyperParam::Int(e) => *e as usize,
            _ => 1,
        });
        let batch_size_node = NodeRef::from(batch_size as f32);

        let expected_node = NodeRef::from(0.0);
        let loss = NodeRef::chained(
            &[
                batch_size_node.clone(),
                expected_node.clone(),
                self.output.clone(),
            ],
            &MseOp,
        );

        println!();
        println!("Starting training for model ExponentialPredictor...");
        println!("Params: epochs={} step={}", epochs, step);
        println!();

        for epoch in 0..epochs {
            println!("Training Epoch {}", epoch);

            let mut avg_loss_sum = 0.0;

            for item in dataset.iter_train() {
                self.input.set_value(item.input);
                expected_node.set_value(item.expected);
                loss.clear_gradients();

                loss.compute_value();
                loss.compute_gradients();

                let param_value = self.param.value();
                let param_gradient = self.param.gradient();
                self.param.set_value(param_value - (param_gradient * step));

                avg_loss_sum += loss.value();
            }

            println!(
                "  average loss={}",
                avg_loss_sum / dataset.train_len() as f32
            );
            println!("  param value={}", self.param.value());
        }

        println!("Training complete!");
        println!();

        Ok(())
    }

    fn test(&self, dataset: &dyn Dataset<f32>) -> Result<(), ()> {
        let dataset = build_dataset(25);

        println!("Starting testing...");
        println!("  dataset_length={}", dataset.len());

        for (input_value, expected_value) in dataset {
            self.input.set_value(input_value);

            self.output.compute_value();
            let actual_value = self.output.value();

            println!(
                "  input={} expected={} actual={} error={}",
                input_value,
                expected_value,
                actual_value,
                actual_value - expected_value
            );
        }

        println!();

        Ok(())
    }

    fn save(&self, filepath: PathBuf) -> Result<(), io::Error> {
        todo!()
    }

    fn load(&self, filepath: PathBuf) -> Result<(), io::Error> {
        todo!()
    }
}

fn build_dataset(length: usize) -> Vec<(f32, f32)> {
    (0..length)
        .map(|_| {
            let x = rand::rng().random::<f32>();
            let y = x.powf(2.0);
            (x, y)
        })
        .collect::<Vec<_>>()
}

pub struct _ExpPredictor;

impl Model<f32> for _ExpPredictor {
    fn name(&self) -> &'static str {
        "Exponential Predictor Stub"
    }

    fn hyperparam_config(&self) -> HashMap<String, HyperParamConfig> {
        HashMap::new()
    }

    fn train(
        &self,
        hyperparams: &HashMap<String, HyperParam>,
        dataset: &dyn Dataset<f32>,
    ) -> Result<(), ()> {
        todo!()
    }

    fn test(&self, dataset: &dyn Dataset<f32>) -> Result<(), ()> {
        todo!()
    }

    fn save(&self, filepath: PathBuf) -> Result<(), io::Error> {
        todo!()
    }

    fn load(&self, filepath: PathBuf) -> Result<(), io::Error> {
        todo!()
    }
}
