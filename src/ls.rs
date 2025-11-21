use std::{
    fs::{self},
    path::PathBuf,
};

use crate::{
    cd::get_curr_directory,
    enums::actions::Action,
    executor::execute_with_redirect,
    utils::{
        printer::{self, print_lines},
        writer::{self, make_file},
    },
};

pub struct LsArgs {
    sort: bool,
}

pub fn ls(command: String) {
    let command_wo_ls = command.replace("ls ", "");
    execute_with_redirect(&command_wo_ls, execute_ls, default_ls_with_command);
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
                    make_file(output_path.to_owned())
                }
                Action::AppendStderr => {
                    printer::print_lines(lines);
                }
                Action::RedirectStderr => {
                    printer::print_lines(lines);
                    make_file(output_path.to_owned())
                }
                Action::RedirectStdout => {
                    let _ = writer::write(output_path.to_path_buf(), lines);
                }
            }
        }
        Err(err) => match executor {
            Action::AppendStdout => {
                println!("{}", err);
                make_file(output_path.to_owned())
            }
            Action::AppendStderr => {
                let _ = writer::append(output_path.to_path_buf(), vec![err]);
            }
            Action::RedirectStdout => {
                println!("{}", err);
                make_file(output_path.to_owned())
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

pub fn default_ls_with_command(command: &str) {
    if command == "ls" {
        default_ls();
        return;
    }

    match get_ls_results(command) {
        Ok(lines) => {
            for line in lines {
                println!("{}", line)
            }
        }
        Err(err) => {
            println!("\r{}", err)
        }
    }
}

pub fn default_ls() {
    let mut lines = Vec::new();
    let curr_dir = get_curr_directory();

    match fs::read_dir(curr_dir) {
        Ok(entries) => {
            for entry in entries {
                let entry = entry.unwrap();
                let file_name = entry.file_name().to_string_lossy().to_string();
                lines.push(file_name);
            }

            print_lines(lines);
        }
        Err(_) => {
            println!("ls failed");
        }
    }
}

pub fn check_ls_args(args: Vec<String>) -> LsArgs {
    let ls_args = LsArgs {
        sort: args.contains(&"-1".to_string()),
    };

    return ls_args;
}
