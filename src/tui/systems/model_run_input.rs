use crate::{
    tui::types::{AppSystemContext, ScreenState},
    utils::metrics::MetricTag,
};
use crossterm::event::KeyCode;

/// Handles user input during model training/testing:
/// - Up/Down arrows cycle through metrics
/// - Escape returns to the model select screen
pub fn run(ctx: AppSystemContext) {
    let ScreenState::ModelRun {
        metrics,
        selected_metric,
        ..
    } = &mut ctx.state.screen
    else {
        return;
    };

    if metrics.is_empty() {
        return;
    }

    // Collect and sort metric tags alphabetically
    let mut sorted_metrics: Vec<&MetricTag> = metrics.keys().collect();
    sorted_metrics.sort_by(|a, b| a.label().cmp(b.label()));

    let mod_amount = if ctx.state.keys_pressed.contains(&KeyCode::Up) {
        1isize
    } else if ctx.state.keys_pressed.contains(&KeyCode::Down) {
        -1
    } else {
        0
    };
    let current_index: isize = selected_metric.as_ref().map_or(0, |m| {
        sorted_metrics
            .iter()
            .position(|sm| m == *sm)
            .unwrap_or_default() as isize
    });
    let modded_index = current_index + mod_amount;

    let max_index = sorted_metrics.len() as isize - 1;
    let new_index = if modded_index < 0 {
        max_index
    } else if modded_index > max_index {
        0
    } else {
        modded_index
    } as usize;

    //println!("Selecting metric {new_index}");
    *selected_metric = Some(sorted_metrics[new_index].clone());
}
