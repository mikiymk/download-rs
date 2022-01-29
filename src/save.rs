use super::download::DownloadFile;
use reqwest::Url;
use std::path::Path;

/// URLに対応した場所にファイルを保存する
pub fn save_file(
    url: &Url,
    download_file: &DownloadFile,
) -> Result<(), Box<dyn std::error::Error>> {
    use std::io::prelude::*;

    let path = path_from_url(&url);
    let path = Path::new(&path);

    create_dir_if_not_exists(path)?;
    let mut file = std::fs::File::create(path)?;

    match download_file {
        DownloadFile::Text { text, .. } => {
            write!(file, "{}", text)?;
        }
        DownloadFile::Binary { byte, .. } => {
            file.write_all(byte)?;
            file.flush()?;
        }
    }

    Ok(())
}

fn path_from_url(url: &Url) -> String {
    let mut path = format!(
        "./downloads/{}{}",
        url.host()
            .and_then(|x| Some(x.to_string()))
            .unwrap_or("".to_string()),
        url.path()
    );
    if path.ends_with("/") {
        path = format!("{}index.html", path);
    }
    path
}

fn create_dir_if_not_exists(path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(parent) = path.parent() {
        if !parent.is_dir() {
            std::fs::create_dir_all(&parent)?;
        }
    }
    Ok(())
}
