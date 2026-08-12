use crate::tui::types::{AppSystemContext, ModelSelectWizard, RunMode, ScreenState, WizardStep};
use crossterm::event::KeyCode;
use std::collections::HashMap;

/// Handles user input for the model select wizard:
/// - Up/Down arrows navigate the list
/// - Enter confirms the current selection and advances the wizard
/// - Backspace goes back to the previous step
pub fn run(ctx: AppSystemContext) {
    let ScreenState::ModelSelect { wizard } = &mut ctx.state.screen else {
        return;
    };

    if ctx.state.keys_pressed.contains(&KeyCode::Up) {
        move_cursor(wizard, -1);
    } else if ctx.state.keys_pressed.contains(&KeyCode::Down) {
        move_cursor(wizard, 1);
    } else if ctx.state.keys_pressed.contains(&KeyCode::Enter) {
        match go_forward(wizard) {
            Some(screen) => ctx.state.screen = screen,
            _ => {}
        }
    } else if ctx.state.keys_pressed.contains(&KeyCode::Backspace) {
        match go_back(wizard) {
            Some(screen) => ctx.state.screen = screen,
            _ => {}
        }
    }

    ctx.state.keys_pressed.clear();
}

fn move_cursor(wizard: &mut ModelSelectWizard, delta: i32) {
    let list_len = match wizard.step {
        WizardStep::SelectModel => wizard.models.len(),
        WizardStep::SelectRunMode => wizard.run_modes.len(),
        WizardStep::SelectDataset => wizard.datasets.len(),
        WizardStep::Confirm => 0,
    };

    if list_len == 0 {
        return;
    }

    let current = wizard.cursor as i32;
    let new_cursor = (current + delta).clamp(0, (list_len - 1) as i32) as usize;
    wizard.cursor = new_cursor;
}

fn go_forward(wizard: &mut ModelSelectWizard) -> Option<ScreenState> {
    match wizard.step {
        WizardStep::SelectModel => {
            wizard.selected_model = Some(wizard.cursor);
            wizard.step = WizardStep::SelectRunMode;
            wizard.cursor = 0;
            None
        }
        WizardStep::SelectRunMode => {
            wizard.selected_run_mode = Some(wizard.cursor);
            wizard.step = WizardStep::SelectDataset;
            wizard.cursor = 0;
            None
        }
        WizardStep::SelectDataset => {
            wizard.selected_dataset = Some(wizard.cursor);
            wizard.step = WizardStep::Confirm;
            wizard.cursor = 0;
            None
        }
        WizardStep::Confirm => {
            let mode = match wizard.selected_run_mode {
                Some(selected) => wizard.run_modes[selected],
                None => RunMode::default(),
            };

            // Create mocked metrics for the model run
            let metrics = create_mocked_metrics();

            let screen = ScreenState::ModelRun {
                mode,
                metrics,
                selected_metric: None,
            };
            Some(screen)
        }
    }
}

fn go_back(wizard: &mut ModelSelectWizard) -> Option<ScreenState> {
    match wizard.step {
        WizardStep::SelectModel => {
            return Some(ScreenState::Home);
        }
        WizardStep::SelectRunMode => {
            wizard.step = WizardStep::SelectModel;
            wizard.selected_run_mode = None;
            wizard.cursor = wizard.selected_model.unwrap_or(0);
        }
        WizardStep::SelectDataset => {
            wizard.step = WizardStep::SelectRunMode;
            wizard.selected_dataset = None;
            wizard.cursor = wizard.selected_run_mode.unwrap_or(0);
        }
        WizardStep::Confirm => {
            wizard.step = WizardStep::SelectDataset;
            wizard.cursor = wizard.selected_dataset.unwrap_or(0);
        }
    }

    None
}

fn create_mocked_metrics() -> HashMap<crate::tui::types::MetricTag, crate::tui::types::Metric> {
    use crate::tui::types::{Datapoint, Metric, MetricScalar, MetricSeries, MetricTag};
    use ratatui::widgets::GraphType;

    let mut epochs_metric = MetricScalar::new(5);
    epochs_metric.format_str = Some("Epoch {} of {}");

    let mut loss_metric = MetricSeries::default();
    loss_metric.datapoints = (0..100)
        .map(|i| {
            let x = i as u32;
            let y = (1.0 / (1.0 + x as f32 * 0.05)) + (x as f32 * 0.13).sin() * 0.02;
            Datapoint::new(x, y)
        })
        .collect::<Vec<_>>();
    loss_metric.graph = GraphType::Line;
    loss_metric.format_str = Some("Loss {.3}");

    let mut metrics = HashMap::new();
    metrics.insert(MetricTag::Usize("epochs"), Metric::Usize(epochs_metric));
    metrics.insert(MetricTag::F32Series("loss"), Metric::F32Series(loss_metric));

    metrics
}
