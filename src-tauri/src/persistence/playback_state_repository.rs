use super::database::Database;
use crate::domain::errors::AppError;
use crate::domain::playback_state::ScriptPlaybackState;

pub struct PlaybackStateRepository {
    db: std::sync::Arc<Database>,
}

impl PlaybackStateRepository {
    pub fn new(db: std::sync::Arc<Database>) -> Self {
        Self { db }
    }

    pub fn save(&self, state: &ScriptPlaybackState) -> Result<ScriptPlaybackState, AppError> {
        let conn = self.db.conn();
        conn.execute(
            "INSERT OR REPLACE INTO script_playback_state
             (script_id, scroll_offset_px, speed_multiplier, font_size, line_height, mirror_mode, mirror_vertical, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            rusqlite::params![
                state.script_id,
                state.scroll_offset_px,
                state.speed_multiplier,
                state.font_size,
                state.line_height,
                state.mirror_mode.map(|v| v as i32),
                state.mirror_vertical.map(|v| v as i32),
                state.updated_at,
            ],
        )?;
        Ok(state.clone())
    }

    pub fn load_by_script_id(&self, script_id: &str) -> Result<ScriptPlaybackState, AppError> {
        let conn = self.db.conn();
        let mut stmt = conn.prepare(
            "SELECT script_id, scroll_offset_px, speed_multiplier, font_size, line_height, mirror_mode, mirror_vertical, updated_at
             FROM script_playback_state WHERE script_id = ?1",
        )?;
        let result = stmt.query_row(rusqlite::params![script_id], |row| {
            Ok(ScriptPlaybackState {
                script_id: row.get(0)?,
                scroll_offset_px: row.get(1)?,
                speed_multiplier: row.get(2)?,
                font_size: row.get(3)?,
                line_height: row.get(4)?,
                mirror_mode: row.get::<_, Option<i32>>(5)?.map(|v| v != 0),
                mirror_vertical: row.get::<_, Option<i32>>(6)?.map(|v| v != 0),
                updated_at: row.get(7)?,
            })
        });
        result.map_err(|e| match e {
            rusqlite::Error::QueryReturnedNoRows => AppError::NotFound(script_id.to_string()),
            other => AppError::Database(other.to_string()),
        })
    }

    pub fn delete(&self, script_id: &str) -> Result<(), AppError> {
        let conn = self.db.conn();
        let rows = conn.execute(
            "DELETE FROM script_playback_state WHERE script_id = ?1",
            rusqlite::params![script_id],
        )?;
        if rows == 0 {
            return Err(AppError::NotFound(script_id.to_string()));
        }
        Ok(())
    }
}
