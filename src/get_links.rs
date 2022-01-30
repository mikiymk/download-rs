use once_cell::sync::Lazy;
use regex::Regex;
use reqwest::Url;
use scraper::Selector;

static SELECTORS: Lazy<[(Selector, &'static str); 14]> = Lazy::new(|| {
    [
        (Selector::parse("a[href]").unwrap(), "href"),
        (Selector::parse("link[href]").unwrap(), "href"),
        (Selector::parse("area[href]").unwrap(), "href"),
        (Selector::parse("img[src]").unwrap(), "src"),
        (Selector::parse("script[src]").unwrap(), "src"),
        (Selector::parse("iframe[src]").unwrap(), "src"),
        (Selector::parse("portal[src]").unwrap(), "src"),
        (Selector::parse("embed[src]").unwrap(), "src"),
        (Selector::parse("audio[src]").unwrap(), "src"),
        (Selector::parse("video[src]").unwrap(), "src"),
        (Selector::parse("track[src]").unwrap(), "src"),
        (
            Selector::parse("meta[property=\"og:image\"]").unwrap(),
            "content",
        ),
        (
            Selector::parse("meta[property=\"og:url\"]").unwrap(),
            "content",
        ),
        (
            Selector::parse("meta[name=\"twitter:image\"]").unwrap(),
            "content",
        ),
    ]
});

static SRCSET_SELECTOR: Lazy<Selector> = Lazy::new(|| Selector::parse("source[srcset]").unwrap());

static REGULAR_EXPRESSIONS: Lazy<[Regex; 5]> = Lazy::new(|| {
    [
        Regex::new(r#"url\("([^"]+)"\)"#).unwrap(),
        Regex::new(r#"url\('(([^']|\\')+)'\)"#).unwrap(),
        Regex::new(r#"url\(([^ "'()]+)\)"#).unwrap(),
        Regex::new(r#"@import "([^"]+)""#).unwrap(),
        Regex::new(r#"@import '(([^']|\\')+)'"#).unwrap(),
    ]
});

pub fn get_links(content_type: &str, url: &Url, document: &str) -> Vec<Url> {
    if content_type.starts_with("text/html") {
        get_links_from_html(&url, &document)
    } else if content_type.starts_with("text/css") {
        get_links_from_css(&url, &document)
    } else {
        Vec::new()
    }
}

fn get_links_from_html(url: &Url, document: &str) -> Vec<Url> {
    let document = scraper::Html::parse_document(&document);
    let base = document
        .select(&Selector::parse("base").unwrap())
        .nth(0)
        .and_then(|x| x.value().attr("href"))
        .and_then(|x| url.join(x).ok())
        .unwrap_or(url.clone())
        .to_owned();

    let mut urls = Vec::new();

    for (selector, attribute) in &*SELECTORS {
        urls.extend(
            document
                .select(&selector)
                .flat_map(|x| x.value().attr(attribute))
                .flat_map(|x| join_url(&base, x)),
        );
    }

    for selected in document.select(&*SRCSET_SELECTOR) {
        let attr = selected.value().attr("srcset").unwrap();
        for splited in attr.split(',') {
            let src = match splited.split_whitespace().nth(0) {
                Some(src) => src,
                None => continue,
            };

            let src = match join_url(&base, &src) {
                Ok(src) => src,
                Err(_) => continue,
            };

            urls.push(src);
        }
    }

    urls
}

fn get_links_from_css(url: &Url, document: &str) -> Vec<Url> {
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

fn join_url(base: &Url, url: &str) -> Result<Url, url::ParseError> {
    if url.starts_with("data:") {
        Url::parse(url)
    } else {
        base.join(url)
    }
}
