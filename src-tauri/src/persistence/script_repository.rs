use super::database::Database;
use crate::domain::errors::AppError;
use crate::domain::script::Script;

pub struct ScriptRepository {
    db: std::sync::Arc<Database>,
}

impl ScriptRepository {
    pub fn new(db: std::sync::Arc<Database>) -> Self {
        Self { db }
    }

    pub fn create(&self, script: &Script) -> Result<Script, AppError> {
        let conn = self.db.conn();
        conn.execute(
            "INSERT INTO scripts (id, title, content, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?5)",
            rusqlite::params![
                script.id,
                script.title,
                script.content,
                script.created_at,
                script.updated_at,
            ],
        )?;
        Ok(script.clone())
    }

    pub fn update(&self, script: &Script) -> Result<Script, AppError> {
        let conn = self.db.conn();
        let rows = conn.execute(
            "UPDATE scripts SET title = ?1, content = ?2, updated_at = ?3 WHERE id = ?4",
            rusqlite::params![script.title, script.content, script.updated_at, script.id,],
        )?;
        if rows == 0 {
            return Err(AppError::NotFound(script.id.clone()));
        }
        Ok(script.clone())
    }

    pub fn delete(&self, id: &str) -> Result<(), AppError> {
        let conn = self.db.conn();
        let rows = conn.execute("DELETE FROM scripts WHERE id = ?1", rusqlite::params![id])?;
        if rows == 0 {
            return Err(AppError::NotFound(id.to_string()));
        }
        Ok(())
    }

    pub fn get_by_id(&self, id: &str) -> Result<Script, AppError> {
        let conn = self.db.conn();
        let mut stmt = conn.prepare(
            "SELECT id, title, content, created_at, updated_at FROM scripts WHERE id = ?1",
        )?;
        let result = stmt.query_row(rusqlite::params![id], |row| {
            Ok(Script {
                id: row.get(0)?,
                title: row.get(1)?,
                content: row.get(2)?,
                created_at: row.get(3)?,
                updated_at: row.get(4)?,
            })
        });
        result.map_err(|e| match e {
            rusqlite::Error::QueryReturnedNoRows => AppError::NotFound(id.to_string()),
            other => AppError::Database(other.to_string()),
        })
    }

    pub fn list_all(&self) -> Result<Vec<Script>, AppError> {
        let conn = self.db.conn();
        let mut stmt = conn.prepare(
            "SELECT id, title, content, created_at, updated_at FROM scripts ORDER BY updated_at DESC",
        )?;
        let rows = stmt.query_map([], |row| {
            Ok(Script {
                id: row.get(0)?,
                title: row.get(1)?,
                content: row.get(2)?,
                created_at: row.get(3)?,
                updated_at: row.get(4)?,
            })
        })?;
        let mut scripts = Vec::new();
        for row in rows {
            scripts.push(row?);
        }
        Ok(scripts)
    }

    pub fn search(&self, query: &str) -> Result<Vec<Script>, AppError> {
        let conn = self.db.conn();
        let pattern = format!("%{}%", query);
        let mut stmt = conn.prepare(
            "SELECT id, title, content, created_at, updated_at FROM scripts WHERE title LIKE ?1 OR content LIKE ?1 ORDER BY updated_at DESC",
        )?;
        let rows = stmt.query_map(rusqlite::params![pattern], |row| {
            Ok(Script {
                id: row.get(0)?,
                title: row.get(1)?,
                content: row.get(2)?,
                created_at: row.get(3)?,
                updated_at: row.get(4)?,
            })
        })?;
        let mut scripts = Vec::new();
        for row in rows {
            scripts.push(row?);
        }
        Ok(scripts)
    }
}
