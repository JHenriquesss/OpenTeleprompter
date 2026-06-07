mod adapters;
mod commands;
mod domain;
mod persistence;
mod services;

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
        .manage(script_handler)
        .manage(settings_handler)
        .manage(playback_state_handler)
        .manage(import_export_handler)
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
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
