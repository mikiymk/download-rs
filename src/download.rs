use bytes::Bytes;
use reqwest::Url;

#[derive(Debug, Clone)]
pub enum DownloadBody {
    Text { text: String },
    Binary { byte: Bytes },
}

#[derive(Debug, Clone)]
pub struct DownloadFile {
    pub body: DownloadBody,
    pub content_type: String,
    pub location: Url,
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
    let location = resp.url().clone();

    println!("{} {} {}", format_byte(content_length), content_type, url);

    let body = if content_type.starts_with("text") {
        let text = resp.text().await?;
        DownloadBody::Text { text }
    } else {
        let byte = resp.bytes().await?;
        DownloadBody::Binary { byte }
    };

    Ok(DownloadFile {
        body,
        content_type,
        location,
    })
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
