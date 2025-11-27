use std::os::unix::fs::MetadataExt;
use std::os::unix::fs::PermissionsExt;
use std::{
    fs::{self},
    path::{Path, PathBuf},
    time::UNIX_EPOCH,
};

use chrono::DateTime;
use regex::Regex;

use crate::utils::parser::get_formatted_args;
use crate::{
    commands::{cd::get_curr_directory, Command, CommandResult},
    enums::actions::Action,
    executor::execute_with_redirect,
    utils::{
        printer::{self},
        writer::{self, make_file},
    },
};

pub struct Ls;
impl Command for Ls {
    fn name(&self) -> &'static str {
        "ls"
    }
    fn run(&self, cmd: &str) -> CommandResult {
        let command = cmd.replace("ls ", "");
        return ls(&command);
    }
}

pub struct LsArgs {
    sort: bool,
    long: bool,
}

pub fn ls(command: &str) -> CommandResult {
    return execute_with_redirect(&command, execute_ls, default_ls_with_command);
}

pub fn execute_ls(output_path: &PathBuf, command: &String, args: Vec<String>, executor: &Action) {
    let ls_args = check_ls_args(args);

    match get_ls_results(command) {
        Ok(mut lines) => {
            if ls_args.sort {
                lines.sort();
            }

            if ls_args.long {
                lines = lines.iter().map(|x| get_file_metadata(x)).collect()
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

pub fn get_file_metadata(path: &str) -> String {
    let path = Path::new(path);
    if let Ok(metadata) = fs::metadata(path) {
        let file_type = if metadata.is_dir() {
            "d"
        } else if metadata.is_file() {
            "-"
        } else if metadata.file_type().is_symlink() {
            "l"
        } else {
            "?"
        };

        let mode = metadata.permissions().mode();
        let perms = format!(
            "{}{}{}{}{}{}{}{}{}",
            if mode & 0o400 != 0 { "r" } else { "-" },
            if mode & 0o200 != 0 { "w" } else { "-" },
            if mode & 0o100 != 0 { "x" } else { "-" },
            if mode & 0o040 != 0 { "r" } else { "-" },
            if mode & 0o020 != 0 { "w" } else { "-" },
            if mode & 0o010 != 0 { "x" } else { "-" },
            if mode & 0o004 != 0 { "r" } else { "-" },
            if mode & 0o002 != 0 { "w" } else { "-" },
            if mode & 0o001 != 0 { "x" } else { "-" },
        );

        let nlink = metadata.nlink();

        let uid = metadata.uid();
        let gid = metadata.gid();

        let size = metadata.len();

        if let Ok(mtime) = metadata.modified() {
            let duration = mtime.duration_since(UNIX_EPOCH).unwrap();
            let datetime = DateTime::from_timestamp(duration.as_secs() as i64, 0).unwrap();

            let result = format!(
                "{}{} {:>2} {}:{} {:>5} {} {}",
                file_type,
                perms,
                nlink,
                uid,
                gid,
                size,
                datetime,
                path.display()
            );

            return result;
        }
    }

    return path.to_string_lossy().to_string();
}

pub fn default_ls_with_command(command: &str) -> CommandResult {
    let args = get_formatted_args(command);
    let ls_args = check_ls_args(args);

    let re: Regex = Regex::new(r"-[a-zA-Z]+").unwrap();
    let command = re.replace(command, "").trim().to_string();

    if command.is_empty() {
        return default_ls(ls_args);
    }

    match get_ls_results(&command) {
        Ok(lines) => {
            return CommandResult::Output(lines.join("\n"));
        }
        Err(err) => {
            println!("{}", err);
            return CommandResult::Failed;
        }
    }
}

pub fn default_ls(ls_args: LsArgs) -> CommandResult {
    let curr_dir = get_curr_directory();

    if let Some(entries) = fs::read_dir(curr_dir).ok() {
        let mut lines = Vec::new();

        for entry in entries.filter_map(|e| e.ok()) {
            if ls_args.long {
                let file_name = get_file_metadata(&entry.file_name().to_string_lossy().to_string());
                lines.push(file_name);
            } else {
                let file_name = entry.file_name().to_string_lossy().to_string();
                lines.push(file_name);
            }
        }

        if ls_args.sort {
            lines.sort();
        }

        return CommandResult::Output(lines.join("\n"));
    }

    return CommandResult::Failed;
}

pub fn check_ls_args(args: Vec<String>) -> LsArgs {
    let ls_args = LsArgs {
        sort: args.contains(&"-1".to_string()),
        long: args.contains(&"-la".to_string()),
    };

    return ls_args;
}
