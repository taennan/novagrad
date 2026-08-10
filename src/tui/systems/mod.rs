use crate::tui::types::AppSystem;

mod app_quitter;
mod home_screen_switcher;
mod input_receiver;
mod model_run_input;
mod title_setter;
mod wizard_input;

pub fn ordered() -> [AppSystem; 6] {
    [
        title_setter::run,
        input_receiver::run,
        app_quitter::run,
        home_screen_switcher::run,
        wizard_input::run,
        model_run_input::run,
    ]
}
