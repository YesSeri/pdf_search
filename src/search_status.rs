use std::process::Output;

#[non_exhaustive]
#[derive(Debug, PartialEq, Clone)]
pub enum SearchStatus {
    Found,
    NoFilesFound,
    NoMatchesFound,
    NotSearched,
}

impl SearchStatus {
    pub fn get_status_string(&self) -> String {
        match self {
            SearchStatus::Found => {
                "Found matches:".to_string()
            }
            SearchStatus::NoFilesFound => {
                "No files found, make sure your glob pattern is correct.".to_string()
            }
            SearchStatus::NoMatchesFound => {
                "No matches for your search term has been found".to_string()
            }
            SearchStatus::NotSearched => {
                "The search has not been run".to_string()
            }
        }
    }
}


impl SearchStatus {
    pub fn new() -> SearchStatus {
        SearchStatus::NotSearched
    }
}

impl From<&Output> for SearchStatus {
    fn from(output: &Output) -> Self {
        if output.status.success() {
            if output.stderr.is_empty() {
                if output.stdout.is_empty() {
                    SearchStatus::NoMatchesFound
                } else {
                    SearchStatus::Found
                }
            } else {
                SearchStatus::NoFilesFound
            }
        } else {
            panic!("Error in conversion to SearchStatus.");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn just_created() {
        let status = SearchStatus::new();
        assert_eq!(SearchStatus::NotSearched, status)
    }
}