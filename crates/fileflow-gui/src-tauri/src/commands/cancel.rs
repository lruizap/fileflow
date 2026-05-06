use std::sync::atomic::Ordering;

use tauri::State;

use crate::state::CancelState;

#[tauri::command]
pub fn cancel_current_job(state: State<CancelState>) -> Result<(), String> {
    state.flag.store(true, Ordering::SeqCst);
    Ok(())
}