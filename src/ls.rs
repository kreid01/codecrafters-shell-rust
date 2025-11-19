use std::{
    fs::{self},
    path::{self, PathBuf},
};

use crate::{change_directory, redirect, writer};

const LS_ARGS: [&str; 1] = ["-1"];

pub struct LsArgs {
    sort: bool,
}

pub fn ls(command: String) {
    let command_wo_ls = command.replace("ls ", "");

    match command {
        _redirect_error if command.contains("1>") => {
            redirect::redirect_stdout(&command_wo_ls, write_ls);
        }
        _redirect_output if command.contains("2>") => {
            redirect::redirect_stderr(&command_wo_ls, write_ls);
        }
        _ => {
            let current_dir = change_directory::get_curr_directory();
            for x in current_dir.iter() {
                println!("{}", x.display());
            }
        }
    }
}

pub fn check_ls_args(args: Vec<String>) -> (LsArgs, Vec<String>) {
    let ls_args = LsArgs {
        sort: args.contains(&"-1".to_string()),
    };

    let remaining_args: Vec<String> = args
        .iter()
        .filter(|x| !LS_ARGS.contains(&x.as_str()))
        .cloned()
        .collect();

    (ls_args, remaining_args)
}

pub fn write_ls(output_path: PathBuf, args: Vec<String>) {
    let mut lines: Vec<String> = Vec::new();
    let (ls_args, checked_args) = check_ls_args(args);

    for arg in checked_args {
        if let Some(error) = Some(check_dir_not_found("ls".to_string(), &arg, true)) {
            lines.push(error.unwrap());
            continue;
        }

        let entries = fs::read_dir(arg).expect("Cannot read directory");

        for entry in entries {
            let entry = entry.unwrap();
            let file_name = entry.file_name().to_string_lossy().to_string();
            lines.push(file_name);
        }
    }

    if ls_args.sort {
        lines.sort();
    }

    let _ = writer::write(output_path.to_path_buf(), lines);
}

fn check_path(path: &str) -> bool {
    return !path::Path::new(path).is_dir();
}

fn check_dir_not_found(command: String, path: &str, redirect: bool) -> Option<String> {
    if check_path(&path) {
        let result = format!("{}: {}: No such file or directory", command, path);
        if redirect {
            return Some(result);
        }

        println!("{}", result);
        return None;
    }

    return None;
}
