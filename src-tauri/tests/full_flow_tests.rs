//! Full end-to-end flow tests against the REAL application stack.
//!
//! Unlike the other integration tests (which re-implement SQL inline), these
//! construct the actual `Database`, run the real migrations, and drive every
//! `*Service` exactly as the Tauri command handlers do in production. A real
//! on-disk SQLite file is used (temp dir, auto-cleaned), so this exercises the
//! same code path the shipped binary runs — repositories, services, domain
//! validation, JSON (de)serialization, and FK cascade.
//!
//! This is the layer that was previously untested end-to-end: every prior
//! "integration" test bypassed the app's own code.

use openprompter_rs_tauri::domain::settings::{AppSettings, Theme};
use openprompter_rs_tauri::persistence::database::Database;
use openprompter_rs_tauri::persistence::migrations;
use openprompter_rs_tauri::persistence::playback_state_repository::PlaybackStateRepository;
use openprompter_rs_tauri::persistence::script_repository::ScriptRepository;
use openprompter_rs_tauri::persistence::settings_repository::SettingsRepository;
use openprompter_rs_tauri::services::import_export_service::ImportExportService;
use openprompter_rs_tauri::services::playback_state_service::PlaybackStateService;
use openprompter_rs_tauri::services::script_service::ScriptService;
use openprompter_rs_tauri::services::settings_service::SettingsService;
use std::sync::Arc;
use tempfile::TempDir;

/// Build the full real service stack on a fresh temp database.
struct App {
    _dir: TempDir, // keep alive so the db file is not removed mid-test
    scripts: ScriptService,
    settings: SettingsService,
    playback: PlaybackStateService,
    import_export: ImportExportService,
}

fn app() -> App {
    let dir = TempDir::new().unwrap();
    let db = Arc::new(Database::new(dir.path()).expect("db init"));
    migrations::run_migrations(&db).expect("migrations");

    let scripts = ScriptService::new(ScriptRepository::new(Arc::clone(&db)));
    let settings = SettingsService::new(SettingsRepository::new(Arc::clone(&db)));
    let playback = PlaybackStateService::new(PlaybackStateRepository::new(Arc::clone(&db)));
    let import_export =
        ImportExportService::new(ScriptService::new(ScriptRepository::new(Arc::clone(&db))));

    App {
        _dir: dir,
        scripts,
        settings,
        playback,
        import_export,
    }
}

// ---------------------------------------------------------------------------
// Script library CRUD — mirrors the New / Save / Delete / Search buttons.
// ---------------------------------------------------------------------------

#[test]
fn script_crud_full_cycle() {
    let app = app();

    // Create (New script button + Save).
    let s = app
        .scripts
        .create("My Speech".into(), "Hello world".into())
        .unwrap();
    assert_eq!(s.title, "My Speech");
    assert_eq!(s.content, "Hello world");
    assert!(!s.id.is_empty());

    // List shows it.
    let all = app.scripts.list_all().unwrap();
    assert_eq!(all.len(), 1);

    // Get by id (open in editor).
    let got = app.scripts.get_by_id(s.id.clone()).unwrap();
    assert_eq!(got.id, s.id);

    // Update (edit + Save).
    let updated = app
        .scripts
        .update(s.id.clone(), "My Speech v2".into(), "New body".into())
        .unwrap();
    assert_eq!(updated.title, "My Speech v2");
    assert_eq!(updated.content, "New body");

    // Delete.
    app.scripts.delete(s.id.clone()).unwrap();
    assert!(app.scripts.list_all().unwrap().is_empty());
}

#[test]
fn create_rejects_empty_title() {
    let app = app();
    assert!(app.scripts.create("   ".into(), "body".into()).is_err());
}

#[test]
fn duplicate_appends_copy_suffix() {
    let app = app();
    let s = app
        .scripts
        .create("Keynote".into(), "content".into())
        .unwrap();
    let dup = app.scripts.duplicate(s.id.clone()).unwrap();
    assert_eq!(dup.title, "Keynote (Copy)");
    assert_eq!(dup.content, "content");
    assert_ne!(dup.id, s.id);
    assert_eq!(app.scripts.list_all().unwrap().len(), 2);
}

#[test]
fn search_matches_title_and_content_and_empty_lists_all() {
    let app = app();
    app.scripts
        .create("Wedding Toast".into(), "To the happy couple".into())
        .unwrap();
    app.scripts
        .create("Sales Pitch".into(), "Our product is great".into())
        .unwrap();

    // Title match.
    assert_eq!(app.scripts.search("Wedding".into()).unwrap().len(), 1);
    // Content match.
    assert_eq!(app.scripts.search("product".into()).unwrap().len(), 1);
    // No match.
    assert_eq!(app.scripts.search("zzz".into()).unwrap().len(), 0);
    // Empty query -> list all.
    assert_eq!(app.scripts.search("   ".into()).unwrap().len(), 2);
}

// ---------------------------------------------------------------------------
// Import / Export — the Import and Export buttons + native file round-trip.
// ---------------------------------------------------------------------------

#[test]
fn import_from_content_strips_txt_extension_for_title() {
    let app = app();
    let s = app
        .import_export
        .import_from_content("line 1\nline 2\n".into(), "welcome_script.txt".into())
        .unwrap();
    assert_eq!(s.title, "welcome_script");
    assert_eq!(s.content, "line 1\nline 2\n");
    assert_eq!(app.scripts.list_all().unwrap().len(), 1);
}

#[test]
fn export_returns_title_and_content() {
    let app = app();
    let s = app
        .scripts
        .create("Export Me".into(), "body text".into())
        .unwrap();
    let (title, content) = app.import_export.export_content(s.id).unwrap();
    assert_eq!(title, "Export Me");
    assert_eq!(content, "body text");
}

#[test]
fn import_then_export_roundtrip_via_real_file() {
    let app = app();

    // Simulate the OS file dialog: a real .txt on disk.
    let dir = TempDir::new().unwrap();
    let in_path = dir.path().join("conference_talk.txt");
    let body = "Welcome everyone.\n\n[PAUSE]\n\nLet's begin.\n";
    std::fs::write(&in_path, body).unwrap();

    // Read (read_text_file) + import (import_script_from_txt).
    let content = std::fs::read_to_string(&in_path).unwrap();
    let file_name = in_path.file_name().unwrap().to_string_lossy().to_string();
    let imported = app
        .import_export
        .import_from_content(content, file_name)
        .unwrap();
    assert_eq!(imported.title, "conference_talk");

    // Export back out and write to disk (export_script_to_txt_file).
    let (_t, exported) = app.import_export.export_content(imported.id).unwrap();
    let out_path = dir.path().join("exported.txt");
    std::fs::write(&out_path, &exported).unwrap();

    // Round-trip is byte-identical.
    assert_eq!(std::fs::read_to_string(&out_path).unwrap(), body);
}

// ---------------------------------------------------------------------------
// Settings — the Settings panel + Reset button. Survives reload (persisted).
// ---------------------------------------------------------------------------

#[test]
fn settings_default_update_reset_cycle() {
    let app = app();

    // First read seeds + returns defaults.
    let def = app.settings.get().unwrap();
    assert_eq!(def.font_size, 32.0);
    assert!(matches!(def.theme, Theme::Dark));
    assert_eq!(def.countdown_seconds, 3);

    // Update (move sliders / toggle mirror / switch theme).
    let changed = AppSettings {
        font_size: 48.0,
        mirror_mode: true,
        mirror_vertical: true,
        theme: Theme::Light,
        countdown_seconds: 10,
        reading_guide_enabled: true,
        ..Default::default()
    };
    app.settings.update(changed).unwrap();

    let reloaded = app.settings.get().unwrap();
    assert_eq!(reloaded.font_size, 48.0);
    assert!(reloaded.mirror_mode);
    assert!(reloaded.mirror_vertical);
    assert!(matches!(reloaded.theme, Theme::Light));
    assert_eq!(reloaded.countdown_seconds, 10);
    assert!(reloaded.reading_guide_enabled);

    // Reset button restores defaults.
    let reset = app.settings.reset().unwrap();
    assert_eq!(reset.font_size, 32.0);
    assert!(!reset.mirror_mode);
    assert!(matches!(reset.theme, Theme::Dark));
    assert_eq!(app.settings.get().unwrap().font_size, 32.0);
}

#[test]
fn settings_persist_across_new_service_instance() {
    // Same db file, fresh service objects = simulates app restart.
    let dir = TempDir::new().unwrap();
    let db = Arc::new(Database::new(dir.path()).unwrap());
    migrations::run_migrations(&db).unwrap();

    {
        let settings = SettingsService::new(SettingsRepository::new(Arc::clone(&db)));
        let s = AppSettings {
            scroll_speed: 2.5,
            ..Default::default()
        };
        settings.update(s).unwrap();
    }
    {
        let settings = SettingsService::new(SettingsRepository::new(Arc::clone(&db)));
        assert_eq!(settings.get().unwrap().scroll_speed, 2.5);
    }
}

// ---------------------------------------------------------------------------
// Playback state — resume-where-you-left-off (save on exit / load on open).
// ---------------------------------------------------------------------------

#[test]
fn playback_state_save_load_clear() {
    let app = app();
    let s = app.scripts.create("Talk".into(), "content".into()).unwrap();

    app.playback
        .save(
            s.id.clone(),
            1500.0,
            1.5,
            Some(40.0),
            Some(2.0),
            Some(true),
            Some(false),
        )
        .unwrap();

    let loaded = app.playback.load(s.id.clone()).unwrap();
    assert_eq!(loaded.scroll_offset_px, 1500.0);
    assert_eq!(loaded.speed_multiplier, 1.5);
    assert_eq!(loaded.font_size, Some(40.0));
    assert_eq!(loaded.mirror_mode, Some(true));
    assert_eq!(loaded.mirror_vertical, Some(false));

    // Re-save overwrites (resume position advances).
    app.playback
        .save(s.id.clone(), 3000.0, 2.0, None, None, None, None)
        .unwrap();
    assert_eq!(
        app.playback.load(s.id.clone()).unwrap().scroll_offset_px,
        3000.0
    );

    // Clear (restart from top).
    app.playback.clear(s.id.clone()).unwrap();
    assert!(app.playback.load(s.id.clone()).is_err());

    // Clearing again is a no-op (NotFound swallowed).
    app.playback.clear(s.id).unwrap();
}

#[test]
fn deleting_script_cascades_playback_state() {
    let app = app();
    let s = app.scripts.create("Talk".into(), "content".into()).unwrap();
    app.playback
        .save(s.id.clone(), 100.0, 1.0, None, None, None, None)
        .unwrap();
    assert!(app.playback.load(s.id.clone()).is_ok());

    app.scripts.delete(s.id.clone()).unwrap();
    // FK ON DELETE CASCADE removes the orphaned playback row.
    assert!(app.playback.load(s.id).is_err());
}
