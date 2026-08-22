use crate::{
    datasets::{CategorisedDataset, Dataset, Datasets, ExponentialDataset},
    models::{CategorisedModel, ExpPredictor, Model, Models, TrainStepOutcome},
    utils::{
        Logger,
        events::{AppEvent, ModelRunnerEvent},
        state::RunMode,
    },
};
use std::{
    collections::HashMap,
    sync::mpsc::{Receiver, Sender},
    thread::{self, JoinHandle},
    time::Duration,
};

pub fn spawn(
    model_runner_receiver: Receiver<ModelRunnerEvent>,
    model_runner_sender: Sender<ModelRunnerEvent>,
    app_sender: Sender<AppEvent>,
    logger: Logger,
) -> JoinHandle<()> {
    thread::spawn(move || {
        let mut categorised_model = Option::<CategorisedModel>::None;
        let mut categorised_dataset = Option::<CategorisedDataset>::None;
        let mut mode = RunMode::Test;
        let mut is_paused = true;

        loop {
            while let Ok(event) = model_runner_receiver.try_recv() {
                match event {
                    ModelRunnerEvent::Start(new_model, new_dataset, run_mode) => {
                        categorised_model = Some(match new_model {
                            Models::ExpPredictor => {
                                CategorisedModel::F32(Box::new(ExpPredictor::new(
                                    HashMap::new(),
                                    app_sender.clone(),
                                    logger.clone(),
                                )))
                            }
                        });
                        categorised_dataset = Some(match new_dataset {
                            Datasets::ExponentialF32 => CategorisedDataset::F32(Box::new(
                                ExponentialDataset::new(500, 1321432),
                            )),
                        });
                        mode = run_mode;
                        is_paused = false;
                    }
                    ModelRunnerEvent::Pause => {
                        is_paused = true;
                    }
                    ModelRunnerEvent::Resume => {
                        is_paused = false;
                    }
                    ModelRunnerEvent::Stop => {
                        categorised_model = None;
                        categorised_dataset = None;
                        is_paused = true;
                    }
                }
            }

            if !is_paused
                && let Some(categorised_model) = &mut categorised_model
                && let Some(categorised_dataset) = &categorised_dataset
            {
                match (categorised_model, &categorised_dataset) {
                    (CategorisedModel::F32(model), CategorisedDataset::F32(dataset)) => {
                        match mode {
                            RunMode::Test => run_testing(model, dataset),
                            RunMode::Train => run_training(
                                model.as_mut(),
                                dataset.as_ref(),
                                &mut is_paused,
                                &model_runner_sender,
                            ),
                        }
                    }
                    (CategorisedModel::F64(model), CategorisedDataset::F64(dataset)) => {
                        match mode {
                            RunMode::Test => run_testing(model, dataset),
                            RunMode::Train => run_training(
                                model.as_mut(),
                                dataset.as_ref(),
                                &mut is_paused,
                                &model_runner_sender,
                            ),
                        }
                    }
                    _ => {
                        panic!("Incompatible model and dataset types selected")
                    }
                }
            } else {
                thread::sleep(Duration::from_millis(250));
            }
        }
    })
}

fn run_training<T>(
    model: &mut dyn Model<T>,
    dataset: &dyn Dataset<T>,
    is_paused: &mut bool,
    model_runner_sender: &Sender<ModelRunnerEvent>,
) {
    match model.train_step(dataset).expect("Training step failed") {
        TrainStepOutcome::Continue => {}
        TrainStepOutcome::Done => {
            *is_paused = true;
            model_runner_sender.send(ModelRunnerEvent::Stop).unwrap();
        }
    }
}

fn run_testing<T>(model: &Box<dyn Model<T>>, dataset: &Box<dyn Dataset<T>>) {
    model.test(dataset.as_ref()).expect("Testing run failed")
}
