use std::{
    fs::{self},
    path::PathBuf,
};

use crate::{
    enums::Action,
    executor::execute_with_redirect,
    utils::{
        printer,
        writer::{self, make_dir},
    },
};

pub struct LsArgs {
    sort: bool,
}

pub fn ls(command: String) {
    let command_wo_ls = command.replace("ls ", "");
    execute_with_redirect(&command_wo_ls, execute_ls, default_ls);
}

pub fn execute_ls(output_path: &PathBuf, command: &String, args: Vec<String>, executor: &Action) {
    let ls_args = check_ls_args(args);

    match get_ls_results(command) {
        Ok(mut lines) => {
            if ls_args.sort {
                lines.sort();
            }

            match executor {
                Action::AppendStdout => {
                    let _ = writer::append(output_path.to_path_buf(), lines);
                }
                Action::AppendStderr => {
                    printer::print_lines(lines);
                    make_dir(output_path.to_owned())
                }
                Action::RedirectStderr => {
                    printer::print_lines(lines);
                    make_dir(output_path.to_owned())
                }
                Action::RedirectStdout => {
                    let _ = writer::write(output_path.to_path_buf(), lines);
                }
            }
        }
        Err(err) => match executor {
            Action::AppendStdout => {
                println!("{}", err);
                make_dir(output_path.to_owned())
            }
            Action::AppendStderr => {
                let _ = writer::append(output_path.to_path_buf(), vec![err]);
            }
            Action::RedirectStdout => {
                println!("{}", err);
                make_dir(output_path.to_owned())
            }
            Action::RedirectStderr => {
                let _ = writer::write(output_path.to_path_buf(), vec![err]);
            }
        },
    }
}

pub fn get_ls_results(command: &str) -> Result<Vec<String>, String> {
    let mut lines: Vec<String> = Vec::new();

    match fs::read_dir(command) {
        Ok(entries) => {
            for entry in entries {
                let entry = entry.unwrap();
                let file_name = entry.file_name().to_string_lossy().to_string();
                lines.push(file_name);
            }
            return Ok(lines);
        }
        Err(_) => {
            let err = format!("ls: {}: No such file or directory", command);
            return Err(err);
        }
    }
}

pub fn default_ls(command: &str) {
    match get_ls_results(command) {
        Ok(lines) => {
            for line in lines {
                println!("{}", line)
            }
        }
        Err(err) => {
            println!("{}", err)
        }
    }
}

pub fn check_ls_args(args: Vec<String>) -> LsArgs {
    let ls_args = LsArgs {
        sort: args.contains(&"-1".to_string()),
    };

    return ls_args;
}
