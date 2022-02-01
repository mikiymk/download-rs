use regex::Regex;
use reqwest::Url;

pub struct Arguments {
    starts: Vec<Url>,
    allows: Vec<Url>,
    allow_domains: Vec<Regex>,
    ignores: Vec<Url>,
}

impl Arguments {
    pub fn get() -> Result<Arguments, Box<dyn std::error::Error>> {
        let mut starts = Vec::new();
        let mut allows = Vec::new();
        let mut allow_domains = Vec::new();
        let mut ignores = Vec::new();

        let mut args = std::env::args();

        args.next();

        for arg in args {
            if arg.starts_with("--allow-url=") {
                allows.push(Url::parse(&arg[12..])?);
            } else if arg.starts_with("--allow-domain=") {
                let str = format!(r"([^.]\.)*{}$", &arg[15..]);
                allow_domains.push(Regex::new(&str)?)
            } else if arg.starts_with("--ignore=") {
                ignores.push(Url::parse(&arg[9..])?);
            } else {
                starts.push(Url::parse(&arg)?);
            }
        }

        if starts.is_empty() {
            starts.push(Url::parse("https://example.com")?);
        }
        if allows.is_empty() && allow_domains.is_empty() {
            let url = &starts[0];
            let url = format!("{}://{}", url.scheme(), url.host().ok_or("host dont have")?);
            allows.push(Url::parse(&url)?);
        }

        Ok(Arguments {
            starts,
            allows,
            allow_domains,
            ignores,
        })
    }

    pub fn starts(&self) -> &Vec<Url> {
        &self.starts
    }

    pub fn is_allow_url(&self, url: &Url) -> bool {
        for ignore in &self.ignores {
            if url.to_string().starts_with(&ignore.to_string()) {
                return false;
            }
        }

        for allow_domain in &self.allow_domains {
            if let Some(domain) = url.domain() {
                if allow_domain.is_match(domain) {
                    return true;
                }
            }
        }

        for allow in &self.allows {
            if url.to_string().starts_with(&allow.to_string()) {
                return true;
            }
        }

        false
    }
}
