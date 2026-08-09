use crate::models::Model;
use crossterm::event::KeyCode;
use std::collections::HashSet;

pub struct AppState {
    pub title: String,
    pub should_set_title: bool,
    pub keys_pressed: HashSet<KeyCode>,
    pub screen: ScreenState,
    pub should_exit: bool,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            title: "Novagrad".to_string(),
            should_set_title: true,
            keys_pressed: HashSet::new(),
            screen: ScreenState::default(),
            should_exit: false,
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

// ---------------------------------------------------------------------
// Model Run screen state
// ---------------------------------------------------------------------

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum ChartKind {
    Line,
    #[default]
    Bar,
}

#[derive(Debug, Default)]
pub struct MetricSeries {
    pub name: &'static str,
    pub chart_kind: ChartKind,
    /// Used when `chart_kind == ChartKind::Line`.
    pub line_data: Vec<(f64, f64)>,
    /// Used when `chart_kind == ChartKind::Bar`.
    pub bar_data: Vec<(&'static str, u64)>,
}

#[derive(Debug, Default)]
pub struct SystemStats {
    pub cpu_percent: f64,
    pub mem_used_mb: f64,
    pub mem_total_mb: f64,
    pub elapsed_seconds: u64,
    pub current_epoch: u32,
    pub total_epochs: u32,
}

#[derive(Debug, Default)]
pub struct ModelRunState {
    pub metrics: Vec<MetricSeries>,
    /// Index into `metrics` for whichever chart is currently shown.
    /// (Switching this — e.g. on Tab / Left-Right — is handled elsewhere.)
    pub selected_metric: usize,
    pub stats: SystemStats,
}

impl ModelRunState {
    pub fn mocked() -> Self {
        let loss_data: Vec<(f64, f64)> = (0..100)
            .map(|i| {
                let x = i as f64;
                let y = (1.0 / (1.0 + x * 0.05)) + (x * 0.13).sin() * 0.02;
                (x, y)
            })
            .collect();

        let accuracy_data: Vec<(f64, f64)> = (0..100)
            .map(|i| {
                let x = i as f64;
                let y = (1.0 - (1.0 / (1.0 + x * 0.08))).min(0.99);
                (x, y)
            })
            .collect();

        let lr_bars = vec![
            ("epoch 1", 100),
            ("epoch 2", 80),
            ("epoch 3", 64),
            ("epoch 4", 51),
            ("epoch 5", 41),
        ];

        Self {
            metrics: vec![
                MetricSeries {
                    name: "Loss",
                    chart_kind: ChartKind::Line,
                    line_data: loss_data,
                    bar_data: vec![],
                },
                MetricSeries {
                    name: "Accuracy",
                    chart_kind: ChartKind::Line,
                    line_data: accuracy_data,
                    bar_data: vec![],
                },
                MetricSeries {
                    name: "Learning Rate",
                    chart_kind: ChartKind::Bar,
                    line_data: vec![],
                    bar_data: lr_bars,
                },
            ],
            selected_metric: 0,
            stats: SystemStats {
                cpu_percent: 42.3,
                mem_used_mb: 2_150.0,
                mem_total_mb: 8_192.0,
                elapsed_seconds: 754,
                current_epoch: 5,
                total_epochs: 20,
            },
        }
    }
}
