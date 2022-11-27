use crate::search_match::SearchMatch;
use crate::search_status::SearchStatus;
use std::process::Command;

pub struct SearchHandler {
    pub search_status: SearchStatus,
    pub search_matches: Option<Vec<SearchMatch>>,
    glob: String,
    search_term: String,
}

impl SearchHandler {
    pub fn new(glob: &str, search_term: &str) -> SearchHandler {
        SearchHandler {
            search_status: SearchStatus::new(),
            search_matches: None,
            glob: glob.to_string(),
            search_term: search_term.to_string(),
        }
    }
    pub fn search(&mut self) -> SearchStatus {
        let search_hits = self.execute_rga();
        if let Some(search_hits_string) = search_hits {
            self.search_status = SearchStatus::Found;
            self.handle_search_hits(search_hits_string);
            // self.search_matches = Some(parse_search_hits(search_hits.unwrap()));
        } else {
            self.search_status = SearchStatus::NoMatchesFound;
        }
        self.search_status.clone()
    }
    fn execute_rga(&self) -> Option<String> {
        let fixed_arguments = ["rga", "--no-heading", "--line-number", "--path-separator", "/", ];
        let mut powershell = Command::new("powershell.exe");
        let command = powershell.args(fixed_arguments)
            .arg("--glob")
            .arg(&self.glob)
            .arg(&self.search_term);
        let output = command.output().unwrap();
        let result = output.stdout;
        let string = String::from_utf8_lossy(&result).to_string();
        if string.is_empty() {
            None
        } else {
            Some(string)
        }
    }
    fn handle_search_hits(&mut self, result: String) {
        let search_matches: Vec<SearchMatch> = result
            .split_inclusive("\n")
            .map(|s| s.trim().to_string())
            .map(|s| SearchMatch::from(s))
            .collect();
        self.search_matches = Some(search_matches);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_execute_rga_md() {
        let sh = SearchHandler::new("test_assets/test.md", "beautiful");
        let result = sh.execute_rga();
        let expected_result = "test_assets/test.md:5:This is beautiful text.\r\n".to_string();
        assert!(result.is_some());
        assert_eq!(result.unwrap(), expected_result);
    }

    #[test]
    fn test_execute_rga_pdf() {
        let sh = SearchHandler::new("test_assets/test.pdf", "subheading");
        let result = sh.execute_rga();
        let expected_result = "test_assets/test.pdf:2:Page 1: This is a subheading - Test\n".to_string();
        assert!(result.is_some());
        assert_eq!(result.unwrap(), expected_result);
    }

    #[test]
    fn no_such_file() {
        let sh = SearchHandler::new("assets/file_does_not_exist.pdf", "subheading");
        let result = sh.execute_rga();
        let expected_result = None;
        assert_eq!(sh.search_status, SearchStatus::NoFilesFound);
        assert_eq!(result, expected_result);
    }

    fn no_such_file() {
        let sh = SearchHandler::new("assets/*.pdf", "phrase that doesnt exist in test files");
        let result = sh.execute_rga();
        let expected_result = None;
        assert_eq!(sh.search_status, SearchStatus::NoMatchesFound);
    }


    #[test]
    fn search_test() {
        let mut sh = SearchHandler::new("test_assets/test.*", "subheading");
        let result = sh.search();
        let expected_result = SearchStatus::Found;
        assert_eq!(result, expected_result);
        assert_eq!(sh.search_status, SearchStatus::Found);
        assert_eq!(sh.search_matches.unwrap().len(), 2);
    }
}
