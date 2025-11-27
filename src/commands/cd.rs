use std::env;
use std::path::{Path, PathBuf};

use crate::commands::{Command, CommandResult};

pub struct Cd;
impl Command for Cd {
    fn name(&self) -> &'static str {
        "cd"
    }
    fn run(&self, cmd: &str) -> CommandResult {
        return cd(cmd);
    }
}

pub fn cd(command: &str) -> CommandResult {
    let directory = command.split_whitespace().nth(1).unwrap();

    if directory.starts_with("./") {
        cd_relative(command);
        return CommandResult::Success;
    } else if directory.starts_with("../") {
        cd_back(command);
        return CommandResult::Success;
    } else if directory.starts_with("~") {
        cd_home();
        return CommandResult::Success;
    }

    if cd_absolute(directory).is_none() {
        no_file_or_directory(directory);
    }

    return CommandResult::Success;
}

pub fn cd_absolute(directory: &str) -> Option<()> {
    let path = Path::new(directory);
    let new_dir_path = path.join(directory);
    return go_to_directory(new_dir_path);
}

pub fn cd_home() -> Option<()> {
    if let Ok(home) = env::var("HOME") {
        let path = Path::new(&home);
        return go_to_directory(path.to_path_buf());
    }

    None
}

pub fn cd_back(command: &str) -> Option<()> {
    let curr_dir = get_curr_directory();
    let back_count = command.split("./").count();
    let path = Path::new(&curr_dir)
        .ancestors()
        .take(back_count)
        .last()
        .unwrap();

    return go_to_directory(path.to_path_buf());
}

pub fn get_curr_directory() -> PathBuf {
    let curr_dir = env::current_dir().unwrap();
    return Path::new(&curr_dir).to_path_buf();
}

pub fn cd_relative(directory: &str) -> Option<()> {
    let curr_dir = get_curr_directory();
    let next_dir = directory
        .split_whitespace()
        .nth(1)
        .unwrap()
        .replace("./", "");
    let next_dir_path = curr_dir.join(next_dir);

    return go_to_directory(next_dir_path);
}

pub fn go_to_directory(directory: PathBuf) -> Option<()> {
    if directory.is_dir() {
        return Some(assert!(env::set_current_dir(&directory).is_ok()));
    }

    None
}

pub fn no_file_or_directory(directory: &str) {
    println!(
        "cd: {}: No such file or directory",
        directory.split_whitespace().nth(0).unwrap()
    )
}
