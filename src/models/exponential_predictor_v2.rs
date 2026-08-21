use crate::{
    engine::{ExpOp, MseOp, NodeRef},
    models::{Dataset, HyperParam, HyperParamConfig, Model},
    utils::{
        Logger,
        events::AppEvent,
        metrics::{Metric, MetricScalar, MetricTag},
    },
};
use rand::prelude::*;
use std::{collections::HashMap, io, path::PathBuf, sync::mpsc::Sender};

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
        app_sender: Sender<AppEvent>,
        logger: &Logger,
    ) -> Result<(), ()> {
        let epoch_key = "epoch";
        let epoch_tag = MetricTag::Usize(epoch_key);
        let default_epoch = 4usize;
        let epochs = hyperparams
            .get(epoch_key)
            .map_or(default_epoch, |h| match h {
                HyperParam::Int(e) => *e as usize,
                _ => default_epoch,
            });
        if let Err(_) = app_sender.send(AppEvent::MetricModified(
            epoch_tag,
            Metric::Usize(MetricScalar::new(0)),
        )) {
            logger.error("Failed to send metric");
        }

        let step_key = "step";
        let step_tag = MetricTag::F32(step_key);
        let default_step = 1e-6;
        let step = hyperparams.get(step_key).map_or(default_step, |h| match h {
            HyperParam::Float(e) => *e as f32,
            _ => default_step,
        });
        if let Err(_) = app_sender.send(AppEvent::MetricModified(
            step_tag,
            Metric::F32(MetricScalar::new(step)),
        )) {
            logger.error("Failed to send metric");
        }

        let batch_size_key = "batch_size";
        let batch_size_tag = MetricTag::Usize(batch_size_key);
        let default_batch_size = 5usize;
        let batch_size = hyperparams
            .get(batch_size_key)
            .map_or(default_batch_size, |h| match h {
                HyperParam::Int(e) => *e as usize,
                _ => default_batch_size,
            });
        if let Err(_) = app_sender.send(AppEvent::MetricModified(
            batch_size_tag,
            Metric::Usize(MetricScalar::new(batch_size)),
        )) {
            logger.error("Failed to send metric");
        }

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

        logger.log("Starting training for model ExponentialPredictor...");
        logger.log(&format!("Params: epochs={} step={}", epochs, step));

        for epoch in 0..epochs {
            logger.log(&format!("Training Epoch {}", epoch));

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

            logger.log(&format!(
                "  average loss={}",
                avg_loss_sum / dataset.train_len() as f32
            ));
            logger.log(&format!("  param value={}", self.param.value()));
        }

        logger.log("Training complete!");

        Ok(())
    }

    fn test(&self, dataset: &dyn Dataset<f32>, logger: &Logger) -> Result<(), ()> {
        let dataset = build_dataset(25);

        logger.log("Starting testing...");
        logger.log(&format!("  dataset_length={}", dataset.len()));

        for (input_value, expected_value) in dataset {
            self.input.set_value(input_value);

            self.output.compute_value();
            let actual_value = self.output.value();

            logger.log(&format!(
                "  input={} expected={} actual={} error={}",
                input_value,
                expected_value,
                actual_value,
                actual_value - expected_value,
            ));
        }

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
