use crate::{
    models::{
        CategorisedDataset, CategorisedModel, Dataset, Model, Models,
        datasets::{Datasets, ExponentialDataset},
        exponential_predictor_v2::ExpPredictor,
    },
    utils::{
        Logger,
        events::{AppEvent, ModelRunnerEvent},
        state::{ModelState, RunMode},
    },
};
use std::{
    collections::HashMap,
    sync::{
        Arc, Mutex, OnceLock, RwLock,
        mpsc::{Receiver, Sender},
    },
    thread::{self, JoinHandle},
    time::Duration,
};

pub fn spawn(
    model_state: Arc<Mutex<ModelState>>,
    model_runner_receiver: Receiver<ModelRunnerEvent>,
    //logger: Arc<OnceLock<Logger>>,
) -> JoinHandle<()> {
    thread::spawn(move || {
        while let Ok(event) = model_runner_receiver.try_recv() {
            let mut state = model_state.lock().unwrap();
            match event {
                ModelRunnerEvent::Start(new_model, new_dataset, run_mode) => {
                    state.model = Some(match new_model {
                        Models::ExpPredictor => {
                            CategorisedModel::F32(Box::new(ExpPredictor::new()))
                        }
                    });
                    state.dataset = Some(match new_dataset {
                        Datasets::ExponentialF32 => {
                            CategorisedDataset::F32(Box::new(ExponentialDataset::new(500, 1321432)))
                        }
                    });
                    state.mode = run_mode;
                    state.is_paused = false;
                }
                ModelRunnerEvent::Pause => {
                    state.is_paused = true;
                }
                ModelRunnerEvent::Resume => {
                    state.is_paused = false;
                }
                ModelRunnerEvent::Stop => {
                    state.model = None;
                    state.dataset = None;
                    state.is_paused = true;
                }
            }
        }
    })
}
