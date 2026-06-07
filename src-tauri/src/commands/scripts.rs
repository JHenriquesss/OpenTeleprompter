use crate::domain::script::Script;
use crate::services::script_service::ScriptService;
use tauri::State;

pub struct ScriptCommandHandler {
    pub service: ScriptService,
}

#[tauri::command]
pub fn create_script(
    handler: State<ScriptCommandHandler>,
    title: String,
    content: String,
) -> Result<Script, String> {
    handler
        .service
        .create(title, content)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn update_script(
    handler: State<ScriptCommandHandler>,
    id: String,
    title: String,
    content: String,
) -> Result<Script, String> {
    handler
        .service
        .update(id, title, content)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn delete_script(handler: State<ScriptCommandHandler>, id: String) -> Result<(), String> {
    handler.service.delete(id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_script(handler: State<ScriptCommandHandler>, id: String) -> Result<Script, String> {
    handler.service.get_by_id(id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn list_scripts(handler: State<ScriptCommandHandler>) -> Result<Vec<Script>, String> {
    handler.service.list_all().map_err(|e| e.to_string())
}

#[tauri::command]
pub fn search_scripts(
    handler: State<ScriptCommandHandler>,
    query: String,
) -> Result<Vec<Script>, String> {
    handler.service.search(query).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn duplicate_script(
    handler: State<ScriptCommandHandler>,
    id: String,
) -> Result<Script, String> {
    handler.service.duplicate(id).map_err(|e| e.to_string())
}
