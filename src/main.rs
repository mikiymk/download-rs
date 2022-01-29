use reqwest::Url;
use std::collections::HashSet;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    use download::DownloadFile;
    use get_links::get_links;
    use itertools::Itertools;

    let mut urls = Vec::new();
    let mut downloadeds = HashSet::new();

    let initial_url = Url::parse(&get_url())?;
    urls.push(initial_url.clone());

    let sleep_time = std::time::Duration::from_millis(100);

    while let Some(url) = urls.pop() {
        if downloadeds.contains(&url) {
            continue;
        }

        let file = match download_save(&url).await {
            Ok(file) => file,
            Err(err) => {
                eprintln!("{:?}", err);
                continue;
            }
        };

        downloadeds.insert(url.clone());

        let new_urls = match file {
            DownloadFile::Text { text, content_type } => get_links(&content_type, &url, &text),
            _ => Vec::new(),
        };

        let new_urls: Vec<_> = new_urls.into_iter().unique().collect();

        for new_url in new_urls {
            if !downloadeds.contains(&new_url) && initial_url.domain() == new_url.domain() {
                // ダウンロードしてない & 同じドメイン
                urls.push(new_url);
            }
        }

        std::thread::sleep(sleep_time);
    }

    Ok(())
}

fn get_url() -> String {
    std::env::args()
        .nth(1)
        .unwrap_or("https://google.com".to_string())
        .to_owned()
}

async fn download_save(url: &Url) -> Result<download::DownloadFile, Box<dyn std::error::Error>> {
    use download::download_file;
    use save::save_file;

    let file = download_file(url).await?;
    save_file(url, &file)?;
    Ok(file)
}

mod download;
mod get_links;
mod save;
