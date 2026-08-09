use crate::tui::types::{AppSystem, ScreenState, WizardStep};
use crossterm::event::KeyCode;

/// Handles exiting the model select wizard and returning to the home screen
/// when the user presses Escape while on the first wizard step.
pub fn new() -> AppSystem {
    Box::new(|state| {
        let ScreenState::ModelSelect { wizard } = &mut state.screen else {
            return;
        };

        // Only allow exiting from the first step
        if wizard.step == WizardStep::SelectModel
            && state.keys_pressed.contains(&KeyCode::Backspace)
        {
            state.screen = ScreenState::Home;
            state.keys_pressed.clear();
        }
    })
}
