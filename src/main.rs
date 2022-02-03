// TODO
// use javascript
// xx redirect
// x data scheme
// x allow all subdomaion
// testing
// naming

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    use std::env;
    use web_page_downloader::args::Arguments;

    let args = match Arguments::new(env::args()) {
        Ok(args) => args,
        Err(err) => {
            eprintln!("Can't get args");
            return Err(err);
        }
    };

    web_page_downloader::run(args).await
}
