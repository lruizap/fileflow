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

#[tauri::command]
fn run_echo() -> Result<GuiRunResult, String> {
    let action = actions::build_action("echo", &[]).map_err(|e| e.to_string())?;
    let engine = Engine::new();
    let out = engine.run_action(action.as_ref());

    Ok(GuiRunResult {
        status: format_status(out.job.status),
        logs: format_logs(out.logs),
    })
}

#[tauri::command]
fn run_copy(src: String, dst: String, overwrite: bool) -> Result<GuiRunResult, String> {
    let mut args = vec![
        "--src".to_string(),
        src,
        "--dst".to_string(),
        dst,
    ];

    if overwrite {
        args.push("--overwrite".to_string());
    }

    let action = actions::build_action("copy", &args).map_err(|e| e.to_string())?;
    let engine = Engine::new();
    let out = engine.run_action(action.as_ref());

    Ok(GuiRunResult {
        status: format_status(out.job.status),
        logs: format_logs(out.logs),
    })
}

#[tauri::command]
fn run_move(src: String, dst: String, overwrite: bool) -> Result<GuiRunResult, String> {
    let mut args = vec![
        "--src".to_string(),
        src,
        "--dst".to_string(),
        dst,
    ];

    if overwrite {
        args.push("--overwrite".to_string());
    }

    let action = actions::build_action("move", &args).map_err(|e| e.to_string())?;
    let engine = Engine::new();
    let out = engine.run_action(action.as_ref());

    Ok(GuiRunResult {
        status: format_status(out.job.status),
        logs: format_logs(out.logs),
    })
}

#[tauri::command]
fn run_sync(
    src: String,
    dst: String,
    recursive: bool,
    delete_extra: bool,
    overwrite: bool,
) -> Result<GuiRunResult, String> {
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

    let action = actions::build_action("sync", &args).map_err(|e| e.to_string())?;
    let engine = Engine::new();
    let out = engine.run_action(action.as_ref());

    Ok(GuiRunResult {
        status: format_status(out.job.status),
        logs: format_logs(out.logs),
    })
}

#[tauri::command]
fn validate_config(path: String) -> Result<GuiRunResult, String> {
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
}

#[tauri::command]
fn run_config(path: String) -> Result<GuiRunResult, String> {
    let action = actions::build_pipeline_from_config_file(&path).map_err(|e| e.to_string())?;
    let engine = Engine::new();
    let out = engine.run_action(action.as_ref());

    Ok(GuiRunResult {
        status: format_status(out.job.status),
        logs: format_logs(out.logs),
    })
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