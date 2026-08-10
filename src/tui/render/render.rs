use crate::tui::{
    render::{home, model_run, model_select},
    types::{AppState, ScreenState},
};

pub fn render(frame: &mut ratatui::Frame, state: &AppState) {
    match &state.screen {
        ScreenState::Home => home::render(frame),
        ScreenState::ModelSelect { wizard } => model_select::render(frame, wizard),
        ScreenState::ModelRun { mode, run, .. } => model_run::render(frame, mode.clone(), run),
    };
}
