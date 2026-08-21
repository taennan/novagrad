use ratatui::widgets::GraphType;
use std::hash::Hash;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum MetricTag {
    Usize(&'static str),
    F32(&'static str),
    F32Series(&'static str),
}

impl MetricTag {
    pub fn label(&self) -> &'static str {
        match self {
            Self::Usize(l) => l,
            Self::F32(l) => l,
            Self::F32Series(l) => l,
        }
    }
}

#[derive(Clone, Debug)]
pub enum Metric {
    Usize(MetricScalar<usize>),
    F32(MetricScalar<f32>),
    UsizeSeries(MetricSeries<usize>),
    F32Series(MetricSeries<f32>),
}

impl Metric {
    pub fn format_str(&self) -> Option<&'static str> {
        match self {
            Self::Usize(s) => s.format_str,
            Self::F32(s) => s.format_str,
            Self::F32Series(s) => s.format_str,
            Self::UsizeSeries(s) => s.format_str,
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct MetricScalar<T> {
    pub value: T,
    pub format_str: Option<&'static str>,
}

impl<T> MetricScalar<T> {
    pub fn new(value: T) -> Self {
        Self {
            value,
            format_str: None,
        }
    }

    pub fn new_formatted(value: T, format_str: &'static str) -> Self {
        Self {
            value,
            format_str: Some(format_str),
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
    pub fn new(timestamp: u32, value: T) -> Self {
        Self { timestamp, value }
    }
}
