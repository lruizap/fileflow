use std::sync::atomic::Ordering;
use std::time::Instant;

use fileflow_actions as actions;
use fileflow_core::{Engine, JobStatus, Progress};
use tauri::{AppHandle, State};

use crate::progress::{
    build_progress_payload, emit_progress, run_action_with_gui_progress, GuiProgressPayload,
    GuiRunResult,
};
use crate::state::CancelState;

#[tauri::command]
pub async fn run_echo(
    app: AppHandle,
    state: State<'_, CancelState>,
) -> Result<GuiRunResult, String> {
    let cancel_flag = state.flag.clone();

    tauri::async_runtime::spawn_blocking(move || {
        run_action_with_gui_progress(
            app,
            cancel_flag,
            "echo",
            "Comprobando funcionamiento",
            vec![],
        )
    })
    .await
    .map_err(|e| format!("Error ejecutando echo: {e}"))?
}

#[tauri::command]
pub async fn run_copy(
    app: AppHandle,
    state: State<'_, CancelState>,
    src: String,
    dst: String,
    overwrite: bool,
) -> Result<GuiRunResult, String> {
    let cancel_flag = state.flag.clone();

    tauri::async_runtime::spawn_blocking(move || {
        let mut args = vec!["--src".to_string(), src, "--dst".to_string(), dst];

        if overwrite {
            args.push("--overwrite".to_string());
        }

        run_action_with_gui_progress(app, cancel_flag, "copy", "Copiando archivo", args)
    })
    .await
    .map_err(|e| format!("Error ejecutando copy: {e}"))?
}

#[tauri::command]
pub async fn run_move(
    app: AppHandle,
    state: State<'_, CancelState>,
    src: String,
    dst: String,
    overwrite: bool,
) -> Result<GuiRunResult, String> {
    let cancel_flag = state.flag.clone();

    tauri::async_runtime::spawn_blocking(move || {
        let mut args = vec!["--src".to_string(), src, "--dst".to_string(), dst];

        if overwrite {
            args.push("--overwrite".to_string());
        }

        run_action_with_gui_progress(app, cancel_flag, "move", "Moviendo archivo", args)
    })
    .await
    .map_err(|e| format!("Error ejecutando move: {e}"))?
}

#[tauri::command]
pub async fn run_sync(
    app: AppHandle,
    state: State<'_, CancelState>,
    src: String,
    dst: String,
    recursive: bool,
    delete_extra: bool,
    overwrite: bool,
    dry_run: bool,
) -> Result<GuiRunResult, String> {
    let cancel_flag = state.flag.clone();

    tauri::async_runtime::spawn_blocking(move || {
        let mut args = vec!["--src".to_string(), src, "--dst".to_string(), dst];

        if recursive {
            args.push("--recursive".to_string());
        }

        if delete_extra {
            args.push("--delete-extra".to_string());
        }

        if overwrite {
            args.push("--overwrite".to_string());
        }

        if dry_run {
            args.push("--dry-run".to_string());
        }

        let label = if dry_run {
            "Previsualizando sincronización"
        } else {
            "Sincronizando carpetas"
        };

        run_action_with_gui_progress(app, cancel_flag, "sync", label, args)
    })
    .await
    .map_err(|e| format!("Error ejecutando sync: {e}"))?
}

#[tauri::command]
pub async fn validate_config(
    app: AppHandle,
    state: State<'_, CancelState>,
    path: String,
) -> Result<GuiRunResult, String> {
    let cancel_flag = state.flag.clone();

    tauri::async_runtime::spawn_blocking(move || {
        cancel_flag.store(false, Ordering::SeqCst);

        emit_progress(
            &app,
            GuiProgressPayload {
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

        let started_at = Instant::now();

        if cancel_flag.load(Ordering::SeqCst) {
            return Err("Operación cancelada".to_string());
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
            build_progress_payload(
                "Validando automatización",
                &Progress::new(1, 1).with_message("Validación completada"),
                started_at,
                true,
            ),
        );

        cancel_flag.store(false, Ordering::SeqCst);

        Ok(GuiRunResult {
            status: "SUCCESS".to_string(),
            logs,
        })
    })
    .await
    .map_err(|e| format!("Error validando config: {e}"))?
}

#[tauri::command]
pub async fn run_config(
    app: AppHandle,
    state: State<'_, CancelState>,
    path: String,
) -> Result<GuiRunResult, String> {
    let cancel_flag = state.flag.clone();

    tauri::async_runtime::spawn_blocking(move || {
        cancel_flag.store(false, Ordering::SeqCst);

        let action = actions::build_pipeline_from_config_file(&path).map_err(|e| e.to_string())?;
        let engine = Engine::new();

        let started_at = Instant::now();
        let app_for_progress = app.clone();

        emit_progress(
            &app,
            GuiProgressPayload {
                action: "Ejecutando automatización".to_string(),
                file: path.clone(),
                current: 0,
                total: 1,
                percent: 0.0,
                elapsed_seconds: 0,
                eta_seconds: None,
                done: false,
            },
        );

        let listener = std::sync::Arc::new(move |progress: Progress| {
            let payload =
                build_progress_payload("Ejecutando automatización", &progress, started_at, false);
            emit_progress(&app_for_progress, payload);
        });

        let out = engine.run_action_with_progress_and_cancel(
            action.as_ref(),
            listener,
            cancel_flag.clone(),
        );

        let final_progress = out
            .job
            .progress
            .clone()
            .unwrap_or_else(|| Progress::new(1, 1).with_message("Automatización finalizada"));

        emit_progress(
            &app,
            build_progress_payload(
                "Ejecutando automatización",
                &final_progress,
                started_at,
                true,
            ),
        );

        cancel_flag.store(false, Ordering::SeqCst);

        Ok(GuiRunResult {
            status: format_status(out.job.status),
            logs: out
                .logs
                .into_iter()
                .map(|l| format!("[{:?}] {}", l.level, l.message))
                .collect(),
        })
    })
    .await
    .map_err(|e| format!("Error ejecutando config: {e}"))?
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
