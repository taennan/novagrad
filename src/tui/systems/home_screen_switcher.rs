use crate::tui::types::{AppSystem, ModelSelectWizard, ScreenState};

pub fn new() -> AppSystem {
    Box::new(|state| {
        let is_on_home_screen = matches!(state.screen, ScreenState::Home);
        let is_key_pressed = !state.keys_pressed.is_empty();

        if is_on_home_screen && is_key_pressed {
            state.screen = ScreenState::ModelSelect {
                wizard: ModelSelectWizard::default(),
            };
            state.keys_pressed.clear();
        }
    })
}
