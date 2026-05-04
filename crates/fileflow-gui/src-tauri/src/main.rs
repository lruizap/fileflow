#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use fileflow_actions as actions;
use fileflow_core::{Engine, JobStatus, LogEntry};
use serde::Serialize;

#[derive(Debug, Serialize)]
struct GuiRunResult {
    status: String,
    logs: Vec<String>,
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

fn run_action(action_name: &str, args: Vec<String>) -> Result<GuiRunResult, String> {
    let action = actions::build_action(action_name, &args).map_err(|e| e.to_string())?;
    let engine = Engine::new();
    let out = engine.run_action(action.as_ref());

    Ok(GuiRunResult {
        status: format_status(out.job.status),
        logs: format_logs(out.logs),
    })
}

#[tauri::command]
async fn run_echo() -> Result<GuiRunResult, String> {
    tauri::async_runtime::spawn_blocking(move || run_action("echo", vec![]))
        .await
        .map_err(|e| format!("Error ejecutando echo: {e}"))?
}

#[tauri::command]
async fn run_copy(src: String, dst: String, overwrite: bool) -> Result<GuiRunResult, String> {
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

        run_action("copy", args)
    })
    .await
    .map_err(|e| format!("Error ejecutando copy: {e}"))?
}

#[tauri::command]
async fn run_move(src: String, dst: String, overwrite: bool) -> Result<GuiRunResult, String> {
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

        run_action("move", args)
    })
    .await
    .map_err(|e| format!("Error ejecutando move: {e}"))?
}

#[tauri::command]
async fn run_sync(
    src: String,
    dst: String,
    recursive: bool,
    delete_extra: bool,
    overwrite: bool,
) -> Result<GuiRunResult, String> {
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

        run_action("sync", args)
    })
    .await
    .map_err(|e| format!("Error ejecutando sync: {e}"))?
}

#[tauri::command]
async fn validate_config(path: String) -> Result<GuiRunResult, String> {
    tauri::async_runtime::spawn_blocking(move || {
        let config = actions::load_pipeline_config(&path).map_err(|e| e.to_string())?;

        let mut logs = vec![
            "Config OK".to_string(),
            format!("Nombre: {}", config.name),
            format!("Steps: {}", config.steps.len()),
        ];

        for (index, step) in config.steps.iter().enumerate() {
            logs.push(format!("{}. {}", index + 1, step.action));
        }

        Ok(GuiRunResult {
            status: "SUCCESS".to_string(),
            logs,
        })
    })
    .await
    .map_err(|e| format!("Error validando config: {e}"))?
}

#[tauri::command]
async fn run_config(path: String) -> Result<GuiRunResult, String> {
    tauri::async_runtime::spawn_blocking(move || {
        let action = actions::build_pipeline_from_config_file(&path).map_err(|e| e.to_string())?;
        let engine = Engine::new();
        let out = engine.run_action(action.as_ref());

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
        .invoke_handler(tauri::generate_handler![
            run_echo,
            run_copy,
            run_move,
            run_sync,
            validate_config,
            run_config
        ])
        .run(tauri::generate_context!())
        .expect("error while running FileFlow GUI");
}