use crate::{tui::types::AppSystemContext, utils::events::AppEvent};

pub fn run(ctx: AppSystemContext) {
    match ctx.event {
        AppEvent::KeyPress(keycode) => {
            ctx.state.keys_pressed.insert(*keycode);
            //ctx.logger.log(format!("Pressed {keycode}"));
        }
        _ => {}
    }
}
