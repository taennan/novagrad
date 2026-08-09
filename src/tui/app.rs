use crate::{
    models::{
        CategorisedDataset, CategorisedModel, Model, Models,
        datasets::{Datasets, ExponentialDataset},
        exponential_predictor_v2::ExpPredictor,
    },
    tui::{
        render::render,
        systems::{home_screen_switcher, model_run_input, title_setter, wizard_exit, wizard_input},
        types::{AppEvent, AppState, AppSystem, ModelRunnerEvent, RunMode, ScreenState},
    },
};
use crossterm::event::Event;
use ratatui::DefaultTerminal;
use std::{
    collections::HashMap,
    sync::{
        Arc, Mutex,
        mpsc::{self, Receiver, Sender},
    },
    thread::{self, JoinHandle},
    time::Duration,
};

pub fn run() {
    let (sender, receiver) = mpsc::channel::<AppEvent>();
    let (model_runner_sender, model_runner_receiver) = mpsc::channel::<ModelRunnerEvent>();

    let state = Arc::new(Mutex::new(AppState::default()));
    let systems = [
        title_setter::new(),
        home_screen_switcher::new(),
        wizard_input::new(),
        wizard_exit::new(),
        model_run_input::new(),
    ];

    let app_event_send_error_msg = "Failed to send AppEvent";

    let input_sender = sender.clone();
    thread::spawn(move || {
        loop {
            let error_message = "IO error when listening for user input";
            let has_event =
                crossterm::event::poll(Duration::from_millis(100)).expect(error_message);
            if !has_event {
                continue;
            }

            match crossterm::event::read().expect(error_message) {
                Event::Key(key_event) if key_event.is_press() => {
                    if key_event.is_press() {
                        input_sender
                            .send(AppEvent::KeyPress(key_event.code))
                            .expect(app_event_send_error_msg);
                    } else if key_event.is_release() {
                        input_sender
                            .send(AppEvent::KeyRelease(key_event.code))
                            .expect(app_event_send_error_msg);
                    }
                }
                _ => (),
            }
        }
    });

    let tick_sender = sender.clone();
    thread::spawn(move || {
        loop {
            thread::sleep(Duration::from_millis(100));
            tick_sender
                .send(AppEvent::Tick)
                .expect(app_event_send_error_msg);
        }
    });

    let model_runner_thread_app_state = state.clone();
    thread::spawn(move || {
        let mut categorised_model = Option::<CategorisedModel>::None;
        let mut categorised_dataset = Option::<CategorisedDataset>::None;
        let mut paused = true;
        loop {
            while let Ok(event) = model_runner_receiver.try_recv() {
                match event {
                    ModelRunnerEvent::Start(new_model) => {
                        categorised_model = Some(match new_model {
                            Models::ExpPredictor => {
                                CategorisedModel::F32(Box::new(ExpPredictor::new()))
                            }
                        });
                        paused = false;
                    }
                    ModelRunnerEvent::Pause => {
                        paused = true;
                    }
                    ModelRunnerEvent::Resume => {
                        paused = false;
                    }
                    ModelRunnerEvent::Stop => {
                        categorised_model = None;
                        paused = true;
                    }
                    ModelRunnerEvent::SetDataset(new_dataset) => {
                        categorised_dataset = Some(match new_dataset {
                            Datasets::ExponentialF32 => CategorisedDataset::F32(Box::new(
                                ExponentialDataset::new(500, 1321432),
                            )),
                        })
                    }
                }
            }

            let state = model_runner_thread_app_state.lock().unwrap();
            if !paused
                && let Some(categorised_model) = &categorised_model
                && let Some(categorised_dataset) = &categorised_dataset
                && let ScreenState::ModelRun { mode, .. } = state.screen
            {
                let hyperparams = HashMap::new();
                match (&categorised_model, &categorised_dataset) {
                    (CategorisedModel::F32(model), CategorisedDataset::F32(dataset)) => {
                        match mode {
                            RunMode::Test => {
                                model.test(dataset.as_ref()).expect("Testing run failed")
                            }
                            RunMode::Train => model
                                .train(&hyperparams, dataset.as_ref())
                                .expect("Training run failed"),
                        }
                    }
                    (CategorisedModel::F64(model), CategorisedDataset::F64(dataset)) => {
                        match mode {
                            RunMode::Test => {
                                model.test(dataset.as_ref()).expect("Testing run failed")
                            }
                            RunMode::Train => model
                                .train(&hyperparams, dataset.as_ref())
                                .expect("Training run failed"),
                        }
                    }
                    _ => {}
                }
            }

            thread::sleep(Duration::from_millis(250));
        }
    });

    ratatui::run(|terminal| mainloop(terminal, &receiver, &systems, state.clone()));
}

fn spawn_model_runner_thread(sender: Sender<AppEvent>) -> JoinHandle<()> {
    thread::spawn(move || {
        loop {
            thread::sleep(Duration::from_millis(500));
        }
    })
}

fn mainloop(
    terminal: &mut DefaultTerminal,
    receiver: &Receiver<AppEvent>,
    systems: &[AppSystem],
    state: Arc<Mutex<AppState>>,
) {
    while let Ok(event) = receiver.recv() {
        if matches!(event, AppEvent::Quit) {
            break;
        }

        {
            let mut state = state.lock().unwrap();
            for system in systems {
                (system)(&mut state);
            }
        }

        let state = state.lock().unwrap();
        let _ = terminal.draw(|frame| render(frame, &state));
    }
}
