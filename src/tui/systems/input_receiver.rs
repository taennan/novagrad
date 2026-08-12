use crate::tui::types::{AppEvent, AppSystemContext};

pub fn run(ctx: AppSystemContext) {
    match ctx.event {
        AppEvent::KeyPress(keycode) => {
            //println!("Adding {keycode}");
            ctx.state.keys_pressed.insert(*keycode);
        }
        _ => {}
    }
}
