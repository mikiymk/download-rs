use args::Arguments;
use reqwest::Url;
use std::collections::HashSet;

pub async fn run(args: Arguments) -> Result<(), Box<dyn std::error::Error>> {
    use download::DownloadBody;
    use get_links::get_links;
    use itertools::Itertools;

    let sleep_time = std::time::Duration::from_millis(100);
    let mut urls = Vec::new();
    let mut downloadeds = HashSet::new();

    urls.extend(args.starts().clone());

    while let Some(url) = urls.pop() {
        if downloadeds.contains(&url) {
            continue;
        }

        let file = match download_save(&url).await {
            Ok(file) => file,
            Err(err) => {
                eprintln!("{:?} : {}", err, url);
                continue;
            }
        };

        downloadeds.insert(url.clone());

        let new_urls = match file.body {
            DownloadBody::Text { text } => get_links(&file.content_type, &file.location, &text),
            _ => Vec::new(),
        };

        let new_urls: Vec<_> = new_urls.into_iter().unique().collect();

        for new_url in new_urls {
            if !downloadeds.contains(&new_url) && args.is_allow_url(&new_url) {
                // ダウンロードしてない & 同じドメインなら予定リストに追加
                urls.push(new_url);
            }
        }

        // リクエストの間隔を開けるため待機
        std::thread::sleep(sleep_time);
    }
    Ok(())
}

async fn download_save(url: &Url) -> Result<download::DownloadFile, Box<dyn std::error::Error>> {
    use download::download_file;
    use save::save_file;

    let file = download_file(url).await?;
    save_file(&file)?;
    Ok(file)
}

pub mod args;
mod download;
mod get_links;
mod save;
