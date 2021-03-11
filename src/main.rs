use select::document::Document;
use select::predicate::Name;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let website_address = &args[1];
    println!("{}", website_address);
    let document = fetch_links_from_website(website_address).unwrap();
    let links = get_links_from_document(&document);
    print!("{:?}", links)
}

fn fetch_links_from_website(address: &String) -> Result<Document, Box<dyn std::error::Error>> {
    let resp = reqwest::blocking::get(address)?.text()?;
    let document = Document::from(resp.as_str());
    Ok(document)
}

fn get_links_from_document<'a>(document: &'a Document) -> Vec<&'a str> {
    let mut links: Vec<&str> = vec![];
    for node in document.find(Name("a")) {
        links.push(node.attr("href").unwrap())
    }
    links
}
