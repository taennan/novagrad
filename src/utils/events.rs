use crate::{
    models::{Models, datasets::Datasets},
    utils::metrics::{Metric, MetricTag},
};
use crossterm::event::KeyCode;

#[derive(Debug)]
pub enum AppEvent {
    Quit,
    Tick,
    SetTitle(String),
    KeyPress(KeyCode),
    MetricModified(MetricTag, Metric),
    MetricDeleted(MetricTag),
}

pub enum ModelRunnerEvent {
    Start(Models),
    Pause,
    Resume,
    Stop,
    SetDataset(Datasets),
}
