use super::download::{DownloadBody, DownloadFile};
use reqwest::Url;
use std::path::Path;

/// URLに対応した場所にファイルを保存する
pub fn save_file(download_file: &DownloadFile) -> Result<(), Box<dyn std::error::Error>> {
    use std::io::prelude::*;

    let url = &download_file.location;

    let path = path_from_url(&url);
    let path = Path::new(&path);

    create_dir_if_not_exists(path)?;
    let mut file = std::fs::File::create(path)?;

    match &download_file.body {
        DownloadBody::Text { text } => {
            write!(file, "{}", text)?;
        }

        DownloadBody::Binary { byte } => {
            file.write_all(byte)?;
            file.flush()?;
        }
    }

    Ok(())
}

fn path_from_url(url: &Url) -> String {
    if url.scheme() == "data" {
        return format!("./downloads/data-scheme/{}", uuid::Uuid::new_v4());
    }
    let mut path = format!(
        "./downloads/{}{}",
        url.host()
            .and_then(|x| Some(x.to_string()))
            .unwrap_or("no-host".to_string()),
        url.path()
    );
    if path.ends_with("/") {
        path = format!("{}index.html", path);
    }

    match urlencoding::decode(&path) {
        Ok(path) => path.to_string(),
        Err(_) => path,
    }
}

fn create_dir_if_not_exists(path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(parent) = path.parent() {
        if !parent.is_dir() {
            std::fs::create_dir_all(&parent)?;
        }
    }
    Ok(())
}
