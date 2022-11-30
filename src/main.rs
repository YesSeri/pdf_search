#![allow(unused_imports)]
#![allow(dead_code)]

mod search_match;
mod search_status;
mod searcher;
mod fuzzy_finder;
mod powershell;
mod tui;
mod pdf_opener;

use crate::searcher::SearchHandler;
use search_status::SearchStatus;
use std::{env, fs, path::PathBuf};
use crossterm::terminal::ClearType;
use crate::fuzzy_finder::FuzzyFinder;
use crate::pdf_opener::delete_settings_file;
use crate::powershell::run_powershell_command;


fn main() {
    // tui_example::run();
    let _args: Vec<String> = env::args().collect();
    let glob = _args[1].clone();
    let search_term = _args[2].clone();
    println!("Press up and down to select, enter to open file, q to exit without opening.");

    let mut search_handler = SearchHandler::new(&glob, &search_term);
    search_handler.search();

    if let Some(search_matches) = search_handler.search_matches {
        tui::run(search_matches, &search_term).unwrap();
        delete_settings_file();
    } else {
        println!("No matches found.");
    }
    println!("Application has shutdown.");
}


