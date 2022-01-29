use reqwest::Url;
use std::collections::HashSet;

// TODO
// use javascript
// redirect
// data scheme

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    use args::Arguments;
    use download::DownloadFile;
    use get_links::get_links;
    use itertools::Itertools;

    let mut urls = Vec::new();
    let mut downloadeds = HashSet::new();

    let args = match Arguments::get() {
        Ok(args) => args,
        Err(err) => {
            eprintln!("Can't get args: {:?}", err);
            return Err(err);
        }
    };

    urls.extend(args.starts.clone());

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
            if !downloadeds.contains(&new_url) && args.is_allow_url(&new_url) {
                // ダウンロードしてない & 同じドメイン
                urls.push(new_url);
            }
        }

        std::thread::sleep(sleep_time);
    }

    Ok(())
}

async fn download_save(url: &Url) -> Result<download::DownloadFile, Box<dyn std::error::Error>> {
    use download::download_file;
    use save::save_file;

    let file = download_file(url).await?;
    save_file(url, &file)?;
    Ok(file)
}

mod args;
mod download;
mod get_links;
mod save;
