use webpage::{Webpage, WebpageOptions, Link};

#[allow(dead_code)]
pub trait StringUtils {
    fn substring(&self, start: usize, len: usize) -> Self;
}

pub impl StringUtils for String {
    fn substring(&self, start: usize, len: usize) -> Self {
        self.chars().skip(start).take(len).collect()
    }
}

pub struct WebsiteData {
    pub title: String,
    pub formatted_body: String,
    pub links: Vec<Link>,
}

pub fn remove_whitespace(s: &str) -> String {
    s.chars().filter(|c| !c.is_whitespace()).collect()
}

pub fn get_site_data(url: &str) -> WebsiteData {
    let ws_page = Webpage::from_url(url, WebpageOptions::default()).unwrap();
    let html = ws_page.html;
    let mut concat: String = String::from("");
    for line in html.text_content.lines() {
        let trimmed = remove_whitespace(line);
        if !trimmed.is_empty() {
            concat = format!("{concat}{trimmed}");
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

fn main() {
    let data: WebsiteData = get_site_data(&"https://en.wikipedia.org/wiki/Web_scraping");
    print_site_data(&data);
}
