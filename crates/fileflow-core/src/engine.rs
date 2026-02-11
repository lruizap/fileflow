use std::sync::atomic::{AtomicU64, Ordering};
use std::time::SystemTime;

use crate::{
    action::Action,
    context::Context,
    error::Result,
    models::{Job, JobId, JobStatus},
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

    pub fn run_action<A: Action>(&self, action: &A) -> JobRunResult {
        let id = JobId(NEXT_JOB_ID.fetch_add(1, Ordering::SeqCst));
        let mut job = Job::new(id, action.name());

        let mut ctx = Context::new();

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
