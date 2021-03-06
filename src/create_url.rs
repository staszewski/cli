static INITIAL_URL: &str = "https://raw.githubusercontent.com/lpxxn/rust-design-pattern/master/";
pub struct CreateUrl<'a> {
    url: &'a str,
}

impl<'a> CreateUrl<'a> {
    pub fn init() -> CreateUrl<'static> {
        CreateUrl { url: INITIAL_URL }
    }
    pub fn append(&self) {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_correct_initial_url() {
        assert_eq!(CreateUrl::init().url, INITIAL_URL);
    }
}
