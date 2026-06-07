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
        let conn = self.db.conn();
        let mut stmt = conn.prepare("SELECT value FROM settings WHERE key = 'app_settings'")?;
        let result: Result<String, rusqlite::Error> = stmt.query_row([], |row| row.get(0));
        match result {
            Ok(json_str) => {
                let settings: AppSettings = serde_json::from_str(&json_str)
                    .map_err(|e| AppError::General(e.to_string()))?;
                Ok(settings)
            }
            Err(rusqlite::Error::QueryReturnedNoRows) => {
                let settings = AppSettings::default();
                self.save(&settings)?;
                Ok(settings)
            }
            Err(e) => Err(AppError::Database(e.to_string())),
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
