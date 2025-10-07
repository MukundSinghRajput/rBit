use std::path::{Path, PathBuf};

use crate::utils::{download, validate};
use tokio::fs;
use url::Url;

use crate::{Args, error::AppError};

pub async fn run(args: Args) -> Result<PathBuf, AppError> {
    fs::create_dir_all(&args.output).await?;

    if let Ok(url) = Url::parse(&args.source) {
        download(&url, Path::new(&args.output)).await
    } else {
        validate(&args.source)
    }
}
