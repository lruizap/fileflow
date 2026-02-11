use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

use crate::{
    error::{FileFlowError, Result},
    log::{LogEntry, LogLevel},
    models::Progress,
};

#[derive(Debug, Clone)]
pub struct Context {
    logs: Vec<LogEntry>,
    progress: Option<Progress>,
    cancel_flag: Arc<AtomicBool>,
}

impl Context {
    pub fn new() -> Self {
        Self {
            logs: Vec::new(),
            progress: None,
            cancel_flag: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn log(&mut self, level: LogLevel, msg: impl Into<String>) {
        self.logs.push(LogEntry::new(level, msg));
    }

    pub fn info(&mut self, msg: impl Into<String>) {
        self.log(LogLevel::Info, msg);
    }

    pub fn warn(&mut self, msg: impl Into<String>) {
        self.log(LogLevel::Warn, msg);
    }

    pub fn error(&mut self, msg: impl Into<String>) {
        self.log(LogLevel::Error, msg);
    }

    pub fn set_progress(&mut self, progress: Progress) {
        self.progress = Some(progress);
    }

    pub fn progress(&self) -> Option<&Progress> {
        self.progress.as_ref()
    }

    pub fn take_logs(self) -> Vec<LogEntry> {
        self.logs
    }

    // Preparado para cancelación futura:
    pub fn cancel(&self) {
        self.cancel_flag.store(true, Ordering::SeqCst);
    }

    pub fn is_cancelled(&self) -> bool {
        self.cancel_flag.load(Ordering::SeqCst)
    }

    pub fn ensure_not_cancelled(&self) -> Result<()> {
        if self.is_cancelled() {
            return Err(FileFlowError::Cancelled);
        }
        Ok(())
    }
}

impl Default for Context {
    fn default() -> Self {
        Self::new()
    }
}
