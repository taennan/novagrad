use crate::tui::AppSystem;
use crossterm::{ExecutableCommand, terminal};
use std::io;

pub fn new() -> AppSystem {
    Box::new(|state| {
        if state.should_set_title {
            let title_set_command = terminal::SetTitle(&state.title);

            let mut stdout = io::stdout();
            let title_set_result = stdout.execute(title_set_command);
            if title_set_result.is_ok() {
                state.should_set_title = false;
            }
        }
    })
}
