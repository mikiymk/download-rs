use regex::Regex;
use reqwest::Url;
use std::collections::HashSet;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut urls = Vec::new();
    let mut downloadeds = HashSet::new();
    urls.push(Url::parse(&get_url())?);

    let sleep_time = std::time::Duration::from_millis(1000);

    while let Some(url) = urls.pop() {
        let file = match download_file(&url).await {
            Ok(file) => file,
            Err(err) => {
                eprintln!("{:?}", err);
                continue;
            }
        };
        match save_file(&url, &file) {
            Ok(()) => {}
            Err(err) => {
                eprintln!("{:?}", err);
                continue;
            }
        }
        downloadeds.insert(url.clone());

        match file {
            DownloadFile::Text {
                text, content_type, ..
            } if content_type.starts_with("text/html") => {
                urls.extend(get_links_from_html(&url, &text, &downloadeds));
            }

            DownloadFile::Text {
                text, content_type, ..
            } if content_type.starts_with("text/css") => {
                urls.extend(get_links_from_css(&url, &text, &downloadeds))
            }

            _ => {}
        }
        std::thread::sleep(sleep_time);
    }

    Ok(())
}

lazy_static::lazy_static! {
    static ref SELECTOR_TEXTS: [(&'static str, Vec<&'static str>); 3] = [
        ("href", vec!["a", "link", "area"]),
        (
            "src",
            vec!["img", "script", "iframe", "portal", "embed", "audio", "video", "track",],
        ),
        ("srcset", vec!["source"]),
    ];
    static ref REGULAR_EXPRESSIONS: [Regex;2] = [
        Regex::new(r#"url\(("[^"]+"|'([^']|\\')+'|[^ "'()]+)\)"#).unwrap(),
        Regex::new(r#"@import ("[^"]+"|'([^']|\\')+')"#).unwrap(),
    ];
}

fn get_links_from_html(url: &Url, document: &str, downloaded_links: &HashSet<Url>) -> Vec<Url> {
    use scraper::Selector;

    let document = scraper::Html::parse_document(&document);
    let base = document
        .select(&Selector::parse("base").unwrap())
        .nth(0)
        .and_then(|x| x.value().attr("href"))
        .and_then(|x| url.join(x).ok())
        .unwrap_or(url.clone())
        .to_owned();

    let mut urls = Vec::new();

    for (attribute, selectors) in &*SELECTOR_TEXTS {
        for selector in selectors {
            let selector = Selector::parse(&format!("{}[{}]", selector, attribute)).unwrap();

            urls.extend(
                document
                    .select(&selector)
                    .flat_map(|x| x.value().attr(attribute))
                    .map(|x| x.to_string())
                    .map(|x| base.join(&x))
                    .filter(|x| match x {
                        Ok(x) => !downloaded_links.contains(&x),
                        Err(_) => false,
                    })
                    .flatten(),
            );
        }
    }

    urls
}

fn get_links_from_css(url: &Url, document: &str, downloaded_links: &HashSet<Url>) -> Vec<Url> {
    let mut urls = Vec::new();

    for re in &*REGULAR_EXPRESSIONS {
        for cap in re.captures_iter(document) {
            let cap = &cap[1];
            let url = url.join(cap);

            match url {
                Ok(url) if !downloaded_links.contains(&url) => {
                    urls.push(url);
                }
                _ => break,
            }
        }
    }

    urls
}

fn get_url() -> String {
    std::env::args()
        .nth(1)
        .unwrap_or("https://google.com".to_string())
        .to_owned()
}

use bytes::Bytes;
enum DownloadFile {
    Text {
        text: String,
        content_type: String,
    },
    Binary {
        byte: Bytes, /*content_type: String*/
    },
}

async fn download_file(url: &Url) -> Result<DownloadFile, Box<dyn std::error::Error>> {
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

    println!("{} {} {}", content_length, content_type, url);

    if content_type.starts_with("text") {
        let text = resp.text().await?;

        Ok(DownloadFile::Text { content_type, text })
    } else {
        let byte = resp.bytes().await?;

        Ok(DownloadFile::Binary {
            /*content_type,*/ byte,
        })
    }
}

fn save_file(url: &Url, download_file: &DownloadFile) -> Result<(), Box<dyn std::error::Error>> {
    use std::io::prelude::*;
    let mut path = format!(
        "./downloads/{}{}",
        url.host()
            .and_then(|x| Some(x.to_string()))
            .unwrap_or("".to_string()),
        url.path()
    );
    if path.ends_with("/") {
        path = format!("{}index", path);
    }
    let path = std::path::Path::new(&path);
    if let Some(parent) = path.parent() {
        if !parent.is_dir() {
            std::fs::create_dir_all(&parent)?;
        }
    }
    let mut file = std::fs::File::create(&path)?;

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
