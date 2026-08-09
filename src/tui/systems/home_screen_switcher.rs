use crate::tui::types::{AppSystemContext, ModelSelectWizard, ScreenState};

pub fn run(ctx: AppSystemContext) {
    let is_on_home_screen = matches!(ctx.state.screen, ScreenState::Home);
    let is_key_pressed = !ctx.state.keys_pressed.is_empty();

    if is_on_home_screen && is_key_pressed {
        ctx.state.screen = ScreenState::ModelSelect {
            wizard: ModelSelectWizard::default(),
        };
        ctx.state.keys_pressed.clear();
    }
}
