use std::env;
use std::path::Path;

use crate::commands::cat::get_cat_result;
use crate::utils::writer;

pub fn history(cmd: &str, history: &mut Vec<String>, appended_history: &mut Vec<String>) {
    let command = cmd.replace("history ", "");
    match command {
        command if command.starts_with("-r ") => {
            let mut file_history = read_file_history(&command);
            history.append(&mut file_history);
            appended_history.append(&mut file_history);
        }
        command if command.starts_with("-w ") => write_file_history(&command, appended_history),
        command if command.starts_with("-a ") => {
            append_file_history(&command, appended_history);
            appended_history.clear();
        }
        _ => default_history(&command, history),
    }
}

pub fn default_history(cmd: &str, history: &[String]) {
    let diff = match cmd.parse::<usize>() {
        Ok(count) => history.len() - count,
        Err(_) => 0,
    };

    for (i, command) in history.iter().enumerate().skip(diff) {
        println!("\t{} {}", i + 1, command)
    }
}

pub fn read_file_history(cmd: &str) -> Vec<String> {
    let cmd = cmd.replace("-r ", "");
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
    let cmd = Path::new(&cmd.replace("-w ", "")).to_path_buf();
    let _ = writer::write(cmd, history.to_owned());
}

pub fn append_file_history(cmd: &str, history: &[String]) {
    let cmd = Path::new(&cmd.replace("-a ", "")).to_path_buf();
    let _ = writer::append(cmd, history.to_owned());
}

pub fn get_history_env() -> (Vec<String>, Vec<String>) {
    let mut history: Vec<String> = Vec::new();
    let mut appended_history: Vec<String> = Vec::new();

    if let Ok(history_env) = env::var("HISTFILE") {
        if let Ok(history_cat) = get_cat_result(&history_env) {
            for line in history_cat.lines() {
                history.push(line.to_string());
                appended_history.push(line.to_string());
            }
        }
    }

    (history, appended_history)
}

pub fn write_history_env(history: Vec<String>) {
    if let Ok(history_env) = env::var("HISTFILE") {
        let (history_env_arr, _) = get_history_env();

        let history: Vec<String> = history
            .iter()
            .filter(|x| !history_env_arr.contains(&x.trim().to_string()))
            .map(|x| x.to_string())
            .collect();

        let path = Path::new(&history_env).to_path_buf();
        let _ = writer::append(path, history);
    }
}
