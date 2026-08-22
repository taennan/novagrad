use crate::{
    datasets::Dataset,
    engine::{ExpOp, MseOp, NodeRef},
    models::{HyperParam, HyperParamConfig, Model, TrainStepOutcome},
    utils::{
        Logger, Primitive,
        events::AppEvent,
        metrics::{Metric, MetricScalar, MetricSeries},
    },
};
use rand::prelude::*;
use ratatui::widgets::GraphType;
use std::{collections::HashMap, io, path::PathBuf, sync::mpsc::Sender};

#[derive(Debug)]
pub struct ExpPredictor {
    pub input: NodeRef,
    pub output: NodeRef,
    pub param: NodeRef,
    total_epochs: usize,
    batch_size: usize,
    step: f32,
    app_sender: Sender<AppEvent>,
    logger: Logger,
    training_run: Option<TrainingRun>,
}

#[derive(Clone, Debug)]
struct TrainingRun {
    epoch: usize,
    batch: usize,
}

impl ExpPredictor {
    const EPOCH_KEY: &str = "epoch";
    const BATCH_SIZE_KEY: &str = "batch_size";
    const STEP_KEY: &str = "step";
    const LOSS_KEY: &str = "loss";
    const PARAM_KEY: &str = "param";

    pub fn new(
        hyperparams: HashMap<String, HyperParam>,
        app_sender: Sender<AppEvent>,
        logger: Logger,
    ) -> Self {
        let input = NodeRef::from(0.0);
        input.set_label("input");

        let param = NodeRef::from(1.0);
        param.set_label("exponential");

        let output = NodeRef::chained(&[input.clone(), param.clone()], &ExpOp);

        let default_epoch = 4usize;
        let total_epochs = hyperparams
            .get(Self::EPOCH_KEY)
            .map_or(default_epoch, |h| match h {
                HyperParam::Int(e) => *e as usize,
                _ => default_epoch,
            });

        let default_step = 1e-6;
        let step = hyperparams
            .get(Self::STEP_KEY)
            .map_or(default_step, |h| match h {
                HyperParam::Float(e) => *e as f32,
                _ => default_step,
            });

        let default_batch_size = 5usize;
        let batch_size = hyperparams
            .get(Self::BATCH_SIZE_KEY)
            .map_or(default_batch_size, |h| match h {
                HyperParam::Int(e) => *e as usize,
                _ => default_batch_size,
            });

        Self {
            input,
            output,
            param,
            total_epochs,
            step,
            batch_size,
            app_sender,
            logger,
            training_run: None,
        }
    }
}

impl Model<f32> for ExpPredictor {
    fn hyperparam_config(&self) -> HashMap<String, HyperParamConfig> {
        HashMap::new()
    }

    fn train(&self, dataset: &dyn Dataset<f32>) -> Result<(), ()> {
        if let Err(_) = self.app_sender.send(AppEvent::MetricAdded(
            Self::EPOCH_KEY,
            Metric::Usize(MetricScalar::new(0)),
        )) {
            self.logger.error("Failed to send metric");
        }

        if let Err(_) = self.app_sender.send(AppEvent::MetricAdded(
            Self::STEP_KEY,
            Metric::F32(MetricScalar::new(self.step)),
        )) {
            self.logger.error("Failed to send metric");
        }

        if let Err(_) = self.app_sender.send(AppEvent::MetricAdded(
            Self::BATCH_SIZE_KEY,
            Metric::Usize(MetricScalar::new(self.batch_size)),
        )) {
            self.logger.error("Failed to send metric");
        }

        let batch_size_node = NodeRef::from(self.batch_size as f32);
        let expected_node = NodeRef::from(0.0);
        let loss = NodeRef::chained(
            &[
                batch_size_node.clone(),
                expected_node.clone(),
                self.output.clone(),
            ],
            &MseOp,
        );

        self.logger
            .log("Starting training for model ExponentialPredictor...");
        self.logger.log(&format!(
            "Params: epochs={} step={}",
            self.total_epochs, self.step
        ));

        for epoch in 0..self.total_epochs {
            self.logger.log(&format!("Training Epoch {}", epoch));

            let mut avg_loss_sum = 0.0;

            for item in dataset.iter_train() {
                self.input.set_value(item.input);
                expected_node.set_value(item.expected);
                loss.clear_gradients();

                loss.compute_value();
                loss.compute_gradients();

                let param_value = self.param.value();
                let param_gradient = self.param.gradient();
                self.param
                    .set_value(param_value - (param_gradient * self.step));

                avg_loss_sum += loss.value();
            }

            self.logger.log(&format!(
                "  average loss={}",
                avg_loss_sum / dataset.train_len() as f32
            ));
            self.logger
                .log(&format!("  param value={}", self.param.value()));
        }

        self.logger.log("Training complete!");

        Ok(())
    }

    fn train_step(&mut self, dataset: &dyn Dataset<f32>) -> Result<TrainStepOutcome, ()> {
        let mut training_run = self.training_run.clone().unwrap_or_else(|| {
            let epoch_metric =
                MetricScalar::new_formatted(0, &format!("Epoch {{}} of {}", self.total_epochs));
            if let Err(_) = self.app_sender.send(AppEvent::MetricAdded(
                Self::EPOCH_KEY,
                Metric::Usize(epoch_metric),
            )) {
                self.logger.error("Failed to create total_epochs metric");
            }

            if let Err(_) = self.app_sender.send(AppEvent::MetricAdded(
                Self::STEP_KEY,
                Metric::F32(MetricScalar::new(self.step)),
            )) {
                self.logger.error("Failed to create step metric");
            }

            if let Err(_) = self.app_sender.send(AppEvent::MetricAdded(
                Self::BATCH_SIZE_KEY,
                Metric::Usize(MetricScalar::new(self.batch_size)),
            )) {
                self.logger.error("Failed to create batch_size metric");
            }

            if let Err(_) = self.app_sender.send(AppEvent::MetricAdded(
                Self::LOSS_KEY,
                Metric::F32Series(MetricSeries::new(vec![], GraphType::Line)),
            )) {
                self.logger.error("Failed to create loss metric");
            }

            if let Err(_) = self.app_sender.send(AppEvent::MetricAdded(
                Self::PARAM_KEY,
                Metric::F32Series(MetricSeries::new(vec![], GraphType::Line)),
            )) {
                self.logger.error("Failed to create param metric");
            }

            self.logger
                .log("Starting training for model ExponentialPredictor...");
            self.logger.log(&format!(
                "Params: epochs={} step={}",
                self.total_epochs, self.step
            ));

            TrainingRun { epoch: 0, batch: 0 }
        });

        if training_run.epoch >= self.total_epochs {
            return Ok(TrainStepOutcome::Done);
        }

        let batch_size_node = NodeRef::from(self.batch_size as f32);
        let expected_node = NodeRef::from(0.0);
        let loss = NodeRef::chained(
            &[
                batch_size_node.clone(),
                expected_node.clone(),
                self.output.clone(),
            ],
            &MseOp,
        );

        let mut batch_item_index = 0usize;
        let mut batch_loss_sum = 0.0;

        for item in dataset.iter_train() {
            if batch_item_index >= self.batch_size {
                break;
            }

            self.input.set_value(item.input);
            expected_node.set_value(item.expected);
            loss.clear_gradients();

            loss.compute_value();
            loss.compute_gradients();

            let param_value = self.param.value();
            let param_gradient = self.param.gradient();
            self.param
                .set_value(param_value - (param_gradient * self.step));

            batch_loss_sum += loss.value();
            batch_item_index += 1;
        }

        let batch_loss = batch_loss_sum / self.batch_size as f32;
        self.logger.log(&format!(
            "  batch_loss={:.2} param_value={:.5}",
            batch_loss,
            self.param.value()
        ));

        if let Err(_) = self.app_sender.send(AppEvent::MetricModified(
            Self::LOSS_KEY,
            Primitive::F32(batch_loss),
        )) {
            self.logger.error("Failed to send loss metric");
        }
        if let Err(_) = self.app_sender.send(AppEvent::MetricModified(
            Self::PARAM_KEY,
            Primitive::F32(self.param.value()),
        )) {
            self.logger.error("Failed to send param metric");
        }

        let total_batches = dataset.train_len() / self.batch_size;
        let did_end_epoch = training_run.batch >= total_batches;
        if did_end_epoch {
            training_run.epoch += 1;
            training_run.batch = 0;
        } else {
            training_run.batch += 1;
        }

        self.training_run = Some(training_run);

        Ok(TrainStepOutcome::Continue)
    }

    fn test(&self, dataset: &dyn Dataset<f32>) -> Result<(), ()> {
        let dataset = build_dataset(25);

        self.logger.log("Starting testing...");
        self.logger
            .log(&format!("  dataset_length={}", dataset.len()));

        for (input_value, expected_value) in dataset {
            self.input.set_value(input_value);

            self.output.compute_value();
            let actual_value = self.output.value();

            self.logger.log(&format!(
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
