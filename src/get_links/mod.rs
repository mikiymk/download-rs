use reqwest::Url;

pub fn get_links(content_type: &str, url: &Url, document: &str) -> Vec<Url> {
    if content_type.starts_with("text/html") {
        html::get_links_from_html(url, document)
    } else if content_type.starts_with("text/css") {
        css::get_links_from_css(url, document)
    } else {
        Vec::new()
    }
}

fn join_url(base: &Url, url: &str) -> Result<Url, url::ParseError> {
    if url.starts_with("data:") {
        Url::parse(url)
    } else {
        base.join(url)
    }
}

#[test]
fn test_join_url() {
    let base = Url::parse("https://adm-sidem-gs.idolmaster-official.jp/story/#").unwrap();
    let url = "..//wsm/";

    let joined = join_url(&base, url).unwrap();

    assert_eq!(
        joined,
        Url::parse("https://adm-sidem-gs.idolmaster-official.jp/wsm/").unwrap()
    )
}

mod css;
mod html;
