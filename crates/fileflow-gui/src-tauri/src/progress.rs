use std::sync::{atomic::AtomicBool, Arc};
use std::time::Instant;

use fileflow_actions as actions;
use fileflow_core::{Engine, JobStatus, LogEntry, Progress};
use serde::Serialize;
use tauri::{AppHandle, Emitter};

use crate::jobs::JobManager;

#[derive(Debug, Serialize)]
pub struct GuiRunResult {
    pub status: String,
    pub logs: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GuiProgressPayload {
    pub job_id: u64,
    pub action: String,
    pub file: String,
    pub current: u64,
    pub total: u64,
    pub percent: f64,
    pub elapsed_seconds: u64,
    pub eta_seconds: Option<u64>,
    pub done: bool,
}

pub fn run_action_with_gui_progress(
    app: AppHandle,
    manager: JobManager,
    job_id: u64,
    cancel_flag: Arc<AtomicBool>,
    action_name: &str,
    action_label: &str,
    args: Vec<String>,
) -> Result<GuiRunResult, String> {
    let action = actions::build_action(action_name, &args).map_err(|e| e.to_string())?;
    let engine = Engine::new();

    let started_at = Instant::now();
    let app_for_progress = app.clone();
    let manager_for_progress = manager.clone();
    let label_for_progress = action_label.to_string();

    emit_progress(
        &app,
        &manager,
        GuiProgressPayload {
            job_id,
            action: action_label.to_string(),
            file: "Preparando operación...".to_string(),
            current: 0,
            total: 1,
            percent: 0.0,
            elapsed_seconds: 0,
            eta_seconds: None,
            done: false,
        },
    );

    let listener = Arc::new(move |progress: Progress| {
        let payload =
            build_progress_payload(job_id, &label_for_progress, &progress, started_at, false);
        emit_progress(&app_for_progress, &manager_for_progress, payload);
    });

    let out =
        engine.run_action_with_progress_and_cancel(action.as_ref(), listener, cancel_flag.clone());

    let final_progress = out
        .job
        .progress
        .clone()
        .unwrap_or_else(|| Progress::new(1, 1).with_message("Operación finalizada"));

    emit_progress(
        &app,
        &manager,
        build_progress_payload(job_id, action_label, &final_progress, started_at, true),
    );

    Ok(GuiRunResult {
        status: format_status(out.job.status),
        logs: format_logs(out.logs),
    })
}

pub fn emit_progress(app: &AppHandle, manager: &JobManager, payload: GuiProgressPayload) {
    let _ = manager.update_progress(payload.job_id, payload.clone());
    let _ = app.emit("fileflow-progress", payload);
}

pub fn build_progress_payload(
    job_id: u64,
    action_label: &str,
    progress: &Progress,
    started_at: Instant,
    done: bool,
) -> GuiProgressPayload {
    let total = progress.total.max(1);
    let current = progress.current.min(total);
    let percent = (current as f64 / total as f64) * 100.0;
    let elapsed_seconds = started_at.elapsed().as_secs();

    let eta_seconds = if current > 0 && !done {
        let speed = current as f64 / elapsed_seconds.max(1) as f64;
        let remaining = total.saturating_sub(current) as f64;

        if speed > 0.0 {
            Some((remaining / speed).ceil() as u64)
        } else {
            None
        }
    } else {
        None
    };

    GuiProgressPayload {
        job_id,
        action: action_label.to_string(),
        file: progress
            .message
            .clone()
            .unwrap_or_else(|| "Procesando...".to_string()),
        current,
        total,
        percent,
        elapsed_seconds,
        eta_seconds,
        done,
    }
}

fn format_status(status: JobStatus) -> String {
    match status {
        JobStatus::Success => "SUCCESS".to_string(),
        JobStatus::Failed(err) => format!("FAILED: {err}"),
        JobStatus::Cancelled => "CANCELLED".to_string(),
        JobStatus::Running => "RUNNING".to_string(),
        JobStatus::Pending => "PENDING".to_string(),
    }
}

fn format_logs(logs: Vec<LogEntry>) -> Vec<String> {
    logs.into_iter()
        .map(|l| format!("[{:?}] {}", l.level, l.message))
        .collect()
}
