use webpage::{Webpage, WebpageOptions, Link};

#[allow(dead_code)]
pub trait StringUtils {
    fn substring(&self, start: usize, len: usize) -> Self;
}

impl StringUtils for String {
    fn substring(&self, start: usize, len: usize) -> Self {
        self.chars().skip(start).take(len).collect()
    }
}

pub enum BodyFormatType {
    BodyCode,
    BodyArticle,
}

pub struct WebsiteData {
    pub title: String,
    pub formatted_body: String,
    pub links: Vec<Link>,
}

pub fn remove_whitespace(s: &str) -> String {
    s.chars().filter(|c| !c.is_whitespace()).collect()
}

pub fn get_site_data(url: &str, body_format: BodyFormatType, line_delim: Option<char>) -> WebsiteData {
    let ws_page = Webpage::from_url(url, WebpageOptions::default()).expect("Could not read from URL.");
    let http = ws_page.http;
    let delim_excluded = line_delim.is_none();
    assert!(http.headers[0].starts_with("HTTP"));
    assert!(http.body.to_lowercase().starts_with("<!doctype html>"));
    assert_eq!(http.url, url);
    assert_eq!(http.content_type.to_lowercase(), "text/html; charset=UTF-8".to_string().to_lowercase());

    let html = ws_page.html;
    let mut concat: String = String::from("");
    for line in html.text_content.lines() {
        let trimmed = remove_whitespace(&line);
        if !trimmed.is_empty() {
            if !delim_excluded { concat = format!("{concat}{trimmed}{}", line_delim.unwrap_or('\0')); }
            else { concat = format!("{concat}{trimmed}"); }
        }
    }
    return WebsiteData {
        title: html.title.unwrap(),
        formatted_body: concat,
        links: html.links,
    };
}

pub fn print_site_data(data: &WebsiteData) {
    println!("Title: {}\nBody:\n{}", data.title, data.formatted_body);
    for link in &data.links {
        if link.text.is_empty() { println!("Link: {}\n", link.url); }
        else { println!("Name: {}\nLink: {}\n", link.text, link.url); }
    }
}

pub fn get_html_code(url: &str) {
    let page = Webpage::from_url(url, WebpageOptions::default()).expect("Could not read from URL.");

    let http = page.http;
    let body = http.body;
    let mut last_delim: usize = 0;
    let mut content_slice: String = String::from("");
    for (i, c) in body.char_indices() {
        if c == '<' {
            let slice = &body[last_delim..i];
            let cleaned = remove_whitespace(slice);

            if cleaned != content_slice {
                content_slice = cleaned.to_string();
                println!("\t<{content_slice}\n");
            }
            last_delim = i+1;
        }
    }
}

// future plan to add async for ease of use
fn main() {
    //let data: WebsiteData = get_site_data("https://users.rust-lang.org/t/which-syntax-is-preferred-for-checking-if-optionally-is-none/91939", BodyFormatType::BodyCode, None);
    //print_site_data(&data);

    get_html_code("https://users.rust-lang.org/t/which-syntax-is-preferred-for-checking-if-optionally-is-none/91939");
}
