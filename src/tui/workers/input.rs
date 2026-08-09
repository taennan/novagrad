use crate::tui::types::AppEvent;
use crossterm::event::Event;
use std::{
    sync::mpsc::Sender,
    thread::{self, JoinHandle},
    time::Duration,
};

pub fn spawn(app_sender: Sender<AppEvent>) -> JoinHandle<()> {
    let app_event_send_error_msg = "Failed to send AppEvent";
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
                        app_sender
                            .send(AppEvent::KeyPress(key_event.code))
                            .expect(app_event_send_error_msg);
                    } else if key_event.is_release() {
                        app_sender
                            .send(AppEvent::KeyRelease(key_event.code))
                            .expect(app_event_send_error_msg);
                    }
                }
                _ => (),
            }
        }
    })
}
