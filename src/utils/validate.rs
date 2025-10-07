use crate::error::AppError;
use std::path::PathBuf;

pub fn validate(path_str: &str) -> Result<PathBuf, AppError> {
    let path = PathBuf::from(path_str);

    if !path.exists() {
        return Err(AppError::FileNotFound(path));
    }
    Ok(path)
}
