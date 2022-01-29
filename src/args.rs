use reqwest::Url;

pub struct Arguments {
    pub starts: Vec<Url>,
    pub allows: Vec<Url>,
    pub ignores: Vec<Url>,
}

impl Arguments {
    pub fn get() -> Result<Arguments, Box<dyn std::error::Error>> {
        let mut starts = Vec::new();
        let mut allows = Vec::new();
        let mut ignores = Vec::new();

        let mut args = std::env::args();

        args.next();

        for arg in args {
            if arg.starts_with("--allow=") {
                allows.push(Url::parse(&arg[8..])?);
            } else if arg.starts_with("--ignore=") {
                ignores.push(Url::parse(&arg[9..])?);
            } else {
                starts.push(Url::parse(&arg)?);
            }
        }

        if starts.is_empty() {
            starts.push(Url::parse("https://example.com")?);
        }
        if allows.is_empty() {
            let url = &starts[0];
            let url = format!("{}://{}", url.scheme(), url.host().ok_or("host dont have")?);
            allows.push(Url::parse(&url)?)
        }

        Ok(Arguments {
            starts,
            allows,
            ignores,
        })
    }

    pub fn is_allow_url(&self, url: &Url) -> bool {
        for ignore in &self.ignores {
            if url.to_string().starts_with(&ignore.to_string()) {
                return false;
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
