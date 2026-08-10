use crate::tui::types::AppState;
use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    prelude::Line,
    style::Style,
    widgets::Block,
};
use tui_big_text::{BigText, PixelSize};

pub fn render(frame: &mut Frame) {
    let center_block = Block::bordered().title_top(Line::from("v0.1.0").centered());

    let [layout] = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Percentage(50)])
        .areas(frame.area());

    let outer_area = layout.centered(Constraint::Percentage(75), Constraint::Percentage(50));
    let inner_area = center_block.inner(outer_area);

    frame.render_widget(center_block, outer_area);

    // BigText has a fixed height in terminal rows (8 for `Full`), so it
    // needs to be vertically centered by hand rather than stretched.
    let big_text_area = vertical_center(inner_area, 8);

    let big_title = BigText::builder()
        .pixel_size(PixelSize::Full)
        .style(Style::new().bold().cyan())
        .alignment(Alignment::Center)
        .lines(vec!["Novagrad".into()])
        .build();

    frame.render_widget(big_title, big_text_area)
}

fn vertical_center(area: Rect, height: u16) -> Rect {
    let height = height.min(area.height);
    let margin = (area.height - height) / 2;
    Rect {
        x: area.x,
        y: area.y + margin,
        width: area.width,
        height,
    }
}
