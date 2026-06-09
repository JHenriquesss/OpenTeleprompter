//! In-memory [`AppApi`] implementation for WASM component tests.
//!
//! No Tauri, no `invoke`, no native dialogs. State lives in a `RefCell` so a
//! single `Rc<MockApi>` shared via Leptos context behaves like a tiny backend.
//! Tests configure it with builder methods, then assert against `call_log` and
//! the resulting in-memory state.

use std::cell::RefCell;
use std::collections::HashMap;

use async_trait::async_trait;

use super::app_api::AppApi;
use super::tauri_api::{AppSettingsData, ScriptData, ScriptPlaybackStateData, UpdateInfo};

/// Default settings mirroring the backend defaults (used by `reset_settings`
/// and as the initial mock state).
pub fn default_settings() -> AppSettingsData {
    AppSettingsData {
        font_size: 48.0,
        line_height: 1.5,
        text_width: 80.0,
        scroll_speed: 1.0,
        mirror_mode: false,
        theme: "dark".to_string(),
        countdown_seconds: 3,
        mirror_vertical: false,
        reading_guide_enabled: false,
    }
}

struct MockState {
    scripts: Vec<ScriptData>,
    settings: AppSettingsData,
    playback: HashMap<String, ScriptPlaybackStateData>,
    files: HashMap<String, String>,
    open_dialog_result: Option<String>,
    save_dialog_result: Option<String>,
    version: String,
    next_id: u32,
    /// When set, every command returns `Err(msg)` — for error-path tests.
    force_error: Option<String>,
    /// Specific command names that should fail (targeted error injection).
    fail_commands: Vec<String>,
    /// Method names invoked, in order, for assertions.
    call_log: Vec<String>,
    /// `(script_id, path)` recorded for each successful export.
    exports: Vec<(String, String)>,
    /// Update returned by `check_for_update` (`None` = already up to date).
    update: Option<UpdateInfo>,
}

pub struct MockApi {
    inner: RefCell<MockState>,
}

impl Default for MockApi {
    fn default() -> Self {
        Self::new()
    }
}

impl MockApi {
    pub fn new() -> Self {
        Self {
            inner: RefCell::new(MockState {
                scripts: Vec::new(),
                settings: default_settings(),
                playback: HashMap::new(),
                files: HashMap::new(),
                open_dialog_result: None,
                save_dialog_result: None,
                version: "1.1.3".to_string(),
                next_id: 1,
                force_error: None,
                fail_commands: Vec::new(),
                call_log: Vec::new(),
                exports: Vec::new(),
                update: None,
            }),
        }
    }

    // ---- builders -------------------------------------------------------

    pub fn with_scripts(self, scripts: Vec<ScriptData>) -> Self {
        {
            let mut s = self.inner.borrow_mut();
            s.scripts = scripts;
        }
        self
    }

    pub fn with_settings(self, settings: AppSettingsData) -> Self {
        self.inner.borrow_mut().settings = settings;
        self
    }

    pub fn with_playback(self, state: ScriptPlaybackStateData) -> Self {
        self.inner
            .borrow_mut()
            .playback
            .insert(state.script_id.clone(), state);
        self
    }

    pub fn with_file(self, path: &str, content: &str) -> Self {
        self.inner
            .borrow_mut()
            .files
            .insert(path.to_string(), content.to_string());
        self
    }

    pub fn with_open_dialog(self, path: &str) -> Self {
        self.inner.borrow_mut().open_dialog_result = Some(path.to_string());
        self
    }

    pub fn with_save_dialog(self, path: &str) -> Self {
        self.inner.borrow_mut().save_dialog_result = Some(path.to_string());
        self
    }

    /// Force every command to fail — for error-handling/toast tests.
    pub fn failing(self, msg: &str) -> Self {
        self.inner.borrow_mut().force_error = Some(msg.to_string());
        self
    }

    /// Configure `check_for_update` to report this update as available.
    pub fn with_update(self, info: UpdateInfo) -> Self {
        self.inner.borrow_mut().update = Some(info);
        self
    }

    /// Fail only the named command, leaving every other command working.
    /// Use this for flows that call several commands before the one under test
    /// (e.g. import = open_file_dialog → read_text_file → import_script_from_txt).
    pub fn fail_on(self, command: &str) -> Self {
        self.inner
            .borrow_mut()
            .fail_commands
            .push(command.to_string());
        self
    }

    // ---- introspection --------------------------------------------------

    pub fn call_log(&self) -> Vec<String> {
        self.inner.borrow().call_log.clone()
    }

    pub fn was_called(&self, method: &str) -> bool {
        self.inner.borrow().call_log.iter().any(|m| m == method)
    }

    pub fn was_not_called(&self, method: &str) -> bool {
        !self.was_called(method)
    }

    pub fn call_count(&self, method: &str) -> usize {
        self.inner
            .borrow()
            .call_log
            .iter()
            .filter(|m| *m == method)
            .count()
    }

    pub fn script_count(&self) -> usize {
        self.inner.borrow().scripts.len()
    }

    /// Snapshot of the current scripts (for asserting imported title/content).
    pub fn scripts(&self) -> Vec<ScriptData> {
        self.inner.borrow().scripts.clone()
    }

    /// `(script_id, path)` recorded for each successful export.
    pub fn exported(&self) -> Vec<(String, String)> {
        self.inner.borrow().exports.clone()
    }

    pub fn current_settings(&self) -> AppSettingsData {
        self.inner.borrow().settings.clone()
    }

    pub fn playback_count(&self) -> usize {
        self.inner.borrow().playback.len()
    }

    /// The most recently saved/stored playback state for a script (for asserting
    /// that exit/pause persisted the real scroll position, not 0).
    pub fn saved_playback(&self, script_id: &str) -> Option<ScriptPlaybackStateData> {
        self.inner.borrow().playback.get(script_id).cloned()
    }

    // ---- helpers --------------------------------------------------------

    fn log(&self, method: &str) {
        self.inner.borrow_mut().call_log.push(method.to_string());
    }

    /// Returns `Err` if a global failure is set, or if the most-recently-logged
    /// command (the caller) is in the targeted `fail_commands` list. Relies on
    /// every command calling `log()` immediately before `check_error()`.
    fn check_error(&self) -> Result<(), String> {
        let s = self.inner.borrow();
        if let Some(msg) = &s.force_error {
            return Err(msg.clone());
        }
        if let Some(cmd) = s.call_log.last() {
            if s.fail_commands.iter().any(|c| c == cmd) {
                return Err(format!("mock failure: {cmd}"));
            }
        }
        Ok(())
    }

    fn make_script(&self, title: &str, content: &str) -> ScriptData {
        let mut s = self.inner.borrow_mut();
        let id = format!("mock-{}", s.next_id);
        s.next_id += 1;
        ScriptData {
            id,
            title: title.to_string(),
            content: content.to_string(),
            created_at: "2026-01-01T00:00:00Z".to_string(),
            updated_at: "2026-01-01T00:00:00Z".to_string(),
        }
    }
}

#[async_trait(?Send)]
impl AppApi for MockApi {
    async fn create_script(&self, title: &str, content: &str) -> Result<ScriptData, String> {
        self.log("create_script");
        self.check_error()?;
        let script = self.make_script(title, content);
        self.inner.borrow_mut().scripts.push(script.clone());
        Ok(script)
    }

    async fn update_script(
        &self,
        id: &str,
        title: &str,
        content: &str,
    ) -> Result<ScriptData, String> {
        self.log("update_script");
        self.check_error()?;
        let mut s = self.inner.borrow_mut();
        match s.scripts.iter_mut().find(|sc| sc.id == id) {
            Some(sc) => {
                sc.title = title.to_string();
                sc.content = content.to_string();
                sc.updated_at = "2026-01-02T00:00:00Z".to_string();
                Ok(sc.clone())
            }
            None => Err(format!("Script not found: {}", id)),
        }
    }

    async fn delete_script(&self, id: &str) -> Result<(), String> {
        self.log("delete_script");
        self.check_error()?;
        self.inner.borrow_mut().scripts.retain(|sc| sc.id != id);
        Ok(())
    }

    async fn get_script(&self, id: &str) -> Result<ScriptData, String> {
        self.log("get_script");
        self.check_error()?;
        self.inner
            .borrow()
            .scripts
            .iter()
            .find(|sc| sc.id == id)
            .cloned()
            .ok_or_else(|| format!("Script not found: {}", id))
    }

    async fn list_scripts(&self) -> Result<Vec<ScriptData>, String> {
        self.log("list_scripts");
        self.check_error()?;
        Ok(self.inner.borrow().scripts.clone())
    }

    async fn search_scripts(&self, query: &str) -> Result<Vec<ScriptData>, String> {
        self.log("search_scripts");
        self.check_error()?;
        let q = query.to_lowercase();
        Ok(self
            .inner
            .borrow()
            .scripts
            .iter()
            .filter(|sc| {
                sc.title.to_lowercase().contains(&q) || sc.content.to_lowercase().contains(&q)
            })
            .cloned()
            .collect())
    }

    async fn duplicate_script(&self, id: &str) -> Result<ScriptData, String> {
        self.log("duplicate_script");
        self.check_error()?;
        let original = self
            .inner
            .borrow()
            .scripts
            .iter()
            .find(|sc| sc.id == id)
            .cloned()
            .ok_or_else(|| format!("Script not found: {}", id))?;
        let copy = self.make_script(&format!("{} (copy)", original.title), &original.content);
        self.inner.borrow_mut().scripts.push(copy.clone());
        Ok(copy)
    }

    async fn get_settings(&self) -> Result<AppSettingsData, String> {
        self.log("get_settings");
        self.check_error()?;
        Ok(self.inner.borrow().settings.clone())
    }

    async fn update_settings(&self, settings: &AppSettingsData) -> Result<(), String> {
        self.log("update_settings");
        self.check_error()?;
        self.inner.borrow_mut().settings = settings.clone();
        Ok(())
    }

    async fn reset_settings(&self) -> Result<AppSettingsData, String> {
        self.log("reset_settings");
        self.check_error()?;
        let defaults = default_settings();
        self.inner.borrow_mut().settings = defaults.clone();
        Ok(defaults)
    }

    async fn open_file_dialog(&self) -> Result<Option<String>, String> {
        self.log("open_file_dialog");
        self.check_error()?;
        Ok(self.inner.borrow().open_dialog_result.clone())
    }

    async fn save_file_dialog(&self) -> Result<Option<String>, String> {
        self.log("save_file_dialog");
        self.check_error()?;
        Ok(self.inner.borrow().save_dialog_result.clone())
    }

    async fn read_text_file(&self, path: &str) -> Result<Option<String>, String> {
        self.log("read_text_file");
        self.check_error()?;
        Ok(self.inner.borrow().files.get(path).cloned())
    }

    async fn export_script_to_txt_file(&self, id: &str, path: &str) -> Result<(), String> {
        self.log("export_script_to_txt_file");
        self.check_error()?;
        self.inner
            .borrow_mut()
            .exports
            .push((id.to_string(), path.to_string()));
        Ok(())
    }

    async fn import_script_from_txt(
        &self,
        content: &str,
        file_name: &str,
    ) -> Result<ScriptData, String> {
        self.log("import_script_from_txt");
        self.check_error()?;
        let title = file_name
            .strip_suffix(".txt")
            .unwrap_or(file_name)
            .to_string();
        let script = self.make_script(&title, content);
        self.inner.borrow_mut().scripts.push(script.clone());
        Ok(script)
    }

    async fn export_script_to_txt(&self, id: &str) -> Result<(String, String), String> {
        self.log("export_script_to_txt");
        self.check_error()?;
        let script = self
            .inner
            .borrow()
            .scripts
            .iter()
            .find(|sc| sc.id == id)
            .cloned()
            .ok_or_else(|| format!("Script not found: {}", id))?;
        Ok((format!("{}.txt", script.title), script.content))
    }

    async fn get_app_version(&self) -> Result<String, String> {
        self.log("get_app_version");
        self.check_error()?;
        Ok(self.inner.borrow().version.clone())
    }

    async fn set_pip(&self, _enabled: bool) -> Result<(), String> {
        self.log("set_pip");
        self.check_error()?;
        Ok(())
    }

    async fn check_for_update(&self) -> Result<Option<UpdateInfo>, String> {
        self.log("check_for_update");
        self.check_error()?;
        Ok(self.inner.borrow().update.clone())
    }

    async fn install_update(&self) -> Result<(), String> {
        self.log("install_update");
        self.check_error()?;
        Ok(())
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
        self.log("save_playback_state");
        self.check_error()?;
        let state = ScriptPlaybackStateData {
            script_id: script_id.to_string(),
            scroll_offset_px,
            speed_multiplier,
            font_size,
            line_height,
            mirror_mode,
            mirror_vertical,
            updated_at: "2026-01-03T00:00:00Z".to_string(),
        };
        self.inner
            .borrow_mut()
            .playback
            .insert(script_id.to_string(), state.clone());
        Ok(state)
    }

    async fn load_playback_state(
        &self,
        script_id: &str,
    ) -> Result<Option<ScriptPlaybackStateData>, String> {
        self.log("load_playback_state");
        self.check_error()?;
        Ok(self.inner.borrow().playback.get(script_id).cloned())
    }

    async fn clear_playback_state(&self, script_id: &str) -> Result<(), String> {
        self.log("clear_playback_state");
        self.check_error()?;
        self.inner.borrow_mut().playback.remove(script_id);
        Ok(())
    }
}
