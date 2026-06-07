use crate::domain::errors::AppError;
use crate::domain::settings::AppSettings;
use crate::persistence::settings_repository::SettingsRepository;

pub struct SettingsService {
    repo: SettingsRepository,
}

impl SettingsService {
    pub fn new(repo: SettingsRepository) -> Self {
        Self { repo }
    }

    pub fn get(&self) -> Result<AppSettings, AppError> {
        self.repo.get()
    }

    pub fn update(&self, settings: AppSettings) -> Result<(), AppError> {
        self.repo.save(&settings)
    }

    pub fn reset(&self) -> Result<AppSettings, AppError> {
        let defaults = AppSettings::default();
        self.repo.save(&defaults)?;
        Ok(defaults)
    }
}
