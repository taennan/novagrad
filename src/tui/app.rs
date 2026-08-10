use crate::tui::{
    render::render,
    systems,
    types::{AppEvent, AppState, AppSystemContext, ModelRunnerEvent},
    workers,
};
use std::sync::{Arc, Mutex, mpsc};

pub fn run() {
    let (app_sender, app_receiver) = mpsc::channel::<AppEvent>();
    let (model_runner_sender, model_runner_receiver) = mpsc::channel::<ModelRunnerEvent>();

    let state = Arc::new(Mutex::new(AppState::default()));

    workers::input::spawn(app_sender.clone());
    workers::ticker::spawn(app_sender.clone());
    workers::model_runner::spawn(state.clone(), model_runner_receiver);

    ratatui::run(|terminal| {
        let _ = app_sender.send(AppEvent::SetTitle("Novagrad".into()));

        while let Ok(event) = app_receiver.recv() {
            if matches!(event, AppEvent::Quit) {
                break;
            }

            {
                let mut state = state.lock().unwrap();
                for system in &systems::ordered() {
                    (system)(AppSystemContext {
                        state: &mut state,
                        event: &event,
                        app_sender: &app_sender,
                        model_runner_sender: &model_runner_sender,
                    });
                }
            }

            let state = state.lock().unwrap();
            let _ = terminal.draw(|frame| render(frame, &state));
        }
    });
}
