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
            if arg.starts_with("--allow-url=") {
                let str = format!(r"^{}.*", &arg[12..]);
                allows.push(Regex::new(&str)?)
            } else if arg.starts_with("--allow-domain=") {
                let str = format!(r"https?://([^/.]\.)*{}/", &arg[15..]);
                allows.push(Regex::new(&str)?)
            } else if arg.starts_with("--allow-pattern=") {
                allows.push(Regex::new(&arg[16..])?)
            } else if arg.starts_with("--ignore-url=") {
                let str = format!(r"^{}.*", &arg[13..]);
                ignores.push(Regex::new(&str)?)
            } else if arg.starts_with("--ignore-domain=") {
                let str = format!(r"https?://([^/.]\.)*{}/", &arg[16..]);
                ignores.push(Regex::new(&str)?)
            } else if arg.starts_with("--ignore-pattern=") {
                allows.push(Regex::new(&arg[17..])?)
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
        } else if self.allows.is_allow_url(url) {
            true
        } else {
            false
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
