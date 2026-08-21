use crate::utils::events::AppEvent;
use std::{
    sync::mpsc::Sender,
    thread::{self, JoinHandle},
    time::Duration,
};

pub fn spawn(app_sender: Sender<AppEvent>) -> JoinHandle<()> {
    let app_event_send_error_msg = "Failed to send AppEvent";
    thread::spawn(move || {
        loop {
            thread::sleep(Duration::from_millis(100));
            app_sender
                .send(AppEvent::Tick)
                .expect(app_event_send_error_msg);
        }
    })
}
