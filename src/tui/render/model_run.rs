use crate::tui::{ChartKind, ModelRunState};
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    symbols::Marker,
    text::{Line, Text},
    widgets::{Axis, Bar, BarChart, BarGroup, Block, Chart, Dataset, GraphType, Paragraph},
};

pub fn render(frame: &mut Frame, training: bool, run: &ModelRunState) {
    let [header_layout, body_layout] = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Length(1), Constraint::Fill(1)])
        .areas(frame.area());

    let mode_label = if training { "TRAINING" } else { "TESTING" };
    let header = Paragraph::new(format!("MODEL RUN — {mode_label}"))
        .bg(Color::DarkGray)
        .fg(Color::White)
        .bold();
    frame.render_widget(header, header_layout);

    let [chart_layout, side_layout] = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![Constraint::Fill(3), Constraint::Length(30)])
        .areas(body_layout);

    render_metric_tabs_and_chart(frame, chart_layout, run);
    render_side_panel(frame, side_layout, run);
}

fn render_metric_tabs_and_chart(frame: &mut Frame, area: Rect, run: &ModelRunState) {
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

    match metric.chart_kind {
        ChartKind::Line => render_line_chart(frame, chart_layout, metric),
        ChartKind::Bar => render_bar_chart(frame, chart_layout, metric),
    }
}

fn render_line_chart(frame: &mut Frame, area: Rect, metric: &crate::tui::MetricSeries) {
    let x_min = metric
        .line_data
        .iter()
        .map(|(x, _)| *x)
        .fold(f64::INFINITY, f64::min);
    let x_max = metric
        .line_data
        .iter()
        .map(|(x, _)| *x)
        .fold(f64::NEG_INFINITY, f64::max);
    let y_min = metric
        .line_data
        .iter()
        .map(|(_, y)| *y)
        .fold(f64::INFINITY, f64::min);
    let y_max = metric
        .line_data
        .iter()
        .map(|(_, y)| *y)
        .fold(f64::NEG_INFINITY, f64::max);

    let chart = Chart::new(vec![
        Dataset::default()
            .name(metric.name)
            .data(&metric.line_data)
            .marker(Marker::Braille)
            .graph_type(GraphType::Line)
            .style(Style::default().cyan()),
    ])
    .block(Block::bordered().title(format!(" {} ", metric.name)))
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

fn render_bar_chart(frame: &mut Frame, area: Rect, metric: &crate::tui::MetricSeries) {
    let bars: Vec<Bar> = metric
        .bar_data
        .iter()
        .map(|(label, value)| {
            Bar::default()
                .value(*value)
                .label(Line::from(*label))
                .text_value(value.to_string())
                .style(Style::default().fg(Color::Magenta))
        })
        .collect();

    let chart = BarChart::default()
        .block(Block::bordered().title(format!(" {} ", metric.name)))
        .data(BarGroup::default().bars(&bars))
        .bar_width(7)
        .bar_gap(2);

    frame.render_widget(chart, area);
}

fn render_side_panel(frame: &mut Frame, area: Rect, run: &ModelRunState) {
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

/// Mock run state so the screen has something to render before the real
/// training/testing backend is wired up.
pub fn mock_model_run_state() -> ModelRunState {
    let loss_data: Vec<(f64, f64)> = (0..100)
        .map(|i| {
            let x = i as f64;
            let y = (1.0 / (1.0 + x * 0.05)) + (x * 0.13).sin() * 0.02;
            (x, y)
        })
        .collect();

    let accuracy_data: Vec<(f64, f64)> = (0..100)
        .map(|i| {
            let x = i as f64;
            let y = (1.0 - (1.0 / (1.0 + x * 0.08))).min(0.99);
            (x, y)
        })
        .collect();

    let lr_bars = vec![
        ("epoch 1", 100),
        ("epoch 2", 80),
        ("epoch 3", 64),
        ("epoch 4", 51),
        ("epoch 5", 41),
    ];

    ModelRunState {
        metrics: vec![
            crate::tui::MetricSeries {
                name: "Loss",
                chart_kind: ChartKind::Line,
                line_data: loss_data,
                bar_data: vec![],
            },
            crate::tui::MetricSeries {
                name: "Accuracy",
                chart_kind: ChartKind::Line,
                line_data: accuracy_data,
                bar_data: vec![],
            },
            crate::tui::MetricSeries {
                name: "Learning Rate",
                chart_kind: ChartKind::Bar,
                line_data: vec![],
                bar_data: lr_bars,
            },
        ],
        selected_metric: 0,
        stats: crate::tui::SystemStats {
            cpu_percent: 42.3,
            mem_used_mb: 2_150.0,
            mem_total_mb: 8_192.0,
            elapsed_seconds: 754,
            current_epoch: 5,
            total_epochs: 20,
        },
    }
}
