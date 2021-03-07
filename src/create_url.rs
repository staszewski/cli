static INITIAL_URL: &str = "https://raw.githubusercontent.com/lpxxn/rust-design-pattern/master/";
pub struct CreateUrl<'a> {
    url: &'a str,
}
// trait?
impl<'a> CreateUrl<'a> {
    pub fn init() -> CreateUrl<'static> {
        CreateUrl { url: INITIAL_URL }
    }
    pub fn append(&self, path: &str) -> String {
        let newstr = format!("{}{}", self.url, path);
        newstr
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_correct_initial_url() {
        assert_eq!(CreateUrl::init().url, INITIAL_URL);
    }

    #[test]
    fn appent_to_initial_url() {
        assert_eq!(CreateUrl::init().append("lala"), INITIAL_URL);
    }
}
