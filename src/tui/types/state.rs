use crate::tui::types::{
    Datapoint, MetricScalar, MetricSeries,
    metrics::{Metric, MetricTag},
};
use crossterm::event::KeyCode;
use std::collections::{HashMap, HashSet};

#[derive(Default)]
pub struct AppState {
    pub keys_pressed: HashSet<KeyCode>,
    pub screen: ScreenState,
}

impl AppState {
    pub fn new_mocked_model_run_screen() -> Self {
        let mut epochs_metric = MetricScalar::new(5);
        epochs_metric.format_str = Some("Epoch {} of {}");

        let mut loss_metric = MetricSeries::default();
        loss_metric.datapoints = (0..100)
            .map(|i| {
                let x = i as u32;
                let y = (1.0 / (1.0 + x as f32 * 0.05)) + (x as f32 * 0.13).sin() * 0.02;
                Datapoint::new(x as u32, y)
            })
            .collect::<Vec<_>>();
        loss_metric.format_str = Some("Loss {.3}");

        let mut metrics = HashMap::new();
        metrics.insert(MetricTag::Usize("epochs"), Metric::Usize(epochs_metric));
        metrics.insert(MetricTag::F32Series("loss"), Metric::F32Series(loss_metric));

        Self {
            screen: ScreenState::ModelRun {
                metrics,
                mode: RunMode::default(),
                selected_metric: None,
            },
            ..Default::default()
        }
    }
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
        metrics: HashMap<MetricTag, Metric>,
        selected_metric: Option<MetricTag>,
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

/// A model the user can pick: either a fresh, untrained model, or an
/// existing checkpoint to resume training/testing on.
#[derive(Debug, Clone)]
pub enum ModelChoice {
    New {
        name: &'static str,
        architecture: &'static str,
    },
    Existing {
        name: &'static str,
        checkpoint: &'static str,
        last_trained: &'static str,
    },
}

impl ModelChoice {
    pub fn display_name(&self) -> &'static str {
        match self {
            ModelChoice::New { name, .. } => name,
            ModelChoice::Existing { name, .. } => name,
        }
    }
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

#[derive(Debug, Clone)]
pub struct DatasetChoice {
    pub name: &'static str,
    pub description: &'static str,
    pub sample_count: usize,
}

/// All state for the multi-step wizard on the Model Select screen:
/// pick a model -> pick train/test -> pick a dataset -> confirm.
pub struct ModelSelectWizard {
    pub step: WizardStep,

    pub models: Vec<ModelChoice>,
    pub selected_model: Option<usize>,

    pub run_modes: Vec<RunMode>,
    pub selected_run_mode: Option<usize>,

    pub datasets: Vec<DatasetChoice>,
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
            models: mock_models(),
            selected_model: None,
            run_modes: vec![RunMode::Train, RunMode::Test],
            selected_run_mode: None,
            datasets: mock_datasets(),
            selected_dataset: None,
            cursor: 0,
        }
    }
}

fn mock_models() -> Vec<ModelChoice> {
    vec![
        ModelChoice::New {
            name: "sine-net",
            architecture: "3-layer MLP",
        },
        ModelChoice::New {
            name: "tanh-classifier",
            architecture: "CNN",
        },
        ModelChoice::Existing {
            name: "sine-net-v1",
            checkpoint: "epoch_42.pt",
            last_trained: "2 days ago",
        },
        ModelChoice::Existing {
            name: "tanh-classifier-prod",
            checkpoint: "epoch_100.pt",
            last_trained: "last week",
        },
    ]
}

fn mock_datasets() -> Vec<DatasetChoice> {
    vec![
        DatasetChoice {
            name: "sine-wave-synth",
            description: "Synthetic sine wave samples with noise",
            sample_count: 10_000,
        },
        DatasetChoice {
            name: "tanh-curve-synth",
            description: "Synthetic tanh curve samples with noise",
            sample_count: 8_500,
        },
        DatasetChoice {
            name: "real-sensor-data",
            description: "Recorded sensor readings from the test rig",
            sample_count: 42_318,
        },
    ]
}
