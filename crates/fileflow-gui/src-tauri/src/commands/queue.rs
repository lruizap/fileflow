use tauri::{AppHandle, Emitter, State};

use crate::jobs::{JobManager, JobPriority, ManagedJob, QueueState};

use super::actions::schedule_queued_jobs;

#[tauri::command]
pub fn get_queue_state(state: State<'_, JobManager>) -> Result<QueueState, String> {
    Ok(state.state())
}

#[tauri::command]
pub fn set_concurrency_limit(
    app: AppHandle,
    state: State<'_, JobManager>,
    limit: usize,
) -> Result<QueueState, String> {
    let queue = state.set_concurrency_limit(limit);
    schedule_queued_jobs(app, state.inner().clone());
    Ok(queue)
}

#[tauri::command]
pub fn update_job_priority(
    app: AppHandle,
    state: State<'_, JobManager>,
    job_id: u64,
    priority: JobPriority,
) -> Result<ManagedJob, String> {
    let job = state.update_priority(job_id, priority)?;
    let _ = app.emit("fileflow-job-updated", &job);
    schedule_queued_jobs(app, state.inner().clone());
    Ok(job)
}

#[tauri::command]
pub fn cancel_job(
    app: AppHandle,
    state: State<'_, JobManager>,
    job_id: u64,
) -> Result<ManagedJob, String> {
    let job = state.cancel_job(job_id)?;
    let _ = app.emit("fileflow-job-updated", &job);
    schedule_queued_jobs(app, state.inner().clone());
    Ok(job)
}
