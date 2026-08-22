use crate::{
    tui::{
        render::{home, model_run, model_select},
        workers::logger::LogWorkerState,
    },
    utils::state::{AppState, ScreenState},
};
use std::sync::{Arc, Mutex};

pub fn render(
    frame: &mut ratatui::Frame,
    app_state: &AppState,
    log_state: Arc<Mutex<LogWorkerState>>,
) {
    match &app_state.screen {
        ScreenState::Home => home::render(frame),
        ScreenState::ModelSelect { wizard } => model_select::render(frame, wizard),
        ScreenState::ModelRun { .. } => model_run::render(frame, &app_state.screen, log_state),
    };
}
