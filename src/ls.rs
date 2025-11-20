use std::{
    fs::{self},
    path::PathBuf,
};

use crate::{
    executor::{execute_with_redirect, Redirect},
    redirect, writer,
};

const LS_ARGS: [&str; 1] = ["-1"];

pub struct LsArgs {
    sort: bool,
}

pub fn ls(command: String) {
    let command_wo_ls = command.replace("ls ", "");
    execute_with_redirect(&command_wo_ls, write_ls, default_ls);
}

pub fn default_ls(command: &str) -> Option<()> {
    if fs::read_dir(command).is_err() {
        return None;
    }

    let lines = get_ls(command);
    for line in lines {
        print!("{}", line)
    }

    return Some(());
}

pub fn check_ls_args(args: Vec<String>) -> LsArgs {
    let ls_args = LsArgs {
        sort: args.contains(&"-1".to_string()),
    };

    return ls_args;
}

pub fn get_ls(command: &str) -> Vec<String> {
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

pub fn write_ls(output_path: &PathBuf, command: &String, args: Vec<String>, redirect: Redirect) {
    let ls_args = check_ls_args(args);
    let mut lines = get_ls(command);

    if ls_args.sort {
        lines.sort();
    }

    match redirect {
        Redirect::Stdout => {}
        Redirect::Stderr => {}
    }
    let _ = writer::write(output_path.to_path_buf(), lines);
}
