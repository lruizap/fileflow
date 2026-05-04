use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::SystemTime;

use crate::{
    action::Action,
    context::Context,
    error::Result,
    models::{Job, JobId, JobStatus, Progress},
    LogEntry,
};

static NEXT_JOB_ID: AtomicU64 = AtomicU64::new(1);

#[derive(Debug)]
pub struct Engine;

#[derive(Debug)]
pub struct JobRunResult {
    pub job: Job,
    pub logs: Vec<LogEntry>,
}

impl Engine {
    pub fn new() -> Self {
        Self
    }

    pub fn run_action(&self, action: &dyn Action) -> JobRunResult {
        self.run_action_internal(action, None)
    }

    pub fn run_action_with_progress(
        &self,
        action: &dyn Action,
        listener: Arc<dyn Fn(Progress) + Send + Sync>,
    ) -> JobRunResult {
        self.run_action_internal(action, Some(listener))
    }

    fn run_action_internal(
        &self,
        action: &dyn Action,
        listener: Option<Arc<dyn Fn(Progress) + Send + Sync>>,
    ) -> JobRunResult {
        let id = JobId(NEXT_JOB_ID.fetch_add(1, Ordering::SeqCst));
        let mut job = Job::new(id, action.name());

        let mut ctx = match listener {
            Some(listener) => Context::with_progress_listener(listener),
            None => Context::new(),
        };

        job.status = JobStatus::Running;
        job.started_at = Some(SystemTime::now());

        let result: Result<()> = action.execute(&mut ctx);

        job.progress = ctx.progress().cloned();
        job.finished_at = Some(SystemTime::now());

        job.status = match result {
            Ok(_) => JobStatus::Success,
            Err(e) => match e {
                crate::error::FileFlowError::Cancelled => JobStatus::Cancelled,
                other => JobStatus::Failed(other.to_string()),
            },
        };

        let logs = ctx.take_logs();
        JobRunResult { job, logs }
    }
}

impl Default for Engine {
    fn default() -> Self {
        Self::new()
    }
}