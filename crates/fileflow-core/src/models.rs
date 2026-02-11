use std::time::SystemTime;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct JobId(pub u64);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum JobStatus {
    Pending,
    Running,
    Success,
    Failed(String),
    Cancelled,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Progress {
    pub current: u64,
    pub total: u64,
    pub message: Option<String>,
}

impl Progress {
    pub fn new(current: u64, total: u64) -> Self {
        Self { current, total, message: None }
    }

    pub fn with_message(mut self, msg: impl Into<String>) -> Self {
        self.message = Some(msg.into());
        self
    }

    pub fn percent(&self) -> Option<f64> {
        if self.total == 0 { return None; }
        Some((self.current as f64 / self.total as f64) * 100.0)
    }
}

#[derive(Debug, Clone)]
pub struct Job {
    pub id: JobId,
    pub action_name: String,
    pub status: JobStatus,
    pub created_at: SystemTime,
    pub started_at: Option<SystemTime>,
    pub finished_at: Option<SystemTime>,
    pub progress: Option<Progress>,
}

impl Job {
    pub fn new(id: JobId, action_name: impl Into<String>) -> Self {
        Self {
            id,
            action_name: action_name.into(),
            status: JobStatus::Pending,
            created_at: SystemTime::now(),
            started_at: None,
            finished_at: None,
            progress: None,
        }
    }
}
