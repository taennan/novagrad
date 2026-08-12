use std::collections::HashMap;

use crate::tui::types::{Metric, MetricSeries, MetricTag, RunMode, ScreenState};
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    symbols::Marker,
    text::{Line, Text},
    widgets::{Axis, Bar, BarChart, BarGroup, Block, Chart, Dataset, GraphType, Paragraph},
};

pub fn render(frame: &mut Frame, mode: RunMode, screen: &ScreenState) {
    match screen {
        ScreenState::ModelRun {
            mode,
            metrics,
            selected_metric,
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
                .constraints(vec![Constraint::Fill(3), Constraint::Length(30)])
                .areas(body_layout);

            let default_metric_tag = metrics.keys().find(|_| true).unwrap();
            let metric_tag = selected_metric.as_ref().unwrap_or(default_metric_tag);

            render_metric_tabs_and_chart(frame, chart_layout, metrics, selected_metric);
            render_side_panel(frame, side_layout, run);
        }
        _ => {}
    }
}

fn render_metric_tabs_and_chart(
    frame: &mut Frame,
    area: Rect,
    metrics: &HashMap<MetricTag, Metric>,
    selected_metric: &MetricTag,
) {
    if metrics.is_empty() {
        return;
    }

    let [tabs_layout, chart_layout] = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Length(1), Constraint::Fill(1)])
        .areas(area);

    // Tab strip: one label per metric, current selection highlighted.
    let mut spans = Vec::new();
    for (index, metric) in run.metrics.iter().enumerate() {
        if index > 0 {
            spans.push(ratatui::text::Span::raw("  "));
        }
        let style = if index == run.selected_metric {
            Style::default().fg(Color::Black).bg(Color::Cyan).bold()
        } else {
            Style::default().fg(Color::DarkGray)
        };
        spans.push(ratatui::text::Span::styled(
            format!(" [{}] {} ", index + 1, metric.name),
            style,
        ));
    }
    frame.render_widget(Paragraph::new(Line::from(spans)), tabs_layout);

    let Some(metric) = run.metrics.get(run.selected_metric) else {
        return;
    };

    match metric.graph {
        GraphType::Line => render_line_chart(frame, chart_layout, metric),
        //GraphType::Bar => render_bar_chart(frame, chart_layout, metric),
        _ => unimplemented!("Other graph type not implemented"),
    }
}

fn render_line_chart<T>(
    frame: &mut Frame,
    area: Rect,
    metric: &MetricSeries<T>,
    label: &'static str,
) where
    T: Ord + Clone + Into<f64>,
{
    let x_min = metric
        .datapoints
        .iter()
        .map(|d| d.timestamp.clone().into())
        .fold(f64::INFINITY, f64::min);
    let x_max = metric
        .datapoints
        .iter()
        .map(|d| d.timestamp.into())
        .fold(f64::NEG_INFINITY, f64::max);
    let y_min = metric
        .datapoints
        .iter()
        .map(|d| d.value.into())
        .fold(f64::INFINITY, f64::min);
    let y_max = metric
        .datapoints
        .iter()
        .map(|d| d.value.into())
        .fold(f64::NEG_INFINITY, f64::max);

    let datapoints = metric
        .datapoints
        .iter()
        .map(|d| (d.timestamp.into(), d.value.into()))
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

fn render_side_panel(frame: &mut Frame, area: Rect, metrics: &HashMap<MetricTag, Metric>) {
    let block = Block::bordered().title(" Run Info ");
    let inner = block.inner(area);
    frame.render_widget(block, area);

    let stats = &run.stats;
    let mem_percent = if stats.mem_total_mb > 0.0 {
        (stats.mem_used_mb / stats.mem_total_mb) * 100.0
    } else {
        0.0
    };

    let lines = Text::from(vec![
        Line::from(format!(
            "Epoch:  {}/{}",
            stats.current_epoch, stats.total_epochs
        )),
        Line::from(format!(
            "Elapsed: {:02}:{:02}:{:02}",
            stats.elapsed_seconds / 3600,
            (stats.elapsed_seconds % 3600) / 60,
            stats.elapsed_seconds % 60
        )),
        Line::default(),
        Line::from(format!("CPU: {:.1}%", stats.cpu_percent)),
        Line::from(format!(
            "Mem: {:.0} / {:.0} MB ({:.0}%)",
            stats.mem_used_mb, stats.mem_total_mb, mem_percent
        )),
    ]);

    frame.render_widget(Paragraph::new(lines), inner);
}
