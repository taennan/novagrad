use crate::tui::types::{AppEvent, AppSystemContext};

pub fn run(ctx: AppSystemContext) {
    match ctx.event {
        AppEvent::KeyPress(keycode) => {
            ctx.state.keys_pressed.insert(*keycode);
        }
        AppEvent::KeyRelease(keycode) => {
            ctx.state.keys_pressed.remove(keycode);
        }
        _ => {}
    }
}
