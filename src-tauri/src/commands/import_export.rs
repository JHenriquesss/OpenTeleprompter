use crate::adapters;
use crate::domain::script::Script;
use crate::services::import_export_service::ImportExportService;
use std::path::Path;
use tauri::AppHandle;
use tauri::State;

pub struct ImportExportCommandHandler {
    pub service: ImportExportService,
}

#[tauri::command]
pub fn import_script_from_txt(
    handler: State<ImportExportCommandHandler>,
    content: String,
    file_name: String,
) -> Result<Script, String> {
    handler
        .service
        .import_from_content(content, file_name)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn export_script_to_txt(
    handler: State<ImportExportCommandHandler>,
    id: String,
) -> Result<(String, String), String> {
    handler
        .service
        .export_content(id)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn open_file_dialog(app: AppHandle) -> Result<Option<String>, String> {
    Ok(adapters::dialog::pick_file_to_open(&app))
}

#[tauri::command]
pub async fn save_file_dialog(app: AppHandle) -> Result<Option<String>, String> {
    Ok(adapters::dialog::pick_file_to_save(&app))
}

#[tauri::command]
pub async fn read_text_file(path: String) -> Result<Option<String>, String> {
    let p = Path::new(&path);
    if p.exists() {
        adapters::file_system::read_text_file(p)
            .map(Some)
            .map_err(|e| e.to_string())
    } else {
        Ok(None)
    }
}

#[tauri::command]
pub fn export_script_to_txt_file(
    handler: State<ImportExportCommandHandler>,
    id: String,
    path: String,
) -> Result<(), String> {
    let (title, content) = handler
        .service
        .export_content(id)
        .map_err(|e| e.to_string())?;
    let save_path = if path.ends_with(".txt") {
        Path::new(&path).to_path_buf()
    } else {
        Path::new(&path).join(format!("{}.txt", title))
    };
    adapters::file_system::write_text_file(&save_path, &content).map_err(|e| e.to_string())
}
