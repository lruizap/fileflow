use std::fs;

#[tauri::command]
pub fn save_pipeline_json(path: String, content: String) -> Result<(), String> {
    fs::write(&path, content).map_err(|e| format!("No se pudo guardar '{}': {}", path, e))
}

#[tauri::command]
pub fn read_pipeline_json(path: String) -> Result<String, String> {
    fs::read_to_string(&path).map_err(|e| format!("No se pudo leer '{}': {}", path, e))
}
