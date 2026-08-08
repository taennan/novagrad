use crate::models::Model;
use crossterm::event::KeyCode;
use std::collections::HashSet;

#[derive(Default)]
pub struct AppState {
    pub title: String,
    pub should_set_title: bool,
    pub keys_pressed: HashSet<KeyCode>,
    pub screen: ScreenState,
    pub should_exit: bool,
}

pub enum ScreenState {
    Home,
    ModelSelect {
        models: Vec<&'static str>,
    },
    ModelRun {
        training: bool,
        model: Box<dyn Model>,
    },
}

impl Default for ScreenState {
    fn default() -> Self {
        Self::Home
    }
}

pub type AppSystem = Box<dyn Fn(&mut AppState)>;
