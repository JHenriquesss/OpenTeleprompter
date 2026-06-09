pub mod adapters;
mod commands;
pub mod domain;
pub mod persistence;
pub mod services;
mod tray;

use commands::import_export::ImportExportCommandHandler;
use commands::playback_state::PlaybackStateCommandHandler;
use commands::scripts::ScriptCommandHandler;
use commands::settings::SettingsCommandHandler;
use persistence::database::Database;
use persistence::migrations;
use persistence::playback_state_repository::PlaybackStateRepository;
use persistence::script_repository::ScriptRepository;
use persistence::settings_repository::SettingsRepository;
use services::import_export_service::ImportExportService;
use services::playback_state_service::PlaybackStateService;
use services::script_service::ScriptService;
use services::settings_service::SettingsService;
use std::sync::Arc;
use tauri::Emitter;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let app_dir = adapters::file_system::get_app_data_dir("openprompter-rs");
    let database = Arc::new(Database::new(&app_dir).expect("Failed to initialize database"));
    migrations::run_migrations(&database).expect("Failed to run migrations");

    let script_repo = ScriptRepository::new(Arc::clone(&database));
    let settings_repo = SettingsRepository::new(Arc::clone(&database));
    let playback_state_repo = PlaybackStateRepository::new(Arc::clone(&database));

    let script_service = ScriptService::new(script_repo);
    let settings_service = SettingsService::new(settings_repo);
    let playback_state_service = PlaybackStateService::new(playback_state_repo);

    let script_handler = ScriptCommandHandler {
        service: script_service,
    };
    let settings_handler = SettingsCommandHandler {
        service: settings_service,
    };
    let playback_state_handler = PlaybackStateCommandHandler {
        service: playback_state_service,
    };
    let import_export_handler = ImportExportCommandHandler {
        service: ImportExportService::new(ScriptService::new(ScriptRepository::new(Arc::clone(
            &database,
        )))),
    };

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .manage(script_handler)
        .manage(settings_handler)
        .manage(playback_state_handler)
        .manage(import_export_handler)
        .manage(commands::updater::PendingUpdate::default())
        .invoke_handler(tauri::generate_handler![
            commands::scripts::create_script,
            commands::scripts::update_script,
            commands::scripts::delete_script,
            commands::scripts::get_script,
            commands::scripts::list_scripts,
            commands::scripts::search_scripts,
            commands::scripts::duplicate_script,
            commands::settings::get_settings,
            commands::settings::update_settings,
            commands::settings::reset_settings,
            commands::playback_state::save_playback_state,
            commands::playback_state::load_playback_state,
            commands::playback_state::clear_playback_state,
            commands::import_export::import_script_from_txt,
            commands::import_export::export_script_to_txt,
            commands::import_export::open_file_dialog,
            commands::import_export::save_file_dialog,
            commands::import_export::read_text_file,
            commands::import_export::export_script_to_txt_file,
            commands::system::get_app_version,
            commands::system::set_pip,
            commands::updater::check_for_update,
            commands::updater::install_update,
        ])
        .setup(|app| {
            tray::build_tray(app.handle())?;
            Ok(())
        })
        .on_window_event(|window, event| {
            // Close (X) hides to the tray instead of quitting; the app keeps
            // running. Full exit is via the tray's Quit item. Emit a one-time
            // hint so the frontend can tell the user it is still running.
            // Only the main window hides to the tray on close. Secondary
            // windows (e.g. the PiP window) must close normally.
            if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                if window.label() == "main" {
                    api.prevent_close();
                    let _ = window.hide();
                    let _ = window.emit("close-to-tray", ());
                }
            }

            // Drag-and-drop import, handled in the backend (typed window event)
            // rather than via a JS listener. Supported files are imported through
            // the real service; the frontend is told to refresh via an event.
            if let tauri::WindowEvent::DragDrop(tauri::DragDropEvent::Drop { paths, .. }) = event {
                use tauri::Manager;
                let handler = window.state::<ImportExportCommandHandler>();
                let mut imported = 0u32;
                for path in paths {
                    if !adapters::document::is_supported(path) {
                        continue;
                    }
                    let name = path
                        .file_name()
                        .and_then(|s| s.to_str())
                        .unwrap_or("imported")
                        .to_string();
                    if let Ok(text) = adapters::document::extract_text(path) {
                        if handler.service.import_from_content(text, name).is_ok() {
                            imported += 1;
                        }
                    }
                }
                if imported > 0 {
                    let _ = window.emit("library-changed", imported);
                }
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
