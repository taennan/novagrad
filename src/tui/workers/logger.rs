use crate::utils::LogWorkerEvent;
use std::{
    fs::{self, OpenOptions},
    io::Write,
    path::{Path, PathBuf},
    sync::{Arc, Mutex, mpsc::Receiver},
    thread::{self, JoinHandle},
};

#[derive(Debug)]
pub struct LogWorkerState {
    max_logs: usize,
    logs: Vec<String>,
}

impl LogWorkerState {
    pub fn new() -> Self {
        Self {
            max_logs: 100,
            logs: vec![],
        }
    }

    pub fn push(&mut self, log: String) {
        let is_maxed = self.logs.len() == self.max_logs;
        if is_maxed {
            let _ = self.logs.remove(0);
        }
        self.logs.push(log);
    }

    pub fn logs(&self) -> &[String] {
        &self.logs
    }
}

pub fn spawn<P>(
    logpath: P,
    state: Arc<Mutex<LogWorkerState>>,
    receiver: Receiver<LogWorkerEvent>,
) -> JoinHandle<()>
where
    P: Into<PathBuf>,
{
    let logpath = logpath.into();
    thread::spawn(move || {
        while let Ok(event) = receiver.recv() {
            match &event {
                LogWorkerEvent::Clear => fs::remove_file(&logpath).unwrap(),
                LogWorkerEvent::Log(s) => write_log(&logpath, &state, "LOG", s),
                LogWorkerEvent::Info(s) => write_log(&logpath, &state, "INFO", s),
                LogWorkerEvent::Warn(s) => write_log(&logpath, &state, "WARN", s),
                LogWorkerEvent::Error(s) => write_log(&logpath, &state, "ERROR", s),
            }
        }
    })
}

fn write_log<P>(logpath: P, state: &Arc<Mutex<LogWorkerState>>, variant: &str, body: &str)
where
    P: AsRef<Path>,
{
    let Ok(mut file) = OpenOptions::new().create(true).append(true).open(logpath) else {
        return;
    };

    let now = chrono::offset::Local::now().time();
    let message = format!("[{} {}] {}\n", variant, now, body);

    let _ = file.write(message.as_bytes());
    state.lock().unwrap().push(message);
}
