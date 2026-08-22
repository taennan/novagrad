use ratatui::widgets::GraphType;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Clone, Debug)]
pub enum Metric {
    Usize(MetricScalar<usize>),
    F32(MetricScalar<f32>),
    UsizeSeries(MetricSeries<usize>),
    F32Series(MetricSeries<f32>),
}

impl Metric {
    pub fn format_str(&self) -> Option<&str> {
        match self {
            Self::Usize(s) => s.format_str.as_deref(),
            Self::F32(s) => s.format_str.as_deref(),
            Self::F32Series(s) => s.format_str.as_deref(),
            Self::UsizeSeries(s) => s.format_str.as_deref(),
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct MetricScalar<T> {
    pub value: T,
    pub format_str: Option<String>,
}

impl<T> MetricScalar<T> {
    pub fn new(value: T) -> Self {
        Self {
            value,
            format_str: None,
        }
    }

    pub fn new_formatted<S>(value: T, format_str: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            value,
            format_str: Some(format_str.into()),
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct MetricSeries<T> {
    pub datapoints: Vec<Datapoint<T>>,
    pub graph: GraphType,
    pub format_str: Option<&'static str>,
}

impl<T> MetricSeries<T> {
    pub fn new(datapoints: Vec<Datapoint<T>>, graph: GraphType) -> Self {
        Self {
            datapoints,
            graph,
            format_str: None,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Datapoint<T> {
    pub timestamp: u32,
    pub value: T,
}

impl<T> Datapoint<T> {
    pub fn now(value: T) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        Self {
            timestamp: timestamp as u32,
            value,
        }
    }

    pub fn new(timestamp: u32, value: T) -> Self {
        Self { timestamp, value }
    }
}
