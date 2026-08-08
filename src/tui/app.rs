use crate::tui::{
    AppState, AppSystem,
    render::render,
    systems::{home_screen_switcher, model_run_input, title_setter, wizard_exit, wizard_input},
};
use crossterm::event::{Event, KeyCode};
use ratatui::DefaultTerminal;

pub fn run() {
    let mut state = AppState::default();
    state.title = "Novagrad".into();
    state.should_set_title = true;

    let systems = [
        title_setter::new(),
        home_screen_switcher::new(),
        wizard_input::new(),
        wizard_exit::new(),
        model_run_input::new(),
    ];

    ratatui::run(|terminal| mainloop(terminal, &systems, &mut state));
}

fn mainloop(terminal: &mut DefaultTerminal, systems: &[AppSystem], state: &mut AppState) {
    loop {
        if state.should_exit {
            break;
        }
        for system in systems {
            (system)(state);
        }

        let _ = terminal.draw(|frame| render(frame, state));

        match crossterm::event::read() {
            Ok(Event::Key(key_event)) if key_event.code == KeyCode::Esc => {
                state.should_exit = true;
            }
            Ok(Event::Key(key_event)) if key_event.is_press() => {
                state.keys_pressed.insert(key_event.code);
            }
            Ok(Event::Key(key_event)) if key_event.is_release() => {
                state.keys_pressed.remove(&key_event.code);
            }
            _ => (),
        }
    }
}
