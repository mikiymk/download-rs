#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = get_url();
    let resp = reqwest::get(&url).await?;
    let text = resp.text().await?;
    let document = scraper::Html::parse_document(&text);
    println!("{:#?}", document);
    Ok(())
}

fn get_url() -> String {
    let mut args = std::env::args();
    args.next();
    args.next()
        .get_or_insert("https://google.com".to_string())
        .to_owned()
}
