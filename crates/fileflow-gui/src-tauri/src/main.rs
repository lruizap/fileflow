#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod progress;
mod state;

use state::CancelState;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .manage(CancelState::default())
        .invoke_handler(tauri::generate_handler![
            commands::actions::run_echo,
            commands::actions::run_copy,
            commands::actions::run_move,
            commands::actions::run_sync,
            commands::actions::validate_config,
            commands::actions::run_config,
            commands::cancel::cancel_current_job,
            commands::pipeline_files::save_pipeline_json,
            commands::pipeline_files::read_pipeline_json
        ])
        .run(tauri::generate_context!())
        .expect("error while running FileFlow GUI");
}