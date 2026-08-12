use crate::tui::types::{AppEvent, AppSystemContext};

pub fn run(ctx: AppSystemContext) {
    match ctx.event {
        AppEvent::KeyPress(keycode) => {
            ctx.state.keys_pressed.insert(*keycode);
            //ctx.logger.log(format!("Pressed {keycode}"));
        }
        _ => {}
    }
}
