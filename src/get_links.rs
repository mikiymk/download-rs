use regex::Regex;
use reqwest::Url;

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
                    .flatten(),
            );
        }
    }

    urls
}

fn get_links_from_css(url: &Url, document: &str) -> Vec<Url> {
    let mut urls = Vec::new();

    for re in &*REGULAR_EXPRESSIONS {
        for cap in re.captures_iter(document) {
            if let Ok(url) = url.join(&cap[1]) {
                urls.push(url);
            }
        }
    }

    urls
}
