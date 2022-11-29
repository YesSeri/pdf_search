#![allow(unused_imports)]
#![allow(dead_code)]

mod search_match;
mod search_status;
mod searcher;
mod fuzzy_finder;
mod powershell;
mod tui;
mod tui_example;

use crate::searcher::SearchHandler;
use search_status::SearchStatus;
use std::{env, fs, path::PathBuf};
use crate::fuzzy_finder::FuzzyFinder;
use crate::powershell::run_powershell_command;


fn main() {
    // tui_example::run();
    let _args: Vec<String> = env::args().collect();
    let glob = {
        let g = _args[1].clone();
        if g.ends_with(".pdf") {
            g
        } else {
            format!("{g}.pdf")
        }
    };
    let search_term = _args[2].clone();
    println!("Press up and down to select, enter to open file, q to exit without opening.");

    let mut search_handler = SearchHandler::new(&glob, &search_term);
    search_handler.search();

    if let Some(search_matches) = search_handler.search_matches {
        let selected_match = tui::run(search_matches);
        if let Ok(selected_match) = selected_match {
            let pdf_absolute_path = make_absolute_remove_long_path(&selected_match.path);
            let sumatra_exe = find_sumatra_exe();
            let command = format!("Start-Process -FilePath '{}' '\"{}\" -page {}'", sumatra_exe, pdf_absolute_path, &selected_match.page.to_string());

            println!("opening {}, page {}", pdf_absolute_path, &selected_match.page.to_string());
            let _ = run_powershell_command(command.as_str()).unwrap();
        } else {
            println!("Application has quit without selecting file.");
        }
    }
    println!("Exiting program");
}

fn make_absolute_remove_long_path(path: &PathBuf) -> String {
    let absolute_path = fs::canonicalize(path).unwrap().display().to_string();
    let mut chars = absolute_path.chars();
    for _ in 0..=3 {
        chars.next();
    }
    chars.collect()
}

fn find_sumatra_exe() -> String {
    const SUMATRA_EXE_NAME: &str = "SumatraPDF-3.4.6-64.exe";
    let sumatra_exe_dir = if cfg!(debug_assertions) {
        println!("Debugging enabled");
        // env::current_dir().unwrap()
        PathBuf::from("C:\\Programming\\rust\\pdfSearch")
    } else {
        env::current_exe().unwrap().join("..")
    };
    make_absolute_remove_long_path(&sumatra_exe_dir.join(SUMATRA_EXE_NAME))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_sumatra_exe() {
        let sumatra_exe = find_sumatra_exe();
        assert_eq!(sumatra_exe, "C:\\Programming\\rust\\pdfSearch\\SumatraPDF-3.4.6-64.exe");
    }
}
