use crate::tui::types::{AppEvent, AppState, ModelRunnerEvent};
use std::sync::mpsc::Sender;

pub type AppSystem = fn(AppSystemContext);

pub struct AppSystemContext<'a> {
    pub state: &'a mut AppState,
    pub app_sender: &'a Sender<AppEvent>,
    pub model_runner_sender: &'a Sender<ModelRunnerEvent>,
    pub event: &'a AppEvent,
}
