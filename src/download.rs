use bytes::Bytes;
use reqwest::Url;

pub enum DownloadFile {
    Text { text: String, content_type: String },
    Binary { byte: Bytes },
}

pub async fn download_file(url: &Url) -> Result<DownloadFile, Box<dyn std::error::Error>> {
    let resp = reqwest::get(url.clone()).await?;
    let headers = resp.headers();
    let content_type = headers
        .get("content-type")
        .and_then(|x| x.to_str().ok())
        .unwrap_or("application/octet-stream")
        .to_owned();
    let content_length = headers
        .get("content-length")
        .and_then(|x| x.to_str().ok())
        .and_then(|x| x.parse().ok())
        .unwrap_or(0);

    println!("{} {} {}", format_byte(content_length), content_type, url);

    if content_type.starts_with("text") {
        let text = resp.text().await?;
        Ok(DownloadFile::Text { content_type, text })
    } else {
        let byte = resp.bytes().await?;
        Ok(DownloadFile::Binary { byte })
    }
}

fn format_byte(size: usize) -> String {
    const KILO: usize = 1024;
    const MEGA: usize = 1024 * 1024;
    const GIGA: usize = 1024 * 1024 * 1024;
    const TERA: usize = 1024 * 1024 * 1024 * 1024;

    if size < KILO {
        format!("{: >4} ", size)
    } else if size < MEGA {
        format!("{: >4}k", size / KILO)
    } else if size < GIGA {
        format!("{: >4}M", size / MEGA)
    } else if size < TERA {
        format!("{: >4}G", size / GIGA)
    } else {
        format!("{: >4}T", size / TERA)
    }
}
