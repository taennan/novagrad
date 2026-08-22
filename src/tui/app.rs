use crate::{
    tui::{
        render::render,
        systems,
        workers::{self, logger::LogWorkerState},
    },
    utils::{
        LogWorkerEvent, Logger,
        events::{AppEvent, ModelRunnerEvent},
        state::AppState,
        system::AppSystemContext,
    },
};
use std::{
    env,
    sync::{Arc, Mutex, mpsc},
};

pub fn run() {
    let (app_sender, app_receiver) = mpsc::channel::<AppEvent>();
    let (log_sender, log_receiver) = mpsc::channel::<LogWorkerEvent>();
    let (model_runner_sender, model_runner_receiver) = mpsc::channel::<ModelRunnerEvent>();

    let data_dir = env::current_dir().unwrap().join("data");
    let logfile = data_dir.join("logs.txt");
    let logger = Logger::new(log_sender);
    let log_state = Arc::new(Mutex::new(LogWorkerState::new()));
    logger.clear();

    let app_state = Arc::new(Mutex::new(AppState::default()));

    workers::logger::spawn(&logfile, log_state.clone(), log_receiver);
    workers::input::spawn(app_sender.clone());
    workers::ticker::spawn(app_sender.clone());
    workers::model_runner::spawn(
        model_runner_receiver,
        model_runner_sender.clone(),
        app_sender.clone(),
        logger.clone(),
    );

    ratatui::run(|terminal| {
        let _ = app_sender.send(AppEvent::SetTitle("Novagrad".into()));

        while let Ok(event) = app_receiver.recv() {
            if matches!(event, AppEvent::Quit) {
                break;
            }

            {
                let mut app_state = app_state.lock().unwrap();
                for system in &systems::ordered() {
                    (system)(AppSystemContext {
                        state: &mut app_state,
                        event: &event,
                        app_sender: &app_sender,
                        model_runner_sender: &model_runner_sender,
                        logger: &logger,
                    });
                }

                app_state.keys_pressed.clear();
            }

            let app_state = app_state.lock().unwrap();
            let _ = terminal.draw(|frame| render(frame, &app_state, log_state.clone()));
        }
    });
}
