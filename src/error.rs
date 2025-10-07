use std::path::PathBuf;
use thiserror::Error;
use url::Url;

/// Custom error types for the application.
#[derive(Error, Debug)]
pub enum AppError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("HTTP request error: {0}")]
    Http(#[from] reqwest::Error),
    #[error("URL parsing error: {0}")]
    Url(#[from] url::ParseError),
    #[error("File not found: {0}")]
    FileNotFound(PathBuf),
    #[error("Failed to download torrent file from {url}: {status}")]
    DownloadFailed {
        url: Url,
        status: reqwest::StatusCode,
    },
}
