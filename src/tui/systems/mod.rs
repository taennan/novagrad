use crate::tui::types::AppSystem;

mod app_quitter;
mod home_screen_switcher;
mod input_receiver;
mod metric_modifier;
mod model_run_input;
mod title_setter;
mod wizard_input;

pub fn ordered() -> [AppSystem; 7] {
    [
        title_setter::run,
        input_receiver::run,
        app_quitter::run,
        home_screen_switcher::run,
        wizard_input::run,
        metric_modifier::run,
        model_run_input::run,
    ]
}
