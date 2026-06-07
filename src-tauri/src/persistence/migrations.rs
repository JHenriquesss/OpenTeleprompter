use super::database::Database;

pub fn run_migrations(db: &Database) -> Result<(), rusqlite::Error> {
    let conn = db.conn();
    conn.execute_batch(
        "
        CREATE TABLE IF NOT EXISTS scripts (
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
        );
        ",
    )?;
    Ok(())
}
