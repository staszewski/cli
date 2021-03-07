static INITIAL_URL: &str = "https://raw.githubusercontent.com/lpxxn/rust-design-pattern/master/";
pub struct UrlBuilder {
    url: String,
}

impl UrlBuilder {
    pub fn init() -> UrlBuilder {
        UrlBuilder {
            url: INITIAL_URL.to_string(),
        }
    }
    pub fn append(&mut self, path: &str) {
        self.url = format!("{}{}", self.url, path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_correct_initial_url() {
        assert_eq!(UrlBuilder::init().url, INITIAL_URL);
    }

    #[test]
    fn appent_to_initial_url() {
        let mut url_builder = UrlBuilder::init();
        url_builder.append("d");
        url_builder.append("adasd");
        assert_eq!(
            url_builder.url,
            concat!(
                "https://raw.githubusercontent.com/lpxxn/rust-design-pattern/master/",
                "d",
                "adasd"
            )
        );
    }
}
