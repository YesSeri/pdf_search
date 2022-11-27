#![allow(unused_imports)]
#![allow(dead_code)]

mod search_match;
mod search_status;
mod searcher;
mod fuzzy_finder;
mod powershell;

use crate::searcher::SearchHandler;
use search_status::SearchStatus;
use std::{env, path::PathBuf};
use crate::fuzzy_finder::FuzzyFinder;
use crate::powershell::run_powershell_command;

// fn main() {
    // let fuzzy_finder = FuzzyFinder::new();
    // fuzzy_finder.start();
    // run_powershell_command("echo 'aaa\nbbb\n' | fzf ");
//
// }

fn main() {
    let _args: Vec<String> = env::args().collect();
    let glob = _args[1].clone();
    let search_term = _args[2].clone();

    // let glob = "test_assets/*.pdf".to_string();
    // let search_term = "beautiful".to_string();
    let mut search_handler = SearchHandler::new(&glob, &search_term);
    let search_status = search_handler.search();
    match search_status {
        SearchStatus::Found => {
            search_status.get_status_string();
            let search_matches = search_handler.search_matches.unwrap();
            let mut current_file: PathBuf = PathBuf::new();
            for search_match in search_matches {
                if current_file != search_match.path {
                    println!();
                    current_file = search_match.path.clone();
                    println!("{}:", current_file.display());
                }

                println!("{}", search_match);
            }
        }
        SearchStatus::NoMatchesFound | SearchStatus::NoFilesFound => {
            println!("{}", search_status.get_status_string());
        }
        SearchStatus::NotSearched => {
            search_status.get_status_string();
            panic!("Search status should not be NotSearched")
        }
    }
}
