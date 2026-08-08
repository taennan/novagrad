use crate::models::Model;
use crossterm::event::KeyCode;
use std::collections::HashSet;

pub type AppSystem = Box<dyn Fn(&mut AppState)>;

#[derive(Default)]
pub struct AppState {
    pub title: String,
    pub should_set_title: bool,
    pub keys_pressed: HashSet<KeyCode>,
    pub screen: ScreenState,
    pub should_exit: bool,
}

#[derive(Default)]
pub enum ScreenState {
    #[default]
    Home,
    ModelSelect {
        wizard: ModelSelectWizard,
    },
    ModelRun {
        training: bool,
        model: Box<dyn Model>,
        run: ModelRunState,
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

    pub fn is_new(&self) -> bool {
        matches!(self, ModelChoice::New { .. })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RunMode {
    Train,
    Test,
}

impl RunMode {
    pub fn label(&self) -> &'static str {
        match self {
            RunMode::Train => "Train",
            RunMode::Test => "Test",
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

// ---------------------------------------------------------------------
// Model Run screen state
// ---------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChartKind {
    Line,
    Bar,
}

pub struct MetricSeries {
    pub name: &'static str,
    pub chart_kind: ChartKind,
    /// Used when `chart_kind == ChartKind::Line`.
    pub line_data: Vec<(f64, f64)>,
    /// Used when `chart_kind == ChartKind::Bar`.
    pub bar_data: Vec<(&'static str, u64)>,
}

pub struct SystemStats {
    pub cpu_percent: f64,
    pub mem_used_mb: f64,
    pub mem_total_mb: f64,
    pub elapsed_seconds: u64,
    pub current_epoch: u32,
    pub total_epochs: u32,
}

pub struct ModelRunState {
    pub metrics: Vec<MetricSeries>,
    /// Index into `metrics` for whichever chart is currently shown.
    /// (Switching this — e.g. on Tab / Left-Right — is handled elsewhere.)
    pub selected_metric: usize,
    pub stats: SystemStats,
}
