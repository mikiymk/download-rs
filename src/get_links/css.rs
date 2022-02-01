use super::join_url;
use once_cell::sync::Lazy;
use regex::Regex;
use reqwest::Url;

static REGULAR_EXPRESSIONS: Lazy<[Regex; 5]> = Lazy::new(|| {
    [
        Regex::new(r#"url\("([^"]+)"\)"#).unwrap(),
        Regex::new(r#"url\('(([^']|\\')+)'\)"#).unwrap(),
        Regex::new(r#"url\(([^ "'()]+)\)"#).unwrap(),
        Regex::new(r#"@import "([^"]+)""#).unwrap(),
        Regex::new(r#"@import '(([^']|\\')+)'"#).unwrap(),
    ]
});

pub fn get_links_from_css(url: &Url, document: &str) -> Vec<Url> {
    let mut urls = Vec::new();

    for re in &*REGULAR_EXPRESSIONS {
        for cap in re.captures_iter(document) {
            if let Ok(url) = join_url(url, &cap[1]) {
                urls.push(url);
            }
        }
    }

    urls
}
