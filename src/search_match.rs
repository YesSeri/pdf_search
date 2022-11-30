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

    // Used to filter pointless rows containing only ● and stuff like that
    fn string_contains_ascii_letters(s:&str) -> bool{
        s.chars().any(|c| c.is_ascii_alphabetic())
    }
}

impl From<String> for SearchMatch {
    fn from(string: String) -> Self {
        let re_match = Regex::new(r"^.*\.pdf:\d*:Page\s\d*:\s?").unwrap();
        // removes these two lines. Both can optionally end with blank_space
        // file_path/file.pdf:1:Page 1:
        // file_path/file.pdf-6-Page 1:
        let re_only_keep_text = Regex::new(r"^.*\.pdf[:-]\d*[:-]Page\s\d*:\s?").unwrap();

        let result = string.lines().find(|line| {
            re_match.is_match(line)
        });
        let first_match_string = match result {
            None => {
                println!("No match found for string:\n{}", string);
                panic!();
            }
            Some(r) => {
                r
            }
        };
        let context: Vec<String> = string.lines().
            filter_map(|line| {
                // If the text is empty we dont want it, so we return none
                // if there is text, we return Some(text)
                let text = re_only_keep_text.replace(line, "");
                SearchMatch::string_contains_ascii_letters(&text).then_some(text.to_string())
            }).collect();
        let context = context.join("\n");

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
            context,
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
    fn convert_string_empty_line_middle() {
        let string = r#"test_assets/context.pdf-1-Page 1: c
test_assets/context.pdf:2:Page 1: Test
test_assets/context.pdf-3-Page 1:
test_assets/context.pdf-4-Page 1: more text"#.to_string();
        let sm = SearchMatch::from(string);
        let expected_sm = SearchMatch::new(
            PathBuf::from("test_assets/context.pdf"),
            1,
            2,
            "Test".to_string(),
            "c\nTest\nmore text".to_string(),
        );
        assert_eq!(expected_sm.context, sm.context);
        assert_eq!(expected_sm, sm);
    }

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
            "c\nTest\nd".to_string(),
        );
        assert_eq!(sm, expected_sm);
    }
    #[test]
    fn string_contains_ascii(){
        let s1 = "hi there";
        let s2 = "höj there";
        let s3 = "öäå";
        assert!(SearchMatch::string_contains_ascii_letters(s1));
        assert!(SearchMatch::string_contains_ascii_letters(s2));
        assert!(!SearchMatch::string_contains_ascii_letters(s3));
    }

}
