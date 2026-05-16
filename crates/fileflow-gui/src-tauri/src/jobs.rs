use std::collections::{HashMap, HashSet};
use std::sync::{
    atomic::{AtomicBool, AtomicU64, Ordering},
    Arc, Mutex,
};
use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};

use crate::progress::GuiProgressPayload;

pub const DEFAULT_CONCURRENCY_LIMIT: usize = 2;
pub const MAX_CONCURRENCY_LIMIT: usize = 8;

#[derive(Debug, Clone)]
pub enum JobRequest {
    Action {
        action_name: String,
        action_label: String,
        args: Vec<String>,
    },
    RunConfig {
        path: String,
    },
    ValidateConfig {
        path: String,
    },
}

#[derive(Debug, Clone, Copy, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum JobPriority {
    Low,
    Normal,
    High,
    Critical,
}

impl JobPriority {
    fn score(self) -> u8 {
        match self {
            Self::Low => 0,
            Self::Normal => 1,
            Self::High => 2,
            Self::Critical => 3,
        }
    }
}

impl Default for JobPriority {
    fn default() -> Self {
        Self::Normal
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ManagedJobStatus {
    Queued,
    Running,
    Success,
    Failed,
    Cancelled,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ManagedJob {
    pub id: u64,
    pub command: String,
    pub label: String,
    pub status: ManagedJobStatus,
    pub priority: JobPriority,
    pub progress: Option<GuiProgressPayload>,
    pub logs: Vec<String>,
    pub error: Option<String>,
    pub created_at: u128,
    pub started_at: Option<u128>,
    pub finished_at: Option<u128>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QueueState {
    pub jobs: Vec<ManagedJob>,
    pub concurrency_limit: usize,
    pub running_count: usize,
    pub queued_count: usize,
}

#[derive(Clone)]
pub struct JobManager {
    inner: Arc<Mutex<JobManagerInner>>,
    next_id: Arc<AtomicU64>,
}

struct JobManagerInner {
    jobs: HashMap<u64, JobRecord>,
    running: HashSet<u64>,
    concurrency_limit: usize,
    next_order: u64,
}

struct JobRecord {
    job: ManagedJob,
    request: JobRequest,
    cancel_flag: Arc<AtomicBool>,
    order: u64,
}

impl Default for JobManager {
    fn default() -> Self {
        Self {
            inner: Arc::new(Mutex::new(JobManagerInner {
                jobs: HashMap::new(),
                running: HashSet::new(),
                concurrency_limit: DEFAULT_CONCURRENCY_LIMIT,
                next_order: 0,
            })),
            next_id: Arc::new(AtomicU64::new(1)),
        }
    }
}

impl JobManager {
    pub fn submit(
        &self,
        command: impl Into<String>,
        label: impl Into<String>,
        request: JobRequest,
        priority: JobPriority,
    ) -> ManagedJob {
        let id = self.next_id.fetch_add(1, Ordering::SeqCst);
        let mut inner = self.inner.lock().expect("job manager mutex poisoned");
        let order = inner.next_order;
        inner.next_order += 1;

        let job = ManagedJob {
            id,
            command: command.into(),
            label: label.into(),
            status: ManagedJobStatus::Queued,
            priority,
            progress: None,
            logs: Vec::new(),
            error: None,
            created_at: timestamp_ms(),
            started_at: None,
            finished_at: None,
        };

        inner.jobs.insert(
            id,
            JobRecord {
                job: job.clone(),
                request,
                cancel_flag: Arc::new(AtomicBool::new(false)),
                order,
            },
        );

        job
    }

    pub fn state(&self) -> QueueState {
        let inner = self.inner.lock().expect("job manager mutex poisoned");
        inner.state()
    }

    pub fn get_job(&self, job_id: u64) -> Option<ManagedJob> {
        let inner = self.inner.lock().expect("job manager mutex poisoned");
        inner.jobs.get(&job_id).map(|record| record.job.clone())
    }

    pub fn set_concurrency_limit(&self, limit: usize) -> QueueState {
        let mut inner = self.inner.lock().expect("job manager mutex poisoned");
        inner.concurrency_limit = limit.clamp(1, MAX_CONCURRENCY_LIMIT);
        inner.state()
    }

    pub fn update_priority(
        &self,
        job_id: u64,
        priority: JobPriority,
    ) -> Result<ManagedJob, String> {
        let mut inner = self.inner.lock().expect("job manager mutex poisoned");
        let record = inner
            .jobs
            .get_mut(&job_id)
            .ok_or_else(|| format!("Job {job_id} no existe"))?;

        if !matches!(record.job.status, ManagedJobStatus::Queued) {
            return Err("Solo se puede cambiar la prioridad de trabajos en cola".to_string());
        }

        record.job.priority = priority;
        Ok(record.job.clone())
    }

    pub fn cancel_job(&self, job_id: u64) -> Result<ManagedJob, String> {
        let mut inner = self.inner.lock().expect("job manager mutex poisoned");
        let record = inner
            .jobs
            .get_mut(&job_id)
            .ok_or_else(|| format!("Job {job_id} no existe"))?;

        match record.job.status {
            ManagedJobStatus::Queued => {
                record.cancel_flag.store(true, Ordering::SeqCst);
                record.job.status = ManagedJobStatus::Cancelled;
                record.job.finished_at = Some(timestamp_ms());
                record
                    .job
                    .logs
                    .push("Trabajo cancelado antes de iniciar.".to_string());
            }
            ManagedJobStatus::Running => {
                record.cancel_flag.store(true, Ordering::SeqCst);
            }
            ManagedJobStatus::Success | ManagedJobStatus::Failed | ManagedJobStatus::Cancelled => {}
        }

        Ok(record.job.clone())
    }

    pub fn update_progress(&self, job_id: u64, progress: GuiProgressPayload) -> Option<ManagedJob> {
        let mut inner = self.inner.lock().expect("job manager mutex poisoned");
        let record = inner.jobs.get_mut(&job_id)?;
        record.job.progress = Some(progress);
        Some(record.job.clone())
    }

    pub fn finish_job(
        &self,
        job_id: u64,
        status: ManagedJobStatus,
        logs: Vec<String>,
        error: Option<String>,
    ) -> Option<ManagedJob> {
        let mut inner = self.inner.lock().expect("job manager mutex poisoned");
        inner.running.remove(&job_id);

        let record = inner.jobs.get_mut(&job_id)?;
        record.job.status = status;
        record.job.logs = logs;
        record.job.error = error;
        record.job.finished_at = Some(timestamp_ms());
        Some(record.job.clone())
    }

    pub fn take_next_runnable(&self) -> Option<RunnableJob> {
        let mut inner = self.inner.lock().expect("job manager mutex poisoned");

        if inner.running.len() >= inner.concurrency_limit {
            return None;
        }

        let next_id = inner
            .jobs
            .iter()
            .filter(|(_, record)| matches!(record.job.status, ManagedJobStatus::Queued))
            .max_by_key(|(_, record)| {
                (record.job.priority.score(), std::cmp::Reverse(record.order))
            })
            .map(|(id, _)| *id)?;

        inner.running.insert(next_id);
        let record = inner.jobs.get_mut(&next_id)?;
        record.job.status = ManagedJobStatus::Running;
        record.job.started_at = Some(timestamp_ms());

        Some(RunnableJob {
            job: record.job.clone(),
            request: record.request.clone(),
            cancel_flag: record.cancel_flag.clone(),
        })
    }
}

impl JobManagerInner {
    fn state(&self) -> QueueState {
        let mut jobs = self
            .jobs
            .values()
            .map(|record| record.job.clone())
            .collect::<Vec<_>>();

        jobs.sort_by_key(|job| job.created_at);
        jobs.reverse();

        QueueState {
            jobs,
            concurrency_limit: self.concurrency_limit,
            running_count: self.running.len(),
            queued_count: self
                .jobs
                .values()
                .filter(|record| matches!(record.job.status, ManagedJobStatus::Queued))
                .count(),
        }
    }
}

#[derive(Clone)]
pub struct RunnableJob {
    pub job: ManagedJob,
    pub request: JobRequest,
    pub cancel_flag: Arc<AtomicBool>,
}

pub(crate) fn timestamp_ms() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_millis())
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn request() -> JobRequest {
        JobRequest::Action {
            action_name: "echo".to_string(),
            action_label: "Echo".to_string(),
            args: Vec::new(),
        }
    }

    #[test]
    fn starts_jobs_by_priority_then_fifo_order() {
        let manager = JobManager::default();
        manager.set_concurrency_limit(2);

        let low = manager.submit("run_echo", "Low", request(), JobPriority::Low);
        let high = manager.submit("run_echo", "High", request(), JobPriority::High);
        let normal = manager.submit("run_echo", "Normal", request(), JobPriority::Normal);

        let first = manager.take_next_runnable().expect("first runnable job");
        let second = manager.take_next_runnable().expect("second runnable job");
        let third = manager.take_next_runnable();

        assert_eq!(first.job.id, high.id);
        assert_eq!(second.job.id, normal.id);
        assert!(third.is_none());

        manager.finish_job(first.job.id, ManagedJobStatus::Success, Vec::new(), None);
        let next = manager.take_next_runnable().expect("next queued job");
        assert_eq!(next.job.id, low.id);
    }

    #[test]
    fn clamps_concurrency_limit() {
        let manager = JobManager::default();

        assert_eq!(manager.set_concurrency_limit(0).concurrency_limit, 1);
        assert_eq!(
            manager
                .set_concurrency_limit(MAX_CONCURRENCY_LIMIT + 20)
                .concurrency_limit,
            MAX_CONCURRENCY_LIMIT
        );
    }

    #[test]
    fn queued_job_can_be_cancelled_before_running() {
        let manager = JobManager::default();
        let job = manager.submit("run_echo", "Queued", request(), JobPriority::Normal);

        let cancelled = manager.cancel_job(job.id).expect("cancel queued job");

        assert!(matches!(cancelled.status, ManagedJobStatus::Cancelled));
        assert!(manager.take_next_runnable().is_none());
    }
}
