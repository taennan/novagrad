use crate::tui::{AppSystem, types::ScreenState};
use crossterm::event::KeyCode;

/// Handles user input during model training/testing:
/// - Left/Right arrows cycle through metrics
/// - Tab also cycles through metrics (next)
/// - Escape returns to the model select screen
pub fn new() -> AppSystem {
    Box::new(|state| {
        let ScreenState::ModelRun { run, .. } = &mut state.screen else {
            return;
        };

        let metrics_count = run.metrics.len();
        if metrics_count == 0 {
            return;
        }

        if state.keys_pressed.contains(&KeyCode::Left) {
            run.selected_metric = (run.selected_metric + metrics_count - 1) % metrics_count;
            state.keys_pressed.clear();
        } else if state.keys_pressed.contains(&KeyCode::Right)
            || state.keys_pressed.contains(&KeyCode::Tab)
        {
            run.selected_metric = (run.selected_metric + 1) % metrics_count;
            state.keys_pressed.clear();
        }
    })
}
