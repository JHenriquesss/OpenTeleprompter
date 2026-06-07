use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

pub async fn invoke_tauri<A, R>(cmd: &str, args: A) -> Result<R, String>
where
    A: Serialize,
    R: DeserializeOwned,
{
    let args_json =
        serde_wasm_bindgen::to_value(&args).map_err(|e| format!("Serialize error: {}", e))?;
    let result = invoke(cmd, args_json).await;
    serde_wasm_bindgen::from_value(result).map_err(|e| format!("Invoke error: {}", e))
}

pub async fn invoke_tauri_unit<A>(cmd: &str, args: A) -> Result<(), String>
where
    A: Serialize,
{
    let args_json =
        serde_wasm_bindgen::to_value(&args).map_err(|e| format!("Serialize error: {}", e))?;
    invoke(cmd, args_json).await;
    Ok(())
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptData {
    pub id: String,
    pub title: String,
    pub content: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettingsData {
    pub font_size: f64,
    pub line_height: f64,
    pub text_width: f64,
    pub scroll_speed: f64,
    pub mirror_mode: bool,
    pub theme: String,
    pub countdown_seconds: u32,
    pub mirror_vertical: bool,
    pub reading_guide_enabled: bool,
}

pub async fn create_script(title: &str, content: &str) -> Result<ScriptData, String> {
    invoke_tauri::<_, ScriptData>(
        "create_script",
        serde_json::json!({ "title": title, "content": content }),
    )
    .await
}

pub async fn update_script(id: &str, title: &str, content: &str) -> Result<ScriptData, String> {
    invoke_tauri::<_, ScriptData>(
        "update_script",
        serde_json::json!({ "id": id, "title": title, "content": content }),
    )
    .await
}

pub async fn delete_script(id: &str) -> Result<(), String> {
    invoke_tauri_unit("delete_script", serde_json::json!({ "id": id })).await
}

pub async fn get_script(id: &str) -> Result<ScriptData, String> {
    invoke_tauri::<_, ScriptData>("get_script", serde_json::json!({ "id": id })).await
}

pub async fn list_scripts() -> Result<Vec<ScriptData>, String> {
    invoke_tauri("list_scripts", serde_json::json!({})).await
}

pub async fn search_scripts(query: &str) -> Result<Vec<ScriptData>, String> {
    invoke_tauri("search_scripts", serde_json::json!({ "query": query })).await
}

pub async fn duplicate_script(id: &str) -> Result<ScriptData, String> {
    invoke_tauri("duplicate_script", serde_json::json!({ "id": id })).await
}

pub async fn get_settings() -> Result<AppSettingsData, String> {
    invoke_tauri("get_settings", serde_json::json!({})).await
}

pub async fn update_settings(settings: &AppSettingsData) -> Result<(), String> {
    invoke_tauri_unit(
        "update_settings",
        serde_json::json!({ "settings": settings }),
    )
    .await
}

pub async fn reset_settings() -> Result<AppSettingsData, String> {
    invoke_tauri("reset_settings", serde_json::json!({})).await
}

pub async fn open_file_dialog() -> Result<Option<String>, String> {
    invoke_tauri("open_file_dialog", serde_json::json!({})).await
}

pub async fn save_file_dialog() -> Result<Option<String>, String> {
    invoke_tauri("save_file_dialog", serde_json::json!({})).await
}

pub async fn read_text_file(path: &str) -> Result<Option<String>, String> {
    invoke_tauri("read_text_file", serde_json::json!({ "path": path })).await
}

pub async fn export_script_to_txt_file(id: &str, path: &str) -> Result<(), String> {
    invoke_tauri_unit(
        "export_script_to_txt_file",
        serde_json::json!({ "id": id, "path": path }),
    )
    .await
}

pub async fn import_script_from_txt(content: &str, file_name: &str) -> Result<ScriptData, String> {
    invoke_tauri(
        "import_script_from_txt",
        serde_json::json!({ "content": content, "file_name": file_name }),
    )
    .await
}

pub async fn export_script_to_txt(id: &str) -> Result<(String, String), String> {
    invoke_tauri::<_, (String, String)>("export_script_to_txt", serde_json::json!({ "id": id }))
        .await
}

pub async fn get_app_version() -> Result<String, String> {
    invoke_tauri("get_app_version", serde_json::json!({})).await
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptPlaybackStateData {
    pub script_id: String,
    pub scroll_offset_px: f64,
    pub speed_multiplier: f64,
    pub font_size: Option<f64>,
    pub line_height: Option<f64>,
    pub mirror_mode: Option<bool>,
    pub mirror_vertical: Option<bool>,
    pub updated_at: String,
}

pub async fn save_playback_state(
    script_id: &str,
    scroll_offset_px: f64,
    speed_multiplier: f64,
    font_size: Option<f64>,
    line_height: Option<f64>,
    mirror_mode: Option<bool>,
    mirror_vertical: Option<bool>,
) -> Result<ScriptPlaybackStateData, String> {
    invoke_tauri(
        "save_playback_state",
        serde_json::json!({
            "scriptId": script_id,
            "scrollOffsetPx": scroll_offset_px,
            "speedMultiplier": speed_multiplier,
            "fontSize": font_size,
            "lineHeight": line_height,
            "mirrorMode": mirror_mode,
            "mirrorVertical": mirror_vertical,
        }),
    )
    .await
}

pub async fn load_playback_state(
    script_id: &str,
) -> Result<Option<ScriptPlaybackStateData>, String> {
    invoke_tauri(
        "load_playback_state",
        serde_json::json!({ "scriptId": script_id }),
    )
    .await
}

pub async fn clear_playback_state(script_id: &str) -> Result<(), String> {
    invoke_tauri_unit(
        "clear_playback_state",
        serde_json::json!({ "scriptId": script_id }),
    )
    .await
}
