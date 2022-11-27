#[non_exhaustive]
#[derive(Debug, PartialEq, Clone)]
pub enum SearchStatus {
    Found,
    NoMatchesFound,
    NoFilesFound,
    NotSearched,
}

impl SearchStatus {}


impl SearchStatus {
    pub fn new() -> SearchStatus {
        SearchStatus::NotSearched
    }
}

mod tests{
    use super::*;
    #[test]
    fn just_created(){
        let status = SearchStatus::new();
        assert_eq!(SearchStatus::NotSearched, status)
    }
}