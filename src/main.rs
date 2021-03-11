use select::document::Document;
use select::predicate::Name;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let website_address = &args[1];
    println!("{}", website_address);
    fetch_links_from_website(website_address);
}

fn fetch_links_from_website(address: &String) -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::blocking::get(address)?.text()?;
    let document = Document::from(resp.as_str());
    let mut links: Vec<&str> = vec![];
    for node in document.find(Name("a")) {
        links.push(node.attr("href").unwrap())
    }
    println!("{:?}", links.len());
    Ok(())
}
