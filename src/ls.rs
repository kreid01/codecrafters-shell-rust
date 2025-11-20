use std::{
    fs::{self},
    path::PathBuf,
};

use crate::{executor::execute_with_redirect, redirect::Action, utils::writer};

pub struct LsArgs {
    sort: bool,
}

pub fn ls(command: String) {
    let command_wo_ls = command.replace("ls ", "");
    execute_with_redirect(&command_wo_ls, execute_ls, default_ls);
}

pub fn execute_ls(output_path: &PathBuf, command: &String, args: Vec<String>, executor: &Action) {
    let ls_args = check_ls_args(args);
    let mut lines = get_ls_results(command);

    if ls_args.sort {
        lines.sort();
    }

    match executor {
        Action::Append => {
            let _ = writer::append(output_path.to_path_buf(), lines);
        }
        _ => {
            let _ = writer::write(output_path.to_path_buf(), lines);
        }
    }
}

pub fn get_ls_results(command: &str) -> Vec<String> {
    let mut lines: Vec<String> = Vec::new();

    match fs::read_dir(command) {
        Ok(entries) => {
            for entry in entries {
                let entry = entry.unwrap();
                let file_name = entry.file_name().to_string_lossy().to_string();
                lines.push(file_name);
            }
        }
        Err(_) => {
            let error = format!("ls: {}: No such file or directory", command);
            lines.push(error);
        }
    }

    return lines;
}

pub fn default_ls(command: &str) {
    if fs::read_dir(command).is_err() {
        println!("ls: {}: No such file or directory", command)
    }

    let lines = get_ls_results(command);
    for line in lines {
        print!("{}", line)
    }
}

pub fn check_ls_args(args: Vec<String>) -> LsArgs {
    let ls_args = LsArgs {
        sort: args.contains(&"-1".to_string()),
    };

    return ls_args;
}
