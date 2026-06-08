use super::database::Database;
use crate::domain::errors::AppError;
use crate::domain::settings::AppSettings;

pub struct SettingsRepository {
    db: std::sync::Arc<Database>,
}

impl SettingsRepository {
    pub fn new(db: std::sync::Arc<Database>) -> Self {
        Self { db }
    }

    pub fn get(&self) -> Result<AppSettings, AppError> {
        // Read the stored JSON in a scope that drops the connection lock before
        // we may call `save()`. `std::sync::Mutex` is NOT re-entrant, so holding
        // the guard across `self.save()` (which locks again on the same thread)
        // deadlocks — which froze first-run settings loading on an empty DB.
        let stored: Option<String> = {
            let conn = self.db.conn();
            let mut stmt =
                conn.prepare("SELECT value FROM settings WHERE key = 'app_settings'")?;
            match stmt.query_row([], |row| row.get::<_, String>(0)) {
                Ok(json_str) => Some(json_str),
                Err(rusqlite::Error::QueryReturnedNoRows) => None,
                Err(e) => return Err(AppError::Database(e.to_string())),
            }
        };

        match stored {
            Some(json_str) => {
                serde_json::from_str(&json_str).map_err(|e| AppError::General(e.to_string()))
            }
            None => {
                // First run: seed and return defaults (lock now released).
                let settings = AppSettings::default();
                self.save(&settings)?;
                Ok(settings)
            }
        }
    }

    pub fn save(&self, settings: &AppSettings) -> Result<(), AppError> {
        let conn = self.db.conn();
        let json_str =
            serde_json::to_string(settings).map_err(|e| AppError::General(e.to_string()))?;
        conn.execute(
            "INSERT OR REPLACE INTO settings (key, value) VALUES ('app_settings', ?1)",
            rusqlite::params![json_str],
        )?;
        Ok(())
    }
}
