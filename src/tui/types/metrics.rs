use ratatui::widgets::GraphType;
use std::hash::Hash;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum MetricTag {
    Usize(&'static str),
    F32Series(&'static str),
}

impl MetricTag {
    pub fn label(&self) -> &'static str {
        match self {
            Self::Usize(l) => l,
            Self::F32Series(l) => l,
        }
    }
}

#[derive(Debug)]
pub enum Metric {
    UsizeSeries(MetricSeries<usize>),
    F32Series(MetricSeries<f32>),
    Usize(MetricScalar<usize>),
}

impl Metric {
    pub fn format_str(&self) -> Option<&'static str> {
        match self {
            Self::F32Series(s) => s.format_str,
            Self::UsizeSeries(s) => s.format_str,
            Self::Usize(s) => s.format_str,
        }
    }
}

#[derive(Debug, Default)]
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
}

#[derive(Debug, Default)]
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

#[derive(Debug)]
pub struct Datapoint<T> {
    pub timestamp: u32,
    pub value: T,
}

impl<T> Datapoint<T> {
    pub fn new(timestamp: u32, value: T) -> Self {
        Self { timestamp, value }
    }
}
