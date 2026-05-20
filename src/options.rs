pub struct WebpageOptions {
    allow_insecure: bool,
    follow_location: bool,
    max_redirections: u32,
    timeout: std::time::Duration,
    useragent: String,
    headers: Vec<String>,
}

#[non_exhaustive]
pub struct Link {
    pub url: String,
    pub text: String,
}

use webpage::{Webpage, WebpageOptions};

let mut options = WebpageOptions::default();
options.allow_insecure = true;
let info = Webpage::from_url("https://example.org", options).expect("Halp, could not fetch");
