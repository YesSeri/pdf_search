mod search_match;
mod search_status;
mod searcher;

use std::env;
use search_match::SearchMatch;
use search_status::SearchStatus;
use crate::searcher::SearchHandler;

fn main() {
    let _args: Vec<String> = env::args().collect();
    // let glob = args[1].clone();
    // let search_term = args[2].clone();

    let glob = "test_assets/asjdnkaowiaoijodijoasd";
    let search_term = "beautiful";
    let mut search_handler = SearchHandler::new(glob, search_term);
    let search_status = search_handler.search();
    match search_status {
        SearchStatus::Found => {
            let search_matches = search_handler.search_matches.unwrap();
            for search_match in search_matches {
                println!("{}", search_match);
            }
        }
        SearchStatus::NoMatchesFound => {
            println!("No matches found.");
        }
        SearchStatus::NotSearched => { panic!("Search status should not be NotSearched") }
    }
// println!("{:?}", _search_matches);
}


