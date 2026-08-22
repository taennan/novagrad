use crate::utils::state::{ModelSelectWizard, RunMode, WizardStep};
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    prelude::Line,
    style::{Color, Modifier, Style, Stylize},
    text::{Span, Text},
    widgets::{Block, Paragraph},
};

pub fn render(frame: &mut Frame, wizard: &ModelSelectWizard) {
    let [title_layout, breadcrumb_layout, main_layout, hint_layout] = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Fill(1),
            Constraint::Length(1),
        ])
        .areas(frame.area());

    let title = Paragraph::new("MODEL SELECT")
        .bg(Color::DarkGray)
        .fg(Color::White)
        .bold();
    frame.render_widget(title, title_layout);

    render_breadcrumb(frame, breadcrumb_layout, wizard.step);

    match wizard.step {
        WizardStep::SelectModel => render_model_step(frame, main_layout, wizard),
        WizardStep::SelectRunMode => render_run_mode_step(frame, main_layout, wizard),
        WizardStep::SelectDataset => render_dataset_step(frame, main_layout, wizard),
        WizardStep::Confirm => render_confirm_step(frame, main_layout, wizard),
    }

    let hint = Paragraph::new("↑/↓ move   enter select   esc back").fg(Color::DarkGray);
    frame.render_widget(hint, hint_layout);
}

fn render_breadcrumb(frame: &mut Frame, area: Rect, step: WizardStep) {
    let steps = ["1. Model", "2. Mode", "3. Dataset", "4. Confirm"];
    let current = match step {
        WizardStep::SelectModel => 0,
        WizardStep::SelectRunMode => 1,
        WizardStep::SelectDataset => 2,
        WizardStep::Confirm => 3,
    };

    let mut spans = Vec::new();
    for (index, label) in steps.iter().enumerate() {
        if index > 0 {
            spans.push(Span::raw("  →  ").fg(Color::DarkGray));
        }
        let style = match index.cmp(&current) {
            std::cmp::Ordering::Less => Style::default().fg(Color::Green),
            std::cmp::Ordering::Equal => Style::default()
                .fg(Color::Black)
                .bg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
            std::cmp::Ordering::Greater => Style::default().fg(Color::DarkGray),
        };
        spans.push(Span::styled(format!(" {label} "), style));
    }

    frame.render_widget(Paragraph::new(Line::from(spans)), area);
}

fn render_model_step(frame: &mut Frame, area: Rect, wizard: &ModelSelectWizard) {
    let items: Vec<Line> = wizard
        .models
        .iter()
        .enumerate()
        .map(|(index, model)| {
            let (label, detail) = match &model.meta().checkpoint {
                Some(checkpoint) => (
                    format!("Existing model: {}", model.meta().name),
                    format!("resume from {:?}", checkpoint.file_name()),
                ),
                _ => (
                    format!("New model: {}", model.meta().name),
                    format!("train from scratch"),
                ),
            };
            list_line(index == wizard.cursor, &label, &detail)
        })
        .collect();

    let list = Paragraph::new(Text::from(items)).block(Block::bordered().title(" Choose a model "));
    frame.render_widget(list, area);
}

fn render_run_mode_step(frame: &mut Frame, area: Rect, wizard: &ModelSelectWizard) {
    let selected_model = wizard
        .selected_model
        .and_then(|index| wizard.models.get(index))
        .map(|model| model.meta().name)
        .unwrap_or("(none)");

    let items: Vec<Line> = wizard
        .run_modes
        .iter()
        .enumerate()
        .map(|(index, mode)| {
            let detail = match mode {
                RunMode::Train => "run training steps and update weights",
                RunMode::Test => "run inference only, no weight updates",
            };
            let mode_label = String::from(mode);
            list_line(index == wizard.cursor, &mode_label, detail)
        })
        .collect();

    let list = Paragraph::new(Text::from(items))
        .block(Block::bordered().title(format!(" Train or test \"{selected_model}\"? ")));
    frame.render_widget(list, area);
}

fn render_dataset_step(frame: &mut Frame, area: Rect, wizard: &ModelSelectWizard) {
    let items: Vec<Line> = wizard
        .datasets
        .iter()
        .enumerate()
        .map(|(index, dataset)| {
            let meta = dataset.meta();
            let detail = format!("{} · {} samples", meta.description, meta.samples);
            list_line(index == wizard.cursor, meta.name, &detail)
        })
        .collect();

    let list =
        Paragraph::new(Text::from(items)).block(Block::bordered().title(" Choose a dataset "));
    frame.render_widget(list, area);
}

fn render_confirm_step(frame: &mut Frame, area: Rect, wizard: &ModelSelectWizard) {
    let model = wizard
        .selected_model
        .and_then(|i| wizard.models.get(i))
        .map(|m| m.meta().name)
        .unwrap_or("(none)");
    let mode = wizard
        .selected_run_mode
        .and_then(|i| wizard.run_modes.get(i))
        .map(|m| String::from(m))
        .unwrap_or("(none)".to_string());
    let dataset = wizard
        .selected_dataset
        .and_then(|i| wizard.datasets.get(i))
        .map(|d| d.meta().name)
        .unwrap_or("(none)");

    let summary = Text::from(vec![
        Line::from(format!("Model:   {model}")),
        Line::from(format!("Mode:    {mode}")),
        Line::from(format!("Dataset: {dataset}")),
        Line::default(),
        Line::from("Press enter to start the run.").fg(Color::Green),
    ]);

    let block = Block::bordered().title(" Confirm ");
    frame.render_widget(Paragraph::new(summary).block(block), area);
}

/// Renders a single wizard list item, highlighted with a `>` cursor and
/// reversed style when active — the same look as `npm init`-style CLIs.
fn list_line<'a>(active: bool, label: &str, detail: &str) -> Line<'a> {
    let (marker, style) = if active {
        (
            "> ",
            Style::default().fg(Color::Black).bg(Color::Cyan).bold(),
        )
    } else {
        ("  ", Style::default().fg(Color::White))
    };

    Line::from(vec![
        Span::styled(marker, style),
        Span::styled(format!("{label}  "), style),
        Span::styled(format!("({detail})"), Style::default().fg(Color::DarkGray)),
    ])
}
