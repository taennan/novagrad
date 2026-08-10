use crate::tui::types::{AppEvent, AppSystemContext};
use crossterm::{ExecutableCommand, terminal};
use std::io;

pub fn run(ctx: AppSystemContext) {
    match &ctx.event {
        AppEvent::SetTitle(new_title) => {
            let title_set_command = terminal::SetTitle(new_title);

            let mut stdout = io::stdout();
            let _ = stdout.execute(title_set_command);
        }
        _ => {}
    }
}
