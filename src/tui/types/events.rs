use crate::models::{Models, datasets::Datasets};
use crossterm::event::KeyCode;

#[derive(Debug)]
pub enum AppEvent {
    Quit,
    Tick,
    SetTitle(String),
    KeyPress(KeyCode),
    KeyRelease(KeyCode),
}

pub enum ModelRunnerEvent {
    Start(Models),
    Pause,
    Resume,
    Stop,
    SetDataset(Datasets),
}
