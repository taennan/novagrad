use crate::tui::{AppState, ScreenState};
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    prelude::Line,
    style::{Color, Stylize},
    text::Text,
    widgets::{Block, Paragraph},
};

pub fn render(frame: &mut Frame, state: &AppState) {
    match &state.screen {
        ScreenState::Home => render_home_screen(frame, state),
        ScreenState::ModelSelect { models } => render_model_select_screen(frame),
        ScreenState::ModelRun { training, model } => todo!(),
    };
}

fn render_home_screen(frame: &mut Frame, state: &AppState) {
    let center_block = Block::bordered().title_top(Line::from("v0.1.0").centered());
    let title = Paragraph::new(Text::from(state.title.clone()))
        .bold()
        .centered()
        .block(center_block);

    let [layout] = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Percentage(50)])
        .areas(frame.area());

    frame.render_widget(
        title,
        layout.centered(Constraint::Percentage(75), Constraint::Percentage(50)),
    );
}

fn render_model_select_screen(frame: &mut Frame) {
    let [title_layout, main_layout] = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Length(1), Constraint::Fill(1)])
        .areas(frame.area());

    let title = Paragraph::new("MODEL SELECT")
        .bg(Color::DarkGray)
        .fg(Color::White)
        .bold();

    let models = Paragraph::new("Sine\nTanh\nSomething");

    frame.render_widget(title, title_layout);
    frame.render_widget(models, main_layout);
}
