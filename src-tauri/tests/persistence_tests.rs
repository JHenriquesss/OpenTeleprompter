use rusqlite::Connection;

fn setup_db() -> Connection {
    let conn = Connection::open_in_memory().unwrap();
    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS scripts (
            id TEXT PRIMARY KEY,
            title TEXT NOT NULL,
            content TEXT NOT NULL DEFAULT '',
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL
        );
        CREATE TABLE IF NOT EXISTS settings (
            key TEXT PRIMARY KEY,
            value TEXT NOT NULL
        );
        CREATE TABLE IF NOT EXISTS script_playback_state (
            script_id TEXT PRIMARY KEY,
            scroll_offset_px REAL NOT NULL DEFAULT 0.0,
            speed_multiplier REAL NOT NULL DEFAULT 1.0,
            font_size REAL,
            line_height REAL,
            mirror_mode INTEGER,
            mirror_vertical INTEGER,
            updated_at TEXT NOT NULL,
            FOREIGN KEY (script_id) REFERENCES scripts(id) ON DELETE CASCADE
        );",
    )
    .unwrap();
    conn
}

#[test]
fn can_create_and_retrieve_script() {
    let conn = setup_db();
    conn.execute(
        "INSERT INTO scripts (id, title, content, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?5)",
        rusqlite::params!["id-1", "Test", "Content", "now", "now"],
    ).unwrap();

    let mut stmt = conn
        .prepare("SELECT id, title, content, created_at, updated_at FROM scripts WHERE id = ?1")
        .unwrap();
    let result: (String, String, String, String, String) = stmt
        .query_row(rusqlite::params!["id-1"], |row| {
            Ok((
                row.get(0)?,
                row.get(1)?,
                row.get(2)?,
                row.get(3)?,
                row.get(4)?,
            ))
        })
        .unwrap();

    assert_eq!(result.1, "Test");
    assert_eq!(result.2, "Content");
}

#[test]
fn can_update_script() {
    let conn = setup_db();
    conn.execute(
        "INSERT INTO scripts (id, title, content, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?5)",
        rusqlite::params!["id-1", "Original", "Original content", "now", "now"],
    ).unwrap();

    conn.execute(
        "UPDATE scripts SET title = ?1, content = ?2, updated_at = ?3 WHERE id = ?4",
        rusqlite::params!["Updated", "Updated content", "later", "id-1"],
    )
    .unwrap();

    let mut stmt = conn
        .prepare("SELECT title, content FROM scripts WHERE id = ?1")
        .unwrap();
    let (title, content): (String, String) = stmt
        .query_row(rusqlite::params!["id-1"], |row| {
            Ok((row.get(0)?, row.get(1)?))
        })
        .unwrap();

    assert_eq!(title, "Updated");
    assert_eq!(content, "Updated content");
}

#[test]
fn can_delete_script() {
    let conn = setup_db();
    conn.execute(
        "INSERT INTO scripts (id, title, content, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?5)",
        rusqlite::params!["id-1", "Test", "Content", "now", "now"],
    ).unwrap();

    conn.execute(
        "DELETE FROM scripts WHERE id = ?1",
        rusqlite::params!["id-1"],
    )
    .unwrap();

    let count: i64 = conn
        .query_row(
            "SELECT COUNT(*) FROM scripts WHERE id = ?1",
            rusqlite::params!["id-1"],
            |row| row.get(0),
        )
        .unwrap();
    assert_eq!(count, 0);
}

#[test]
fn can_list_all_scripts() {
    let conn = setup_db();
    conn.execute(
        "INSERT INTO scripts (id, title, content, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?5)",
        rusqlite::params!["id-1", "A", "", "now", "now"],
    ).unwrap();
    conn.execute(
        "INSERT INTO scripts (id, title, content, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?5)",
        rusqlite::params!["id-2", "B", "", "now", "now"],
    ).unwrap();

    let mut stmt = conn.prepare("SELECT COUNT(*) FROM scripts").unwrap();
    let count: i64 = stmt.query_row([], |row| row.get(0)).unwrap();
    assert_eq!(count, 2);
}

#[test]
fn can_search_scripts_by_title() {
    let conn = setup_db();
    conn.execute(
        "INSERT INTO scripts (id, title, content, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?5)",
        rusqlite::params!["id-1", "Hello World", "Content", "now", "now"],
    ).unwrap();
    conn.execute(
        "INSERT INTO scripts (id, title, content, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?5)",
        rusqlite::params!["id-2", "Goodbye", "Other", "now", "now"],
    ).unwrap();

    let mut stmt = conn
        .prepare("SELECT COUNT(*) FROM scripts WHERE title LIKE ?1")
        .unwrap();
    let count: i64 = stmt
        .query_row(rusqlite::params!["%Hello%"], |row| row.get(0))
        .unwrap();
    assert_eq!(count, 1);
}

#[test]
fn settings_can_be_saved_and_loaded() {
    let conn = setup_db();
    let json = r#"{"font_size":32.0,"line_height":1.8,"text_width":60.0,"scroll_speed":1.0,"mirror_mode":false,"theme":"Dark"}"#;
    conn.execute(
        "INSERT OR REPLACE INTO settings (key, value) VALUES ('app_settings', ?1)",
        rusqlite::params![json],
    )
    .unwrap();

    let result: String = conn
        .query_row(
            "SELECT value FROM settings WHERE key = 'app_settings'",
            [],
            |row| row.get(0),
        )
        .unwrap();
    assert_eq!(result, json);
}

#[test]
fn can_save_playback_state() {
    let conn = setup_db();
    conn.execute(
        "INSERT INTO scripts (id, title, content, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?5)",
        rusqlite::params!["script-1", "Test", "Content", "now", "now"],
    )
    .unwrap();

    conn.execute(
        "INSERT INTO script_playback_state (script_id, scroll_offset_px, speed_multiplier, font_size, line_height, mirror_mode, mirror_vertical, updated_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
        rusqlite::params!["script-1", 1500.0, 1.5, rusqlite::types::Value::Null, rusqlite::types::Value::Null, 1_i32, 0_i32, "now"],
    )
    .unwrap();

    let mut stmt = conn
        .prepare("SELECT scroll_offset_px, speed_multiplier, mirror_mode, mirror_vertical FROM script_playback_state WHERE script_id = ?1")
        .unwrap();
    let (scroll, speed, mirror_h, mirror_v): (f64, f64, i32, i32) = stmt
        .query_row(rusqlite::params!["script-1"], |row| {
            Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?))
        })
        .unwrap();

    assert_eq!(scroll, 1500.0);
    assert_eq!(speed, 1.5);
    assert_eq!(mirror_h, 1);
    assert_eq!(mirror_v, 0);
}

#[test]
fn can_load_playback_state_by_script_id() {
    let conn = setup_db();
    conn.execute(
        "INSERT INTO scripts (id, title, content, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?5)",
        rusqlite::params!["script-1", "Test", "Content", "now", "now"],
    )
    .unwrap();

    conn.execute(
        "INSERT INTO script_playback_state (script_id, scroll_offset_px, speed_multiplier, font_size, line_height, mirror_mode, mirror_vertical, updated_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
        rusqlite::params!["script-1", 500.0, 2.0, rusqlite::types::Value::Null, rusqlite::types::Value::Null, 0_i32, 0_i32, "now"],
    )
    .unwrap();

    let mut stmt = conn
        .prepare("SELECT COUNT(*) FROM script_playback_state WHERE script_id = ?1")
        .unwrap();
    let count: i64 = stmt
        .query_row(rusqlite::params!["script-1"], |row| row.get(0))
        .unwrap();

    assert_eq!(count, 1);
}

#[test]
fn can_update_playback_state() {
    let conn = setup_db();
    conn.execute(
        "INSERT INTO scripts (id, title, content, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?5)",
        rusqlite::params!["script-1", "Test", "Content", "now", "now"],
    )
    .unwrap();

    conn.execute(
        "INSERT INTO script_playback_state (script_id, scroll_offset_px, speed_multiplier, font_size, line_height, mirror_mode, mirror_vertical, updated_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
        rusqlite::params!["script-1", 500.0, 1.0, rusqlite::types::Value::Null, rusqlite::types::Value::Null, 0_i32, 0_i32, "now"],
    )
    .unwrap();

    conn.execute(
        "UPDATE script_playback_state SET scroll_offset_px = ?1, speed_multiplier = ?2, updated_at = ?3 WHERE script_id = ?4",
        rusqlite::params![2000.0, 2.5, "later", "script-1"],
    )
    .unwrap();

    let mut stmt = conn
        .prepare("SELECT scroll_offset_px, speed_multiplier FROM script_playback_state WHERE script_id = ?1")
        .unwrap();
    let (scroll, speed): (f64, f64) = stmt
        .query_row(rusqlite::params!["script-1"], |row| {
            Ok((row.get(0)?, row.get(1)?))
        })
        .unwrap();

    assert_eq!(scroll, 2000.0);
    assert_eq!(speed, 2.5);
}

#[test]
fn can_delete_playback_state() {
    let conn = setup_db();
    conn.execute(
        "INSERT INTO scripts (id, title, content, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?5)",
        rusqlite::params!["script-1", "Test", "Content", "now", "now"],
    )
    .unwrap();

    conn.execute(
        "INSERT INTO script_playback_state (script_id, scroll_offset_px, speed_multiplier, font_size, line_height, mirror_mode, mirror_vertical, updated_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
        rusqlite::params!["script-1", 500.0, 1.0, rusqlite::types::Value::Null, rusqlite::types::Value::Null, 0_i32, 0_i32, "now"],
    )
    .unwrap();

    conn.execute(
        "DELETE FROM script_playback_state WHERE script_id = ?1",
        rusqlite::params!["script-1"],
    )
    .unwrap();

    let count: i64 = conn
        .query_row(
            "SELECT COUNT(*) FROM script_playback_state WHERE script_id = ?1",
            rusqlite::params!["script-1"],
            |row| row.get(0),
        )
        .unwrap();
    assert_eq!(count, 0);
}
