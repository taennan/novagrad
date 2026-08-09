use crate::tui::types::AppSystem;

mod home_screen_switcher;
mod model_run_input;
mod title_setter;
mod wizard_input;

pub fn ordered() -> [AppSystem; 4] {
    [
        title_setter::run,
        home_screen_switcher::run,
        wizard_input::run,
        model_run_input::run,
    ]
}
