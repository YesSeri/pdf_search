use std::fmt::{Display, Formatter};
use std::path;

#[derive(PartialEq, Debug)]
pub struct SearchMatch {
    path: path::PathBuf,
    line: usize,
    pub content: String,
}

impl SearchMatch {
    pub fn new(path: path::PathBuf, line: usize, content: String) -> SearchMatch {
        SearchMatch {
            path,
            line,
            content,
        }
    }
}

impl From<String> for SearchMatch {
    fn from(string: String) -> Self {
        let mut split = string.splitn(3, ":");
        let path = split.next().unwrap();
        let line = split.next().unwrap();
        let content = split.next().unwrap().trim();
        SearchMatch::new(
            path::PathBuf::from(path),
            line.parse().unwrap(),
            content.to_string(),
        )
    }
}

impl Display for SearchMatch {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self
            .path
            .extension()
            .expect("The file should always have and ending.")
            == "pdf"
        {
            let content = self.content.split_once(": ").unwrap().1;
            write!(f, "{}", content)
        } else {
            write!(f, "{}", self.content)
        }
    }
}

mod tests {
    use super::*;

    #[test]
    fn convert_string() {
        let string = "test_assets/test.pdf:2:Page 1: This is a subheading - Test\n".to_string();
        let expected = SearchMatch::new(
            path::PathBuf::from("test_assets/test.pdf"),
            2,
            "Page 1: This is a subheading - Test".to_string(),
        );
        let search_match = SearchMatch::from(string);
        assert_eq!(search_match, expected);
    }

    #[test]
    fn display_test() {
        let string = "test_assets/test.pdf:2:Page 1: This is a subheading - Test\n".to_string();
        let search_match = SearchMatch::from(string);
        assert_eq!(search_match.to_string(), "This is a subheading - Test");
    }
}
