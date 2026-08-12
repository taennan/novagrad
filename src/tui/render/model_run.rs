use crate::tui::{
    types::{Metric, MetricSeries, MetricTag, ScreenState},
    workers::logger::LogWorkerState,
};
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    symbols::Marker,
    text::Line,
    widgets::{Axis, Block, Chart, Dataset, GraphType, Paragraph},
};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

// Helper trait to convert metric values to f64
trait ToF64 {
    fn to_f64(&self) -> f64;
}

impl ToF64 for f32 {
    fn to_f64(&self) -> f64 {
        *self as f64
    }
}

impl ToF64 for usize {
    fn to_f64(&self) -> f64 {
        *self as f64
    }
}

pub fn render(frame: &mut Frame, screen: &ScreenState, log_state: Arc<Mutex<LogWorkerState>>) {
    match screen {
        ScreenState::ModelRun {
            mode,
            metrics,
            selected_metric,
            ..  // mode is captured from outer scope
        } => {
            let [header_layout, body_layout] = Layout::default()
                .direction(Direction::Vertical)
                .constraints(vec![Constraint::Length(1), Constraint::Fill(1)])
                .areas(frame.area());

            let header = Paragraph::new(format!("MODEL RUN — {}", String::from(mode)))
                .bg(Color::DarkGray)
                .fg(Color::White)
                .bold();
            frame.render_widget(header, header_layout);

            let [chart_layout, side_layout] = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(vec![Constraint::Fill(3), Constraint::Length(35)])
                .areas(body_layout);

            render_chart_and_controls(frame, chart_layout, metrics, selected_metric);
            render_side_panels(frame, side_layout, metrics, selected_metric, log_state);
        }
        _ => {}
    }
}

fn render_chart_and_controls(
    frame: &mut Frame,
    area: Rect,
    metrics: &HashMap<MetricTag, Metric>,
    selected_metric: &Option<MetricTag>,
) {
    let [controls_layout, chart_layout] = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Length(1), Constraint::Fill(1)])
        .areas(area);

    // Render controls strip (pause/start/save)
    let controls = Paragraph::new(" [SPACE] Pause/Resume  [S] Save  [Q] Quit ")
        .style(Style::default().fg(Color::Gray))
        .block(Block::bordered());
    frame.render_widget(controls, controls_layout);

    // Render chart if a metric is selected and it's a timeseries
    if let Some(metric_tag) = selected_metric {
        if let Some(metric) = metrics.get(metric_tag) {
            match metric {
                Metric::F32Series(series) => {
                    render_line_chart(frame, chart_layout, series, metric_tag.label());
                }
                Metric::UsizeSeries(series) => {
                    render_line_chart(frame, chart_layout, series, metric_tag.label());
                }
                Metric::Usize(_) => {
                    // Scalar metrics don't get displayed in the chart area
                    let placeholder =
                        Paragraph::new("Scalar metric (no chart)").block(Block::bordered());
                    frame.render_widget(placeholder, chart_layout);
                }
            }
        }
    } else {
        let placeholder = Paragraph::new("No metric selected").block(Block::bordered());
        frame.render_widget(placeholder, chart_layout);
    }
}

fn render_line_chart<T>(
    frame: &mut Frame,
    area: Rect,
    metric: &MetricSeries<T>,
    label: &'static str,
) where
    T: Clone + ToF64,
{
    if metric.datapoints.is_empty() {
        let placeholder =
            Paragraph::new("No data points").block(Block::bordered().title(format!(" {} ", label)));
        frame.render_widget(placeholder, area);
        return;
    }

    let x_min = metric
        .datapoints
        .iter()
        .map(|d| d.timestamp as f64)
        .fold(f64::INFINITY, f64::min);
    let x_max = metric
        .datapoints
        .iter()
        .map(|d| d.timestamp as f64)
        .fold(f64::NEG_INFINITY, f64::max);
    let y_min = metric
        .datapoints
        .iter()
        .map(|d| d.value.clone().to_f64())
        .fold(f64::INFINITY, f64::min);
    let y_max = metric
        .datapoints
        .iter()
        .map(|d| d.value.clone().to_f64())
        .fold(f64::NEG_INFINITY, f64::max);

    let datapoints = metric
        .datapoints
        .iter()
        .map(|d| (d.timestamp as f64, d.value.clone().to_f64()))
        .collect::<Vec<_>>();
    let chart = Chart::new(vec![
        Dataset::default()
            .name(label)
            .data(&datapoints)
            .marker(Marker::Braille)
            .graph_type(GraphType::Line)
            .style(Style::default().cyan()),
    ])
    .block(Block::bordered().title(format!(" {} ", label)))
    .x_axis(
        Axis::default()
            .title("Step")
            .style(Style::default().white())
            .bounds([x_min, x_max])
            .labels([format!("{x_min:.0}"), format!("{x_max:.0}")]),
    )
    .y_axis(
        Axis::default()
            .title("Value")
            .style(Style::default().white())
            .bounds([y_min, y_max])
            .labels([format!("{y_min:.3}"), format!("{y_max:.3}")]),
    );

    frame.render_widget(chart, area);
}

fn render_side_panels(
    frame: &mut Frame,
    area: Rect,
    metrics: &HashMap<MetricTag, Metric>,
    selected_metric: &Option<MetricTag>,
    log_state: Arc<Mutex<LogWorkerState>>,
) {
    let [metrics_layout, logs_layout] = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Fill(1), Constraint::Fill(1)])
        .areas(area);

    render_metrics_panel(frame, metrics_layout, metrics, selected_metric);
    render_logs_panel(frame, logs_layout, log_state);
}

fn render_metrics_panel(
    frame: &mut Frame,
    area: Rect,
    metrics: &HashMap<MetricTag, Metric>,
    selected_metric: &Option<MetricTag>,
) {
    let block = Block::bordered().title(" Metrics ");
    let inner = block.inner(area);
    frame.render_widget(block, area);

    if metrics.is_empty() {
        let msg = Paragraph::new("No metrics available");
        frame.render_widget(msg, inner);
        return;
    }

    // Collect and sort metric tags alphabetically
    let mut sorted_metric_tags: Vec<&MetricTag> = metrics.keys().collect();
    sorted_metric_tags.sort_by(|a, b| a.label().cmp(b.label()));

    let lines: Vec<Line> = sorted_metric_tags
        .iter()
        .map(|tag| {
            let is_selected = selected_metric.as_ref() == Some(tag);
            let label = metrics
                .get(tag)
                .map_or(tag.label(), |m| m.format_str().unwrap_or(tag.label()));
            let style = if is_selected {
                Style::default().fg(Color::Black).bg(Color::Cyan).bold()
            } else {
                Style::default().fg(Color::White)
            };
            Line::styled(format!(" > {} ", label), style)
        })
        .collect();

    frame.render_widget(Paragraph::new(lines), inner);
}

fn render_logs_panel(frame: &mut Frame, area: Rect, log_state: Arc<Mutex<LogWorkerState>>) {
    let block = Block::bordered().title(" Logs ");
    let inner = block.inner(area);
    frame.render_widget(block, area);

    let log_state = log_state.lock().unwrap();
    if !log_state.logs().is_empty() {
        let mut text = String::new();
        for log in log_state.logs().iter() {
            text.push_str(log);
        }

        let para = Paragraph::new(text);
        frame.render_widget(para, inner);
    } else {
        // Placeholder for logs - user will add logic later
        let placeholder = Paragraph::new("[Logs will appear here]");
        frame.render_widget(placeholder, inner);
    }
}
