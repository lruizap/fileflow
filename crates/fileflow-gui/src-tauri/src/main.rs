#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use std::time::Instant;

use fileflow_actions as actions;
use fileflow_core::{Engine, JobStatus, LogEntry, Progress};
use serde::Serialize;
use tauri::{AppHandle, Emitter, State};

#[derive(Clone)]
struct CancelState {
    flag: Arc<AtomicBool>,
}

#[derive(Debug, Serialize)]
struct GuiRunResult {
    status: String,
    logs: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct GuiProgressPayload {
    action: String,
    file: String,
    current: u64,
    total: u64,
    percent: f64,
    elapsed_seconds: u64,
    eta_seconds: Option<u64>,
    done: bool,
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
    logs
        .into_iter()
        .map(|l| format!("[{:?}] {}", l.level, l.message))
        .collect()
}

fn emit_progress(app: &AppHandle, payload: GuiProgressPayload) {
    let _ = app.emit("fileflow-progress", payload);
}

fn build_progress_payload(
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

fn run_action_with_gui_progress(
    app: AppHandle,
    cancel_flag: Arc<AtomicBool>,
    action_name: &str,
    action_label: &str,
    args: Vec<String>,
) -> Result<GuiRunResult, String> {
    cancel_flag.store(false, Ordering::SeqCst);

    let action = actions::build_action(action_name, &args)
        .map_err(|e| e.to_string())?;

    let engine = Engine::new();

    let started_at = Instant::now();

    let app_for_progress = app.clone();
    let label_for_progress = action_label.to_string();

    emit_progress(
        &app,
        GuiProgressPayload {
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
        let payload = build_progress_payload(
            &label_for_progress,
            &progress,
            started_at,
            false,
        );

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
        .unwrap_or_else(|| {
            Progress::new(1, 1)
                .with_message("Operación finalizada")
        });

    emit_progress(
        &app,
        build_progress_payload(
            action_label,
            &final_progress,
            started_at,
            true,
        ),
    );

    cancel_flag.store(false, Ordering::SeqCst);

    Ok(GuiRunResult {
        status: format_status(out.job.status),
        logs: format_logs(out.logs),
    })
}

#[tauri::command]
fn cancel_current_job(state: State<CancelState>) -> Result<(), String> {
    state.flag.store(true, Ordering::SeqCst);
    Ok(())
}

#[tauri::command]
fn save_pipeline_json(
    path: String,
    content: String,
) -> Result<(), String> {
    fs::write(&path, content)
        .map_err(|e| format!("No se pudo guardar '{}': {}", path, e))
}

#[tauri::command]
fn read_pipeline_json(path: String) -> Result<String, String> {
    fs::read_to_string(&path)
        .map_err(|e| format!("No se pudo leer '{}': {}", path, e))
}

#[tauri::command]
async fn run_echo(
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
async fn run_copy(
    app: AppHandle,
    state: State<'_, CancelState>,
    src: String,
    dst: String,
    overwrite: bool,
) -> Result<GuiRunResult, String> {
    let cancel_flag = state.flag.clone();

    tauri::async_runtime::spawn_blocking(move || {
        let mut args = vec![
            "--src".to_string(),
            src,
            "--dst".to_string(),
            dst,
        ];

        if overwrite {
            args.push("--overwrite".to_string());
        }

        run_action_with_gui_progress(
            app,
            cancel_flag,
            "copy",
            "Copiando archivo",
            args,
        )
    })
    .await
    .map_err(|e| format!("Error ejecutando copy: {e}"))?
}

#[tauri::command]
async fn run_move(
    app: AppHandle,
    state: State<'_, CancelState>,
    src: String,
    dst: String,
    overwrite: bool,
) -> Result<GuiRunResult, String> {
    let cancel_flag = state.flag.clone();

    tauri::async_runtime::spawn_blocking(move || {
        let mut args = vec![
            "--src".to_string(),
            src,
            "--dst".to_string(),
            dst,
        ];

        if overwrite {
            args.push("--overwrite".to_string());
        }

        run_action_with_gui_progress(
            app,
            cancel_flag,
            "move",
            "Moviendo archivo",
            args,
        )
    })
    .await
    .map_err(|e| format!("Error ejecutando move: {e}"))?
}

#[tauri::command]
async fn run_sync(
    app: AppHandle,
    state: State<'_, CancelState>,
    src: String,
    dst: String,
    recursive: bool,
    delete_extra: bool,
    overwrite: bool,
) -> Result<GuiRunResult, String> {
    let cancel_flag = state.flag.clone();

    tauri::async_runtime::spawn_blocking(move || {
        let mut args = vec![
            "--src".to_string(),
            src,
            "--dst".to_string(),
            dst,
        ];

        if recursive {
            args.push("--recursive".to_string());
        }

        if delete_extra {
            args.push("--delete-extra".to_string());
        }

        if overwrite {
            args.push("--overwrite".to_string());
        }

        run_action_with_gui_progress(
            app,
            cancel_flag,
            "sync",
            "Sincronizando carpetas",
            args,
        )
    })
    .await
    .map_err(|e| format!("Error ejecutando sync: {e}"))?
}

#[tauri::command]
async fn validate_config(
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

        let config = actions::load_pipeline_config(&path)
            .map_err(|e| e.to_string())?;

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
                &Progress::new(1, 1)
                    .with_message("Validación completada"),
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
async fn run_config(
    app: AppHandle,
    state: State<'_, CancelState>,
    path: String,
) -> Result<GuiRunResult, String> {
    let cancel_flag = state.flag.clone();

    tauri::async_runtime::spawn_blocking(move || {
        cancel_flag.store(false, Ordering::SeqCst);

        let action = actions::build_pipeline_from_config_file(&path)
            .map_err(|e| e.to_string())?;

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

        let listener = Arc::new(move |progress: Progress| {
            let payload = build_progress_payload(
                "Ejecutando automatización",
                &progress,
                started_at,
                false,
            );

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
            .unwrap_or_else(|| {
                Progress::new(1, 1)
                    .with_message("Automatización finalizada")
            });

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
            logs: format_logs(out.logs),
        })
    })
    .await
    .map_err(|e| format!("Error ejecutando config: {e}"))?
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .manage(CancelState {
            flag: Arc::new(AtomicBool::new(false)),
        })
        .invoke_handler(tauri::generate_handler![
            run_echo,
            run_copy,
            run_move,
            run_sync,
            validate_config,
            run_config,
            cancel_current_job,
            save_pipeline_json,
            read_pipeline_json
        ])
        .run(tauri::generate_context!())
        .expect("error while running FileFlow GUI");
}