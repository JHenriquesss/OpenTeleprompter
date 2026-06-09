use tauri::AppHandle;
use tauri_plugin_dialog::DialogExt;

// We use the NON-blocking dialog API (`pick_file`/`save_file` with a callback)
// and wait on a channel. The callback runs on the main event loop, so the
// dialog actually shows; the caller (an async command on a worker thread) blocks
// on `recv` without freezing the UI. `blocking_pick_file` would deadlock when
// the command runs on the main thread.

pub fn pick_file_to_open(app: &AppHandle) -> Option<String> {
    let (tx, rx) = std::sync::mpsc::channel();
    app.dialog()
        .file()
        .add_filter(
            "Documents",
            crate::adapters::document::supported_extensions(),
        )
        .pick_file(move |f| {
            let _ = tx.send(f);
        });
    rx.recv()
        .ok()
        .flatten()
        .and_then(|f| f.as_path().map(|p| p.to_string_lossy().to_string()))
}

pub fn pick_file_to_save(app: &AppHandle) -> Option<String> {
    let (tx, rx) = std::sync::mpsc::channel();
    app.dialog()
        .file()
        .add_filter("Text Files", &["txt"])
        .save_file(move |f| {
            let _ = tx.send(f);
        });
    rx.recv()
        .ok()
        .flatten()
        .and_then(|f| f.as_path().map(|p| p.to_string_lossy().to_string()))
}
