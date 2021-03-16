use select::document::Document;
use select::predicate::Name;
use std::env;
use std::io::{Error, ErrorKind};
use url::Url;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let website_address = &args[1];
    let url_website_address = Url::parse(website_address)?;
    let base = base_url(url_website_address).unwrap();
    let document = fetch_links_from_website(website_address).unwrap();
    let links = get_links_from_document(&document);
    get_status_code_from_links(links, &base.to_string());
    Ok(())
}

fn fetch_links_from_website(address: &String) -> Result<Document, Box<dyn std::error::Error>> {
    let resp = reqwest::blocking::get(address)?.text()?;
    let document = Document::from(resp.as_str());
    // Add match arm
    Ok(document)
}

fn get_links_from_document<'a>(document: &'a Document) -> Vec<&'a str> {
    let mut links: Vec<&str> = vec![];
    for node in document.find(Name("a")) {
        links.push(node.attr("href").unwrap())
    }
    links
}

fn validate_link(link: String, website_address: &String) -> String {
    if link.contains("https") {
        link
    } else {
        let relative_link = format!("{}{}", website_address.to_owned(), link.clone());
        relative_link
    }
}

fn get_status_code_from_links(
    links: Vec<&str>,
    base_link: &String,
) -> Result<(), Box<(dyn std::error::Error + 'static)>> {
    println!("{:?}", links);
    for link in links {
        println!("{}", &validate_link(link.to_string(), base_link));
        let resp = reqwest::blocking::get(&validate_link(link.to_string(), base_link))?;
        if resp.status().is_success() {
            println!("success!");
        } else if resp.status().is_server_error() {
            println!("server error!");
        } else {
            println!("Something else happened. Status: {:?}", resp.status());
        }
    }
    Ok(())
}

fn base_url(mut url: Url) -> Result<Url, ()> {
    match url.path_segments_mut() {
        Ok(mut path) => {
            path.clear();
        }
        Err(error) => {
            return Err(error);
        }
    }

    url.set_query(None);

    Ok(url)
}
