use crate::{
    models::{
        CategorisedDataset, CategorisedModel, Models,
        datasets::{Datasets, ExponentialDataset},
        exponential_predictor_v2::ExpPredictor,
    },
    tui::types::{AppState, RunMode, ScreenState},
    utils::{
        Logger,
        events::{AppEvent, ModelRunnerEvent},
    },
};
use std::{
    collections::HashMap,
    sync::{
        Arc, Mutex, OnceLock,
        mpsc::{Receiver, Sender},
    },
    thread::{self, JoinHandle},
    time::Duration,
};

pub fn spawn(
    state: Arc<Mutex<AppState>>,
    model_runner_receiver: Receiver<ModelRunnerEvent>,
    app_sender: Sender<AppEvent>,
    logger: Arc<OnceLock<Logger>>,
) -> JoinHandle<()> {
    thread::spawn(move || {
        let mut categorised_model = Option::<CategorisedModel>::None;
        let mut categorised_dataset = Option::<CategorisedDataset>::None;
        let mut paused = true;
        loop {
            while let Ok(event) = model_runner_receiver.try_recv() {
                match event {
                    ModelRunnerEvent::Start(new_model) => {
                        categorised_model = Some(match new_model {
                            Models::ExpPredictor => {
                                CategorisedModel::F32(Box::new(ExpPredictor::new()))
                            }
                        });
                        paused = false;
                    }
                    ModelRunnerEvent::Pause => {
                        paused = true;
                    }
                    ModelRunnerEvent::Resume => {
                        paused = false;
                    }
                    ModelRunnerEvent::Stop => {
                        categorised_model = None;
                        paused = true;
                    }
                    ModelRunnerEvent::SetDataset(new_dataset) => {
                        categorised_dataset = Some(match new_dataset {
                            Datasets::ExponentialF32 => CategorisedDataset::F32(Box::new(
                                ExponentialDataset::new(500, 1321432),
                            )),
                        })
                    }
                }
            }

            if !paused
                && let Some(categorised_model) = &categorised_model
                && let Some(categorised_dataset) = &categorised_dataset
            {
                let mut state = state.lock().unwrap();
                if let ScreenState::ModelRun { mode, metrics, .. } = &mut state.screen {
                    let hyperparams = HashMap::new();
                    match (&categorised_model, &categorised_dataset) {
                        (CategorisedModel::F32(model), CategorisedDataset::F32(dataset)) => {
                            match mode {
                                RunMode::Test => model
                                    .test(dataset.as_ref(), logger.wait())
                                    .expect("Testing run failed"),
                                RunMode::Train => model
                                    .train(
                                        &hyperparams,
                                        dataset.as_ref(),
                                        app_sender.clone(),
                                        logger.wait(),
                                    )
                                    .expect("Training run failed"),
                            }
                        }
                        (CategorisedModel::F64(model), CategorisedDataset::F64(dataset)) => {
                            match mode {
                                RunMode::Test => model
                                    .test(dataset.as_ref(), logger.wait())
                                    .expect("Testing run failed"),
                                RunMode::Train => model
                                    .train(
                                        &hyperparams,
                                        dataset.as_ref(),
                                        app_sender.clone(),
                                        logger.wait(),
                                    )
                                    .expect("Training run failed"),
                            }
                        }
                        _ => {}
                    }
                }
            }

            thread::sleep(Duration::from_millis(250));
        }
    })
}
