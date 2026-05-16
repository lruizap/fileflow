use std::sync::Arc;
use std::time::Instant;

use fileflow_actions as actions;
use fileflow_core::{Engine, JobStatus, Progress};
use tauri::{AppHandle, Emitter, State};

use crate::jobs::{
    timestamp_ms, JobManager, JobPriority, JobRequest, ManagedJob, ManagedJobStatus,
};
use crate::progress::{
    build_progress_payload, emit_progress, run_action_with_gui_progress, GuiProgressPayload,
    GuiRunResult,
};

#[tauri::command]
pub fn run_echo(_priority: Option<JobPriority>) -> Result<ManagedJob, String> {
    run_quick_echo_check()
}

#[tauri::command]
pub fn run_copy(
    app: AppHandle,
    state: State<'_, JobManager>,
    src: String,
    dst: String,
    overwrite: bool,
    priority: Option<JobPriority>,
) -> Result<ManagedJob, String> {
    let mut args = path_pair_args(src, dst);
    push_flag(&mut args, overwrite, "--overwrite");

    submit_action_job(
        app,
        &state,
        "run_copy",
        "Copiar archivo",
        "copy",
        "Copiando archivo",
        args,
        priority,
    )
}

#[tauri::command]
pub fn run_move(
    app: AppHandle,
    state: State<'_, JobManager>,
    src: String,
    dst: String,
    overwrite: bool,
    priority: Option<JobPriority>,
) -> Result<ManagedJob, String> {
    let mut args = path_pair_args(src, dst);
    push_flag(&mut args, overwrite, "--overwrite");

    submit_action_job(
        app,
        &state,
        "run_move",
        "Mover archivo",
        "move",
        "Moviendo archivo",
        args,
        priority,
    )
}

#[tauri::command]
pub fn run_sync(
    app: AppHandle,
    state: State<'_, JobManager>,
    src: String,
    dst: String,
    recursive: bool,
    delete_extra: bool,
    overwrite: bool,
    dry_run: bool,
    priority: Option<JobPriority>,
) -> Result<ManagedJob, String> {
    let mut args = path_pair_args(src, dst);
    push_flag(&mut args, recursive, "--recursive");
    push_flag(&mut args, delete_extra, "--delete-extra");
    push_flag(&mut args, overwrite, "--overwrite");
    push_flag(&mut args, dry_run, "--dry-run");

    let label = if dry_run {
        "Previsualizar sincronización"
    } else {
        "Sincronizar carpetas"
    };

    let progress_label = if dry_run {
        "Previsualizando sincronización"
    } else {
        "Sincronizando carpetas"
    };

    submit_action_job(
        app,
        &state,
        "run_sync",
        label,
        "sync",
        progress_label,
        args,
        priority,
    )
}

#[tauri::command]
pub fn run_watch(
    app: AppHandle,
    state: State<'_, JobManager>,
    path: String,
    config: String,
    recursive: bool,
    debounce_ms: u64,
    priority: Option<JobPriority>,
) -> Result<ManagedJob, String> {
    let mut args = vec![
        "--path".to_string(),
        path,
        "--config".to_string(),
        config,
        "--debounce-ms".to_string(),
        debounce_ms.to_string(),
    ];
    push_flag(&mut args, recursive, "--recursive");

    submit_action_job(
        app,
        &state,
        "run_watch",
        "Vigilar carpeta",
        "watch",
        "Vigilando carpeta",
        args,
        priority,
    )
}

#[tauri::command]
pub fn validate_config(
    app: AppHandle,
    state: State<'_, JobManager>,
    path: String,
    priority: Option<JobPriority>,
) -> Result<ManagedJob, String> {
    submit_job(
        app,
        &state,
        "validate_config",
        "Validar automatización",
        JobRequest::ValidateConfig { path },
        priority,
    )
}

#[tauri::command]
pub fn run_config(
    app: AppHandle,
    state: State<'_, JobManager>,
    path: String,
    priority: Option<JobPriority>,
) -> Result<ManagedJob, String> {
    submit_job(
        app,
        &state,
        "run_config",
        "Ejecutar automatización",
        JobRequest::RunConfig { path },
        priority,
    )
}

fn submit_action_job(
    app: AppHandle,
    manager: &JobManager,
    command: &str,
    label: &str,
    action_name: &str,
    action_label: &str,
    args: Vec<String>,
    priority: Option<JobPriority>,
) -> Result<ManagedJob, String> {
    submit_job(
        app,
        manager,
        command,
        label,
        JobRequest::Action {
            action_name: action_name.to_string(),
            action_label: action_label.to_string(),
            args,
        },
        priority,
    )
}

fn submit_job(
    app: AppHandle,
    manager: &JobManager,
    command: &str,
    label: &str,
    request: JobRequest,
    priority: Option<JobPriority>,
) -> Result<ManagedJob, String> {
    let job = manager.submit(command, label, request, priority.unwrap_or_default());
    emit_job_update(&app, &job);
    schedule_queued_jobs(app, manager.clone());
    Ok(manager.get_job(job.id).unwrap_or(job))
}

fn run_quick_echo_check() -> Result<ManagedJob, String> {
    let created_at = timestamp_ms();
    let action = actions::build_action("echo", &[]).map_err(|e| e.to_string())?;
    let engine = Engine::new();
    let out = engine.run_action(action.as_ref());
    let status = match out.job.status {
        JobStatus::Success => ManagedJobStatus::Success,
        JobStatus::Cancelled => ManagedJobStatus::Cancelled,
        JobStatus::Failed(_) => ManagedJobStatus::Failed,
        JobStatus::Pending | JobStatus::Running => ManagedJobStatus::Failed,
    };
    let error = match out.job.status {
        JobStatus::Failed(err) => Some(err),
        _ => None,
    };

    Ok(ManagedJob {
        id: 0,
        command: "run_echo".to_string(),
        label: "Prueba rápida".to_string(),
        status,
        priority: JobPriority::Normal,
        progress: None,
        logs: out
            .logs
            .into_iter()
            .map(|l| format!("[{:?}] {}", l.level, l.message))
            .collect(),
        error,
        created_at,
        started_at: Some(created_at),
        finished_at: Some(timestamp_ms()),
    })
}

pub fn schedule_queued_jobs(app: AppHandle, manager: JobManager) {
    while let Some(runnable) = manager.take_next_runnable() {
        emit_job_update(&app, &runnable.job);

        let app_for_job = app.clone();
        let manager_for_job = manager.clone();
        tauri::async_runtime::spawn_blocking(move || {
            let job_id = runnable.job.id;
            let result = run_managed_job(
                app_for_job.clone(),
                manager_for_job.clone(),
                job_id,
                runnable.request,
                runnable.cancel_flag,
            );

            let (status, logs, error) = result_to_job_finish(result);
            if let Some(job) = manager_for_job.finish_job(job_id, status, logs, error) {
                emit_job_update(&app_for_job, &job);
            }

            schedule_queued_jobs(app_for_job, manager_for_job);
        });
    }
}

fn run_managed_job(
    app: AppHandle,
    manager: JobManager,
    job_id: u64,
    request: JobRequest,
    cancel_flag: std::sync::Arc<std::sync::atomic::AtomicBool>,
) -> Result<GuiRunResult, String> {
    match request {
        JobRequest::Action {
            action_name,
            action_label,
            args,
        } => run_action_with_gui_progress(
            app,
            manager,
            job_id,
            cancel_flag,
            &action_name,
            &action_label,
            args,
        ),
        JobRequest::RunConfig { path } => run_config_job(app, manager, job_id, cancel_flag, path),
        JobRequest::ValidateConfig { path } => {
            validate_config_job(app, manager, job_id, cancel_flag, path)
        }
    }
}

fn validate_config_job(
    app: AppHandle,
    manager: JobManager,
    job_id: u64,
    cancel_flag: std::sync::Arc<std::sync::atomic::AtomicBool>,
    path: String,
) -> Result<GuiRunResult, String> {
    let started_at = Instant::now();

    emit_progress(
        &app,
        &manager,
        GuiProgressPayload {
            job_id,
            action: "Validando automatización".to_string(),
            file: path.clone(),
            current: 0,
            total: 1,
            percent: 0.0,
            elapsed_seconds: 0,
            eta_seconds: None,
            done: false,
        },
    );

    if cancel_flag.load(std::sync::atomic::Ordering::SeqCst) {
        return Ok(GuiRunResult {
            status: "CANCELLED".to_string(),
            logs: vec!["Operación cancelada".to_string()],
        });
    }

    let config = actions::load_pipeline_config(&path).map_err(|e| e.to_string())?;

    let mut logs = vec![
        "Config OK".to_string(),
        format!("Nombre: {}", config.name),
        format!("Steps: {}", config.steps.len()),
    ];

    for (index, step) in config.steps.iter().enumerate() {
        logs.push(format!("{}. {}", index + 1, step.action));
    }

    emit_progress(
        &app,
        &manager,
        build_progress_payload(
            job_id,
            "Validando automatización",
            &Progress::new(1, 1).with_message("Validación completada"),
            started_at,
            true,
        ),
    );

    Ok(GuiRunResult {
        status: "SUCCESS".to_string(),
        logs,
    })
}

fn run_config_job(
    app: AppHandle,
    manager: JobManager,
    job_id: u64,
    cancel_flag: std::sync::Arc<std::sync::atomic::AtomicBool>,
    path: String,
) -> Result<GuiRunResult, String> {
    let action = actions::build_pipeline_from_config_file(&path).map_err(|e| e.to_string())?;
    let engine = Engine::new();
    let started_at = Instant::now();
    let app_for_progress = app.clone();
    let manager_for_progress = manager.clone();

    emit_progress(
        &app,
        &manager,
        GuiProgressPayload {
            job_id,
            action: "Ejecutando automatización".to_string(),
            file: path,
            current: 0,
            total: 1,
            percent: 0.0,
            elapsed_seconds: 0,
            eta_seconds: None,
            done: false,
        },
    );

    let listener = Arc::new(move |progress: Progress| {
        let payload = build_progress_payload(
            job_id,
            "Ejecutando automatización",
            &progress,
            started_at,
            false,
        );
        emit_progress(&app_for_progress, &manager_for_progress, payload);
    });

    let out = engine.run_action_with_progress_and_cancel(action.as_ref(), listener, cancel_flag);
    let final_progress = out
        .job
        .progress
        .clone()
        .unwrap_or_else(|| Progress::new(1, 1).with_message("Automatización finalizada"));

    emit_progress(
        &app,
        &manager,
        build_progress_payload(
            job_id,
            "Ejecutando automatización",
            &final_progress,
            started_at,
            true,
        ),
    );

    Ok(GuiRunResult {
        status: format_status(out.job.status),
        logs: out
            .logs
            .into_iter()
            .map(|l| format!("[{:?}] {}", l.level, l.message))
            .collect(),
    })
}

fn result_to_job_finish(
    result: Result<GuiRunResult, String>,
) -> (ManagedJobStatus, Vec<String>, Option<String>) {
    match result {
        Ok(result) if result.status == "SUCCESS" => (ManagedJobStatus::Success, result.logs, None),
        Ok(result) if result.status == "CANCELLED" => {
            (ManagedJobStatus::Cancelled, result.logs, None)
        }
        Ok(result) if result.status.starts_with("FAILED:") => (
            ManagedJobStatus::Failed,
            result.logs,
            Some(result.status.trim_start_matches("FAILED: ").to_string()),
        ),
        Ok(result) => (ManagedJobStatus::Failed, result.logs, Some(result.status)),
        Err(err) => (ManagedJobStatus::Failed, vec![err.clone()], Some(err)),
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

fn emit_job_update(app: &AppHandle, job: &ManagedJob) {
    let _ = app.emit("fileflow-job-updated", job);
}

fn path_pair_args(src: String, dst: String) -> Vec<String> {
    vec!["--src".to_string(), src, "--dst".to_string(), dst]
}

fn push_flag(args: &mut Vec<String>, enabled: bool, flag: &str) {
    if enabled {
        args.push(flag.to_string());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn push_flag_adds_overwrite_when_enabled() {
        let mut args = path_pair_args("a.txt".to_string(), "b.txt".to_string());

        push_flag(&mut args, true, "--overwrite");

        assert!(args.contains(&"--overwrite".to_string()));
    }

    #[test]
    fn push_flag_does_not_add_overwrite_when_disabled() {
        let mut args = path_pair_args("a.txt".to_string(), "b.txt".to_string());

        push_flag(&mut args, false, "--overwrite");

        assert!(!args.contains(&"--overwrite".to_string()));
    }
}
