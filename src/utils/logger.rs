use std::sync::mpsc::Sender;

#[derive(Debug)]
pub enum LogWorkerEvent {
    Log(String),
    Info(String),
    Warn(String),
    Error(String),
    Clear,
}

#[derive(Clone, Debug)]
pub struct Logger {
    sender: Sender<LogWorkerEvent>,
}

impl Logger {
    pub fn new(sender: Sender<LogWorkerEvent>) -> Self {
        Self { sender }
    }

    pub fn clear(&self) {
        let _ = self.sender.send(LogWorkerEvent::Clear);
    }

    pub fn log<S>(&self, message: S)
    where
        S: Into<String>,
    {
        let _ = self.sender.send(LogWorkerEvent::Log(message.into()));
    }

    pub fn info<S>(&self, message: S)
    where
        S: Into<String>,
    {
        let _ = self.sender.send(LogWorkerEvent::Info(message.into()));
    }

    pub fn warn<S>(&self, message: S)
    where
        S: Into<String>,
    {
        let _ = self.sender.send(LogWorkerEvent::Warn(message.into()));
    }

    pub fn error<S>(&self, message: S)
    where
        S: Into<String>,
    {
        let _ = self.sender.send(LogWorkerEvent::Error(message.into()));
    }
}
