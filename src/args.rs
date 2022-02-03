use regex::Regex;
use reqwest::Url;
use std::env::Args;

pub struct Arguments {
    starts: Vec<Url>,
    allows: Rule,
    ignores: Rule,
}

struct Rule {
    patterns: Vec<Regex>,
}

impl Arguments {
    pub fn new(mut args: Args) -> Result<Arguments, Box<dyn std::error::Error>> {
        let mut starts = Vec::new();
        let mut allows = Vec::new();
        let mut ignores = Vec::new();

        args.next();

        for arg in args {
            if let Some(pattern) = arg.strip_prefix("--allow-url=") {
                let str = format!(r"^{}.*", pattern);
                allows.push(Regex::new(&str)?)
            } else if let Some(pattern) = arg.strip_prefix("--allow-domain=") {
                let str = format!(r"https?://([^/.]\.)*{}/", pattern);
                allows.push(Regex::new(&str)?)
            } else if let Some(pattern) = arg.strip_prefix("--allow-pattern=") {
                allows.push(Regex::new(pattern)?)
            } else if let Some(pattern) = arg.strip_prefix("--ignore-url=") {
                let str = format!(r"^{}.*", pattern);
                ignores.push(Regex::new(&str)?)
            } else if let Some(pattern) = arg.strip_prefix("--ignore-domain=") {
                let str = format!(r"https?://([^/.]\.)*{}/", pattern);
                ignores.push(Regex::new(&str)?)
            } else if let Some(pattern) = arg.strip_prefix("--ignore-pattern=") {
                allows.push(Regex::new(pattern)?)
            } else {
                starts.push(Url::parse(&arg)?);
            }
        }

        if starts.is_empty() {
            starts.push(Url::parse("https://example.com")?);
        }

        if allows.is_empty() {
            let url = &starts[0];
            let url = format!(
                "^{}://{}",
                url.scheme(),
                url.host().ok_or("host dont have")?
            );
            allows.push(Regex::new(&url)?);
        }

        Ok(Arguments {
            starts,
            allows: Rule { patterns: allows },
            ignores: Rule { patterns: ignores },
        })
    }

    pub fn starts(&self) -> &Vec<Url> {
        &self.starts
    }

    pub fn is_allow_url(&self, url: &Url) -> bool {
        if self.ignores.is_allow_url(url) {
            false
        } else {
            self.allows.is_allow_url(url)
        }
    }
}

impl Rule {
    fn is_allow_url(&self, url: &Url) -> bool {
        for pattern in &self.patterns {
            let url_str = url.to_string();
            if pattern.is_match(&url_str) {
                return true;
            }
        }
        false
    }
}
