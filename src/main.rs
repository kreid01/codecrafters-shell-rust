use std::env::{self};
use std::fs;
use std::io::{self, Write};
use std::os::unix::fs::PermissionsExt;
use std::path;
use std::process::ExitCode;

mod cat;
mod change_directory;
mod executor;
mod parser;
mod writer;

const BUILTINS: [&str; 5] = ["exit", "echo", "type", "pwd", "cd"];

fn main() -> ExitCode {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut command = String::new();
        io::stdin().read_line(&mut command).unwrap();

        if command.split_whitespace().nth(0).unwrap() == "exit" {
            return ExitCode::from(0);
        }

        match command.split_whitespace().nth(0).unwrap() {
            "echo" => echo(command),
            "type" => execute_type(&command),
            "pwd" => pwd(),
            "cd" => change_directory::cd(&command),
            "cat" => cat::cat(&command),
            "ls" => ls(command),
            _ => executor::execute(&command),
        };
    }
}

pub fn ls(command: String) {
    let command_split: Vec<&str> = command.split_whitespace().collect();
    if command_split.len() > 3 {
        if command_split[3] == ">" || command_split[3] == "1>" {
            let input_path = command_split[2];
            let output_path = path::Path::new(command_split[4]);
            let entries = fs::read_dir(input_path).expect("Cannot read directory");

            let mut lines = Vec::new();
            for entry in entries {
                let entry = entry.unwrap();
                let file_name = entry.file_name().to_string_lossy().to_string();
                lines.push(file_name);
            }

            lines.sort();
            let _ = writer::write(output_path.to_path_buf(), lines);
        }
    } else {
        let current_dir = change_directory::get_curr_directory();
        for x in current_dir.iter() {
            println!("{}", x.display());
        }
    }
}

pub fn echo(command: String) {
    let command_wo_echo = str::replace(&command, "echo ", "");
    let command_split: Vec<&str> = command_wo_echo.split("1>").collect();
    let formatted_command = parser::format_string_command(&command_wo_echo);

    if command_split.len() > 1 {
        let mut contents = Vec::new();
        let input_path = command_split[0];
        let formatted_command = parser::format_string_command(&input_path).trim().to_owned();
        let output_path = path::Path::new(command_split[1].trim());
        contents.push(formatted_command);
        let _ = writer::write(output_path.to_path_buf(), contents);
    } else {
        println!("{}", &formatted_command.trim())
    }
}

fn pwd() {
    let curr_dir = env::current_dir().unwrap();
    println!("{}", curr_dir.display())
}

pub fn execute_type(command: &str) {
    let cmd = command.split_whitespace().nth(1).unwrap_or("");
    if BUILTINS.contains(&cmd) {
        println!("{} is a shell builtin", cmd);
    } else if let Some(path) = get_exe_path(cmd) {
        println!("{} is {}", cmd, path);
    } else {
        println!("{}: not found", cmd.trim());
    }
}

fn get_exe_path(command: &str) -> Option<String> {
    if let Ok(paths) = env::var("PATH") {
        for dir in env::split_paths(&paths) {
            let path = dir.join(command);
            if path.is_file() {
                if let Ok(metadata) = path.metadata() {
                    let permissions = metadata.permissions();
                    if permissions.mode() & 0o111 != 0 {
                        return Some(path.to_string_lossy().to_string());
                    }
                }
            }
        }
    }

    None
}
