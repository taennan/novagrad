use crate::{utils::events::AppEvent, utils::system::AppSystemContext};
use crossterm::event::KeyCode;

pub fn run(ctx: AppSystemContext) {
    if ctx.state.keys_pressed.contains(&KeyCode::Esc) {
        ctx.app_sender
            .send(AppEvent::Quit)
            .expect("Failed to quit gracefully, will force quit");
        ctx.state.keys_pressed.clear();
    }
}
