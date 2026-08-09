use crate::tui::types::AppState;

pub type AppSystem = Box<dyn Fn(&mut AppState)>;
