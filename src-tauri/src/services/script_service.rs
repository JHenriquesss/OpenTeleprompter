use crate::domain::errors::AppError;
use crate::domain::script::Script;
use crate::persistence::script_repository::ScriptRepository;
use chrono::Utc;
use uuid::Uuid;

pub struct ScriptService {
    repo: ScriptRepository,
}

impl ScriptService {
    pub fn new(repo: ScriptRepository) -> Self {
        Self { repo }
    }

    pub fn create(&self, title: String, content: String) -> Result<Script, AppError> {
        if title.trim().is_empty() {
            return Err(AppError::InvalidInput("Title cannot be empty".into()));
        }
        let now = Utc::now().to_rfc3339();
        let script = Script {
            id: Uuid::new_v4().to_string(),
            title: title.trim().to_string(),
            content,
            created_at: now.clone(),
            updated_at: now,
        };
        self.repo.create(&script)
    }

    pub fn update(&self, id: String, title: String, content: String) -> Result<Script, AppError> {
        if title.trim().is_empty() {
            return Err(AppError::InvalidInput("Title cannot be empty".into()));
        }
        let mut script = self.repo.get_by_id(&id)?;
        script.title = title.trim().to_string();
        script.content = content;
        script.updated_at = Utc::now().to_rfc3339();
        self.repo.update(&script)
    }

    pub fn delete(&self, id: String) -> Result<(), AppError> {
        self.repo.delete(&id)
    }

    pub fn get_by_id(&self, id: String) -> Result<Script, AppError> {
        self.repo.get_by_id(&id)
    }

    pub fn list_all(&self) -> Result<Vec<Script>, AppError> {
        self.repo.list_all()
    }

    pub fn search(&self, query: String) -> Result<Vec<Script>, AppError> {
        if query.trim().is_empty() {
            return self.repo.list_all();
        }
        self.repo.search(query.trim())
    }

    pub fn duplicate(&self, id: String) -> Result<Script, AppError> {
        let original = self.repo.get_by_id(&id)?;
        let now = Utc::now().to_rfc3339();
        let script = Script {
            id: Uuid::new_v4().to_string(),
            title: format!("{} (Copy)", original.title),
            content: original.content,
            created_at: now.clone(),
            updated_at: now,
        };
        self.repo.create(&script)
    }
}
