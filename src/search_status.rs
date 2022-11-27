use std::process::Output;

#[non_exhaustive]
#[derive(Debug, PartialEq, Clone)]
pub enum SearchStatus {
    Found,
    NotFound(String),
    NotSearched,
}

impl SearchStatus {}


impl SearchStatus {
    pub fn new() -> SearchStatus {
        SearchStatus::NotSearched
    }
}

impl From<&Output> for SearchStatus {
    fn from(output: &Output) -> Self {
        let error_message = String::from_utf8_lossy(&output.stderr).to_string();
        if output.status.success() {
            if output.stdout.is_empty() {
                return SearchStatus::NotFound(String::from_utf8_lossy(&output.stderr).parse().unwrap());
            } else {
                return SearchStatus::Found;
            }
        } else {
            panic!("Error in conversion to SearchStatus.");
        }
    }
}

mod tests {
    use super::*;

    #[test]
    fn just_created() {
        let status = SearchStatus::new();
        assert_eq!(SearchStatus::NotSearched, status)
    }
}