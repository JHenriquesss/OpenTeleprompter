use crate::domain::errors::AppError;
use std::path::{Path, PathBuf};

#[allow(dead_code)]
pub fn read_text_file(path: &Path) -> Result<String, AppError> {
    Ok(std::fs::read_to_string(path)?)
}

#[allow(dead_code)]
pub fn write_text_file(path: &Path, content: &str) -> Result<(), AppError> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    Ok(std::fs::write(path, content)?)
}

pub fn get_app_data_dir(app_name: &str) -> PathBuf {
    let base = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    base.join(app_name)
}
