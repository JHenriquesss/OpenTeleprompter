use tauri::{AppHandle, Manager, WebviewUrl, WebviewWindowBuilder};

#[tauri::command]
pub fn get_app_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

/// Open a small always-on-top picture-in-picture window that boots straight into
/// the prompter for `script_id` (the frontend reads `?pip=<id>`). If one is
/// already open, just bring it to the front.
#[tauri::command]
pub fn open_pip_window(app: AppHandle, script_id: String) -> Result<(), String> {
    if let Some(existing) = app.get_webview_window("pip") {
        let _ = existing.show();
        let _ = existing.set_focus();
        return Ok(());
    }

    // Pass the script id via an init script (runs before the page loads) rather
    // than the URL: `WebviewUrl::App` resolves the whole string as an asset path,
    // so `index.html?pip=x` / `#pip=x` 404s to a blank page. The frontend reads
    // `window.__PIP_SCRIPT_ID`.
    let init = format!("window.__PIP_SCRIPT_ID = {:?};", script_id);
    WebviewWindowBuilder::new(&app, "pip", WebviewUrl::App("index.html".into()))
        .title("Prompter (PiP)")
        .inner_size(560.0, 320.0)
        .min_inner_size(280.0, 160.0)
        .always_on_top(true)
        .resizable(true)
        .initialization_script(&init)
        .build()
        .map_err(|e| e.to_string())?;
    Ok(())
}
