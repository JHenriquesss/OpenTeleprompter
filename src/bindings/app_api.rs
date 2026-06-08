//! Frontend API abstraction.
//!
//! Components depend on the [`AppApi`] trait instead of the concrete Tauri
//! `invoke` bindings. Production wires in [`RealTauriApi`] (which delegates to
//! the `tauri_api` invoke wrappers); WASM component tests wire in
//! [`crate::bindings::mock_api::MockApi`].
//!
//! The trait is `?Send` because the frontend runs single-threaded in WASM.

use async_trait::async_trait;

use super::tauri_api::{self, AppSettingsData, ScriptData, ScriptPlaybackStateData, UpdateInfo};

/// Mockable surface over every Tauri command the frontend uses.
#[async_trait(?Send)]
pub trait AppApi {
    async fn create_script(&self, title: &str, content: &str) -> Result<ScriptData, String>;
    async fn update_script(
        &self,
        id: &str,
        title: &str,
        content: &str,
    ) -> Result<ScriptData, String>;
    async fn delete_script(&self, id: &str) -> Result<(), String>;
    async fn get_script(&self, id: &str) -> Result<ScriptData, String>;
    async fn list_scripts(&self) -> Result<Vec<ScriptData>, String>;
    async fn search_scripts(&self, query: &str) -> Result<Vec<ScriptData>, String>;
    async fn duplicate_script(&self, id: &str) -> Result<ScriptData, String>;

    async fn get_settings(&self) -> Result<AppSettingsData, String>;
    async fn update_settings(&self, settings: &AppSettingsData) -> Result<(), String>;
    async fn reset_settings(&self) -> Result<AppSettingsData, String>;

    async fn open_file_dialog(&self) -> Result<Option<String>, String>;
    async fn save_file_dialog(&self) -> Result<Option<String>, String>;
    async fn read_text_file(&self, path: &str) -> Result<Option<String>, String>;
    async fn export_script_to_txt_file(&self, id: &str, path: &str) -> Result<(), String>;
    async fn import_script_from_txt(
        &self,
        content: &str,
        file_name: &str,
    ) -> Result<ScriptData, String>;
    async fn export_script_to_txt(&self, id: &str) -> Result<(String, String), String>;

    async fn get_app_version(&self) -> Result<String, String>;

    /// Open the always-on-top picture-in-picture prompter window for a script.
    async fn open_pip_window(&self, script_id: &str) -> Result<(), String>;

    /// Check for an available update. `Ok(None)` = already up to date.
    async fn check_for_update(&self) -> Result<Option<UpdateInfo>, String>;
    /// Download + install the pending update and relaunch.
    async fn install_update(&self) -> Result<(), String>;

    #[allow(clippy::too_many_arguments)]
    async fn save_playback_state(
        &self,
        script_id: &str,
        scroll_offset_px: f64,
        speed_multiplier: f64,
        font_size: Option<f64>,
        line_height: Option<f64>,
        mirror_mode: Option<bool>,
        mirror_vertical: Option<bool>,
    ) -> Result<ScriptPlaybackStateData, String>;
    async fn load_playback_state(
        &self,
        script_id: &str,
    ) -> Result<Option<ScriptPlaybackStateData>, String>;
    async fn clear_playback_state(&self, script_id: &str) -> Result<(), String>;
}

/// Production implementation: forwards to the real Tauri `invoke` wrappers.
///
/// Behavior is byte-identical to calling the `tauri_api` free functions
/// directly — this type only adds the trait dispatch layer.
pub struct RealTauriApi;

#[async_trait(?Send)]
impl AppApi for RealTauriApi {
    async fn create_script(&self, title: &str, content: &str) -> Result<ScriptData, String> {
        tauri_api::create_script(title, content).await
    }
    async fn update_script(
        &self,
        id: &str,
        title: &str,
        content: &str,
    ) -> Result<ScriptData, String> {
        tauri_api::update_script(id, title, content).await
    }
    async fn delete_script(&self, id: &str) -> Result<(), String> {
        tauri_api::delete_script(id).await
    }
    async fn get_script(&self, id: &str) -> Result<ScriptData, String> {
        tauri_api::get_script(id).await
    }
    async fn list_scripts(&self) -> Result<Vec<ScriptData>, String> {
        tauri_api::list_scripts().await
    }
    async fn search_scripts(&self, query: &str) -> Result<Vec<ScriptData>, String> {
        tauri_api::search_scripts(query).await
    }
    async fn duplicate_script(&self, id: &str) -> Result<ScriptData, String> {
        tauri_api::duplicate_script(id).await
    }

    async fn get_settings(&self) -> Result<AppSettingsData, String> {
        tauri_api::get_settings().await
    }
    async fn update_settings(&self, settings: &AppSettingsData) -> Result<(), String> {
        tauri_api::update_settings(settings).await
    }
    async fn reset_settings(&self) -> Result<AppSettingsData, String> {
        tauri_api::reset_settings().await
    }

    async fn open_file_dialog(&self) -> Result<Option<String>, String> {
        tauri_api::open_file_dialog().await
    }
    async fn save_file_dialog(&self) -> Result<Option<String>, String> {
        tauri_api::save_file_dialog().await
    }
    async fn read_text_file(&self, path: &str) -> Result<Option<String>, String> {
        tauri_api::read_text_file(path).await
    }
    async fn export_script_to_txt_file(&self, id: &str, path: &str) -> Result<(), String> {
        tauri_api::export_script_to_txt_file(id, path).await
    }
    async fn import_script_from_txt(
        &self,
        content: &str,
        file_name: &str,
    ) -> Result<ScriptData, String> {
        tauri_api::import_script_from_txt(content, file_name).await
    }
    async fn export_script_to_txt(&self, id: &str) -> Result<(String, String), String> {
        tauri_api::export_script_to_txt(id).await
    }

    async fn get_app_version(&self) -> Result<String, String> {
        tauri_api::get_app_version().await
    }

    async fn open_pip_window(&self, script_id: &str) -> Result<(), String> {
        tauri_api::open_pip_window(script_id).await
    }

    async fn check_for_update(&self) -> Result<Option<UpdateInfo>, String> {
        tauri_api::check_for_update().await
    }
    async fn install_update(&self) -> Result<(), String> {
        tauri_api::install_update().await
    }

    async fn save_playback_state(
        &self,
        script_id: &str,
        scroll_offset_px: f64,
        speed_multiplier: f64,
        font_size: Option<f64>,
        line_height: Option<f64>,
        mirror_mode: Option<bool>,
        mirror_vertical: Option<bool>,
    ) -> Result<ScriptPlaybackStateData, String> {
        tauri_api::save_playback_state(
            script_id,
            scroll_offset_px,
            speed_multiplier,
            font_size,
            line_height,
            mirror_mode,
            mirror_vertical,
        )
        .await
    }
    async fn load_playback_state(
        &self,
        script_id: &str,
    ) -> Result<Option<ScriptPlaybackStateData>, String> {
        tauri_api::load_playback_state(script_id).await
    }
    async fn clear_playback_state(&self, script_id: &str) -> Result<(), String> {
        tauri_api::clear_playback_state(script_id).await
    }
}
