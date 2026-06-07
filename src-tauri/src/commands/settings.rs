use crate::domain::settings::AppSettings;
use crate::services::settings_service::SettingsService;
use tauri::State;

pub struct SettingsCommandHandler {
    pub service: SettingsService,
}

#[tauri::command]
pub fn get_settings(handler: State<SettingsCommandHandler>) -> Result<AppSettings, String> {
    handler.service.get().map_err(|e| e.to_string())
}

#[tauri::command]
pub fn update_settings(
    handler: State<SettingsCommandHandler>,
    settings: AppSettings,
) -> Result<(), String> {
    handler.service.update(settings).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn reset_settings(handler: State<SettingsCommandHandler>) -> Result<AppSettings, String> {
    handler.service.reset().map_err(|e| e.to_string())
}
