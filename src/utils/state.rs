use crate::{
    datasets::{CategorisedDataset, DatasetMeta, Datasets, ExponentialDataset},
    models::{CategorisedModel, ModelMeta, Models, exponential_predictor_v2::ExpPredictor},
    utils::metrics::Metric,
};
use crossterm::event::KeyCode;
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub struct ModelState {
    pub model: Option<CategorisedModel>,
    pub dataset: Option<CategorisedDataset>,
    pub mode: RunMode,
    pub is_paused: bool,
}

#[derive(Default)]
pub struct AppState {
    pub keys_pressed: HashSet<KeyCode>,
    pub screen: ScreenState,
}

#[derive(Default)]
pub enum ScreenState {
    #[default]
    Home,
    ModelSelect {
        wizard: ModelSelectWizard,
    },
    ModelRun {
        mode: RunMode,
        metrics: HashMap<&'static str, Metric>,
        selected_metric: Option<&'static str>,
    },
}

// ---------------------------------------------------------------------
// Model Select wizard state
// ---------------------------------------------------------------------

/// Which step of the "start a new run" wizard the user is on.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WizardStep {
    SelectModel,
    SelectRunMode,
    SelectDataset,
    Confirm,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum RunMode {
    #[default]
    Test,
    Train,
}

impl From<&RunMode> for String {
    fn from(value: &RunMode) -> Self {
        Self::from(value.clone())
    }
}

impl From<RunMode> for String {
    fn from(value: RunMode) -> Self {
        match value {
            RunMode::Train => "Train".into(),
            RunMode::Test => "Test".into(),
        }
    }
}

/// All state for the multi-step wizard on the Model Select screen:
/// pick a model -> pick train/test -> pick a dataset -> confirm.
pub struct ModelSelectWizard {
    pub step: WizardStep,
    pub models: Vec<Models>,
    pub selected_model: Option<usize>,
    pub run_modes: Vec<RunMode>,
    pub selected_run_mode: Option<usize>,
    pub datasets: Vec<Datasets>,
    pub selected_dataset: Option<usize>,
    /// Index currently highlighted in whichever list is active for `step`.
    /// (State transitions / key handling are intentionally not wired up
    /// here — that's handled elsewhere.)
    pub cursor: usize,
}

impl Default for ModelSelectWizard {
    fn default() -> Self {
        Self {
            step: WizardStep::SelectModel,
            models: vec![Models::ExpPredictor],
            selected_model: None,
            run_modes: vec![RunMode::Train, RunMode::Test],
            selected_run_mode: None,
            datasets: vec![Datasets::ExponentialF32],
            selected_dataset: None,
            cursor: 0,
        }
    }
}
