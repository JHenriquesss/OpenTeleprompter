use crate::domain::errors::AppError;
use crate::domain::script::Script;
use crate::services::script_service::ScriptService;

pub struct ImportExportService {
    script_service: ScriptService,
}

impl ImportExportService {
    pub fn new(script_service: ScriptService) -> Self {
        Self { script_service }
    }

    pub fn import_from_content(
        &self,
        content: String,
        file_name: String,
    ) -> Result<Script, AppError> {
        let title = file_name
            .strip_suffix(".txt")
            .unwrap_or(&file_name)
            .to_string();
        self.script_service.create(title, content)
    }

    pub fn export_content(&self, script_id: String) -> Result<(String, String), AppError> {
        let script = self.script_service.get_by_id(script_id)?;
        Ok((script.title, script.content))
    }
}
