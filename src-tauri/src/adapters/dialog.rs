use tauri::AppHandle;
use tauri_plugin_dialog::DialogExt;

pub fn pick_file_to_open(app: &AppHandle) -> Option<String> {
    app.dialog()
        .file()
        .add_filter("Text Files", &["txt"])
        .blocking_pick_file()
        .and_then(|f| f.as_path().map(|p| p.to_string_lossy().to_string()))
}

pub fn pick_file_to_save(app: &AppHandle) -> Option<String> {
    app.dialog()
        .file()
        .add_filter("Text Files", &["txt"])
        .blocking_save_file()
        .and_then(|f| f.as_path().map(|p| p.to_string_lossy().to_string()))
}
