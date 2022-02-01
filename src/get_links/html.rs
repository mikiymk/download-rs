use super::join_url;
use once_cell::sync::Lazy;
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

pub fn get_links_from_html(url: &Url, document: &str) -> Vec<Url> {
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
        let links = match get_links_from_html_srcset(attr, &base) {
            Some(links) => links,
            None => continue,
        };
        urls.extend(links);
    }

    urls
}

fn get_links_from_html_srcset(srcset: &str, base: &Url) -> Option<Vec<Url>> {
    let mut urls = Vec::new();
    for splited in srcset.split(',') {
        let src = splited.split_whitespace().nth(0)?;

        let src = join_url(&base, &src).ok()?;

        urls.push(src);
    }

    Some(urls)
}

#[test]
fn test_srcset() {
    let srcset = "../assets/img/footer/banner-sp.png";
    let url = Url::parse("https://sidem-gs.idolmaster-official.jp/collabo/contact/faq/").unwrap();
    let expect = Url::parse(
        "https://sidem-gs.idolmaster-official.jp/collabo/contact/assets/img/footer/banner-sp.png",
    )
    .unwrap();

    let links = get_links_from_html_srcset(srcset, &url);

    assert_eq!(links, vec![expect])
}
