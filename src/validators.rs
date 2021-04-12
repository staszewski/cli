pub mod validate {
    pub fn validate_link(link: String, website_address: &String) -> String {
        if link.trim() == "/" {
            website_address.to_owned()
        } else if link.contains("https") {
            link
        } else {
            let relative_link = format!(
                "{}{}",
                website_address.to_owned(),
                clean_relative_link(link.clone())
            );
            relative_link
        }
    }

    pub fn clean_relative_link(mut link: String) -> String {
        if link.chars().next().unwrap() == "/".chars().next().unwrap() {
            link.remove(0);
            link
        } else {
            link
        }
    }
}
