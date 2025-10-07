use std::path::{Path, PathBuf};

use tokio::{fs, io::AsyncWriteExt};
use url::Url;

use crate::error::AppError;

pub async fn download(url: &Url, output: &Path) -> Result<PathBuf, AppError> {
    println!("Downloading torrent file from: {}", url);

    let response = reqwest::get(url.clone()).await?;

    if !response.status().is_success() {
        return Err(AppError::DownloadFailed {
            url: url.clone(),
            status: response.status(),
        });
    }

    let filename = url
        .path_segments()
        .and_then(|segments| segments.last())
        .filter(|s| !s.is_empty())
        .unwrap_or("mukund.torrent");

    let mut dest_path = output.to_path_buf();
    dest_path.push(filename);

    let mut file = fs::File::create(&dest_path).await?;
    let content = response.bytes().await?;
    file.write_all(&content).await?;

    println!("Torrent file sucessfully saved to: {}", dest_path.display());

    Ok(dest_path)
}
