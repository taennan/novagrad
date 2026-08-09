use crate::tui::types::AppSystemContext;
use crossterm::{ExecutableCommand, terminal};
use std::io;

pub fn run(ctx: AppSystemContext) {
    if ctx.state.should_set_title {
        let title_set_command = terminal::SetTitle(&ctx.state.title);

        let mut stdout = io::stdout();
        let title_set_result = stdout.execute(title_set_command);
        if title_set_result.is_ok() {
            ctx.state.should_set_title = false;
        }
    }
}
