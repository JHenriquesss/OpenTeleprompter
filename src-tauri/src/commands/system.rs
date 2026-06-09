use tauri::{AppHandle, LogicalSize, Manager};

#[tauri::command]
pub fn get_app_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

/// Picture-in-picture mode: pin the main window as a small always-on-top float
/// (it is already showing the prompter), or restore it to normal size.
///
/// A separate webview window was tried first but runtime-created windows did not
/// load the bundled app (they came up `about:blank`). Pinning the existing,
/// already-rendered window is reliable and gives the same "float over another
/// app" result.
#[tauri::command]
pub fn set_pip(app: AppHandle, enabled: bool) -> Result<(), String> {
    let win = app
        .get_webview_window("main")
        .ok_or_else(|| "main window not found".to_string())?;
    if enabled {
        let _ = win.unmaximize();
        win.set_size(LogicalSize::new(560.0, 320.0))
            .map_err(|e| e.to_string())?;
        win.set_always_on_top(true).map_err(|e| e.to_string())?;
    } else {
        win.set_always_on_top(false).map_err(|e| e.to_string())?;
        win.set_size(LogicalSize::new(1280.0, 800.0))
            .map_err(|e| e.to_string())?;
        let _ = win.center();
    }
    Ok(())
}
