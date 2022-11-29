use std::fmt::{Display, Formatter};
use std::path;
use std::{env, fs, path::PathBuf};
use regex::Regex;

#[derive(PartialEq, Debug, Clone)]
pub struct SearchMatch {
    pub path: PathBuf,
    pub page: usize,
    pub line: usize,
    pub content: String,
    pub context: String,
}

impl SearchMatch {
    pub fn new(path: PathBuf, page: usize, line: usize, content: String, context: String) -> SearchMatch {
        SearchMatch {
            path,
            page,
            line,
            content,
            context,
        }
    }

    pub fn fuzzy_display(&self) -> String {
        format!("{} : {} : {}", &self.path.display().to_string(), &self.page.to_string(), &self.to_string())
    }
}

impl From<String> for SearchMatch {
    fn from(string: String) -> Self {
        let re_match = Regex::new(r"\.pdf:\d*:Page\s\d*: ").unwrap();
        let re_only_keep_text = Regex::new(r".*\.pdf[:-]\d*[:-]Page\s\d*: ").unwrap();

        let first_match_string: &str = string.lines().find(|line| {
            re_match.is_match(line)
        }).unwrap();

        let context: String = string.to_string();
        let context = re_only_keep_text.replace_all(&context, "").to_string();

        let mut split = first_match_string.splitn(4, ':');
        let path = split.next().unwrap();
        let line = split.next().unwrap();
        let mut page_chars = split.next().unwrap().chars();
        for _ in 0..=4 {
            page_chars.next();
        }
        let page: usize = page_chars.collect::<String>().parse::<usize>().unwrap();
        let content = split.next().unwrap().trim();
        SearchMatch::new(
            path::PathBuf::from(path),
            page,
            line.parse().unwrap(),
            content.to_string(),
            context.trim().to_string(),
        )
    }
}

impl Display for SearchMatch {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.content)
    }
}

#[cfg(test)]
mod tests {
    use crate::powershell::run_powershell_command;
    use super::*;

    #[test]
    fn convert_string() {
        let cmd = "rga --no-heading --line-number --path-separator / --ignore-case --glob \"context*.pdf\" -i test -C 3";
        let output = run_powershell_command(cmd).unwrap();
        let cmd_result = String::from_utf8_lossy(output.stdout.as_ref());
        let first_result = cmd_result.split("--").collect::<Vec<&str>>()[0];
        let expected_result = r#"test_assets/context.pdf-1-Page 1: c
test_assets/context.pdf:2:Page 1: Test
test_assets/context.pdf-3-Page 1: d
test_assets/context.pdf-4-Page 1:"#;
        assert_eq!(first_result.trim(), expected_result);
        let sm = SearchMatch::from(first_result.to_string());
        let expected_sm = SearchMatch::new(
            PathBuf::from("test_assets/context.pdf"),
            1,
            2,
            "Test".to_string(),
            r#"test_assets/context.pdf-1-Page 1: c
test_assets/context.pdf:2:Page 1: Test
test_assets/context.pdf-3-Page 1: d
test_assets/context.pdf-4-Page 1:"#.to_string(),
        );
        assert_eq!(sm, expected_sm);
// let search_matches: SearchMatch = cmd_result
//             .map(|s| s.trim().to_string())
//             .map(SearchMatch::from)
//             .collect();
        // let string = "test_assets/test.pdf:2:Page 1: This is a subheading - Test\n".to_string();
        // let expected = SearchMatch::new(
        //     path::PathBuf::from("test_assets/test.pdf"),
        //     1,
        //     2,
        //     "This is a subheading - Test".to_string(),
        // );
        // let search_match = SearchMatch::from(string);
        // assert_eq!(search_match, expected);
    }
    // #[test]
    // fn convert_string() {
    //     let string = "test_assets/test.pdf:2:Page 1: This is a subheading - Test\n".to_string();
    //     let expected = SearchMatch::new(
    //         path::PathBuf::from("test_assets/test.pdf"),
    //         1,
    //         2,
    //         "This is a subheading - Test".to_string(),
    //     );
    //     let search_match = SearchMatch::from(string);
    //     assert_eq!(search_match, expected);
    // }
    //
    // #[test]
    // fn display_test() {
    //     let string = "test_assets/test.pdf:2:Page 1: This is a subheading - Test\n".to_string();
    //     let search_match = SearchMatch::from(string);
    //     assert_eq!(search_match.to_string(), "Page 1: This is a subheading - Test");
    // }
}
