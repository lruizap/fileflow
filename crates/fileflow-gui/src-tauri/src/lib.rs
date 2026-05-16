pub mod commands;
pub mod jobs;
pub mod progress;

use jobs::JobManager;

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .manage(JobManager::default())
        .invoke_handler(tauri::generate_handler![
            commands::actions::run_echo,
            commands::actions::run_copy,
            commands::actions::run_move,
            commands::actions::run_sync,
            commands::actions::run_watch,
            commands::actions::validate_config,
            commands::actions::run_config,
            commands::queue::get_queue_state,
            commands::queue::set_concurrency_limit,
            commands::queue::update_job_priority,
            commands::queue::cancel_job,
            commands::pipeline_files::save_pipeline_json,
            commands::pipeline_files::read_pipeline_json
        ])
        .run(tauri::generate_context!())
        .expect("error while running FileFlow");
}
