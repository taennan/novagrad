use crate::{
    datasets::Datasets,
    models::Models,
    utils::{Primitive, metrics::Metric, state::RunMode},
};
use crossterm::event::KeyCode;

#[derive(Debug)]
pub enum AppEvent {
    Quit,
    Tick,
    SetTitle(String),
    KeyPress(KeyCode),
    MetricAdded(&'static str, Metric),
    MetricModified(&'static str, Primitive),
    MetricDeleted(&'static str),
}

pub enum ModelRunnerEvent {
    Start(Models, Datasets, RunMode),
    Pause,
    Resume,
    Stop,
}
