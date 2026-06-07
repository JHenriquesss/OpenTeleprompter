//! Self-update commands backed by `tauri-plugin-updater`.
//!
//! Two-step flow so the frontend can prompt the user between detection and
//! installation:
//! 1. [`check_for_update`] queries the configured endpoint. If an update exists
//!    it stashes the (non-serializable) [`Update`] handle in [`PendingUpdate`]
//!    managed state and returns lightweight [`UpdateInfo`] metadata.
//! 2. [`install_update`] takes the stashed handle, downloads + installs it, then
//!    relaunches the app.
//!
//! Installation is only ever triggered by an explicit frontend call — there is
//! no silent auto-install.

use std::sync::Mutex;

use serde::Serialize;
use tauri::{AppHandle, State};
use tauri_plugin_updater::{Update, UpdaterExt};

/// Holds the [`Update`] returned by a successful check until the user installs.
/// `Update` is not `Serialize`, so it cannot cross the IPC boundary — we keep it
/// here and only hand the frontend the [`UpdateInfo`] summary.
#[derive(Default)]
pub struct PendingUpdate(pub Mutex<Option<Update>>);

/// Serializable summary of an available update (sent to the frontend).
#[derive(Debug, Clone, Serialize)]
pub struct UpdateInfo {
    pub version: String,
    pub current_version: String,
    pub notes: Option<String>,
    pub date: Option<String>,
}

#[tauri::command]
pub async fn check_for_update(
    app: AppHandle,
    pending: State<'_, PendingUpdate>,
) -> Result<Option<UpdateInfo>, String> {
    let updater = app.updater().map_err(|e| e.to_string())?;
    match updater.check().await {
        Ok(Some(update)) => {
            let info = UpdateInfo {
                version: update.version.clone(),
                current_version: update.current_version.clone(),
                notes: update.body.clone(),
                date: update.date.map(|d| d.to_string()),
            };
            *pending.0.lock().expect("PendingUpdate mutex poisoned") = Some(update);
            Ok(Some(info))
        }
        Ok(None) => Ok(None),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
pub async fn install_update(
    app: AppHandle,
    pending: State<'_, PendingUpdate>,
) -> Result<(), String> {
    // Take the stashed handle (drops the guard before any await — `Update` must
    // not be held across the download future).
    let update = pending
        .0
        .lock()
        .expect("PendingUpdate mutex poisoned")
        .take()
        .ok_or_else(|| "No pending update. Run check_for_update first.".to_string())?;

    update
        .download_and_install(|_chunk, _total| {}, || {})
        .await
        .map_err(|e| e.to_string())?;

    // Relaunch into the freshly installed version. `restart` diverges (`!`).
    app.restart();
}
