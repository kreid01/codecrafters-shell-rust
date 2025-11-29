use std::path::Path;

use crate::commands::cat::get_cat_result;
use crate::utils::writer;

pub fn history(cmd: &str, history: &[String]) {
    let diff = match cmd.replace("history ", "").parse::<usize>() {
        Ok(count) => history.len() - count,
        Err(_) => 0,
    };

    for (i, command) in history.iter().enumerate().skip(diff) {
        println!("\t{} {}", i + 1, command)
    }
}

pub fn read_file_history(cmd: &str) -> Vec<String> {
    let cmd = cmd.replace("history -r ", "");
    let mut file_history = String::new();

    if let Ok(file) = get_cat_result(&cmd) {
        file_history = file;
    }

    file_history
        .trim()
        .split("\n")
        .map(|s| s.to_string())
        .collect()
}

pub fn write_file_history(cmd: &str, history: &Vec<String>) {
    let cmd = Path::new(&cmd.replace("history -w ", "")).to_path_buf();
    let _ = writer::write(cmd, history.to_owned());
}
