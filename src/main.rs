use std::io::{self, Write};
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use std::process::Command;
use std::process::ExitCode;
use std::{env, path};

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
            "echo" => print!("{}", str::replace(&command, "echo ", "")),
            "type" => execute_type(&command),
            "pwd" => pwd(),
            "cd" => cd(&command),
            _ => execute(&command),
        };
    }
}

pub fn execute(command: &str) {
    let exe = command.split_whitespace().nth(0).unwrap();

    if let Some(_) = get_exe(&exe) {
        let args = command.split_whitespace().filter(|x| !x.contains(exe));

        let input = match Command::new(&exe).args(args).output() {
            Ok(output) => output,
            Err(_) => {
                println!("{}: command not found", command.trim());
                return;
            }
        };

        let output = String::from_utf8_lossy(&input.stdout);
        print!("{}", output);
    }
}

fn cd(command: &str) {
    let next_dir = command.split_whitespace().nth(1).unwrap();

    if next_dir.starts_with("./") {
        cd_relative(command);
        return;
    } else if next_dir.starts_with("../") {
        cd_back(command);
        return;
    }

    let path = path::Path::new(next_dir);
    let new_dir_path = path.join(next_dir);
    if new_dir_path.is_dir() {
        assert!(env::set_current_dir(&new_dir_path).is_ok());
        return;
    }

    println!("cd: {}: No such file or directory", next_dir)
}

fn cd_back(command: &str) {
    let curr_dir = env::current_dir().unwrap();
    let count = command.split("./").count();
    let path = path::Path::new(&curr_dir)
        .ancestors()
        .take(count)
        .last()
        .unwrap();

    if path.is_dir() {
        assert!(env::set_current_dir(&path).is_ok());
        return;
    }

    return;
}

fn cd_relative(command: &str) {
    let curr_dir = env::current_dir().unwrap();
    let path = path::Path::new(&curr_dir);
    let next_dir = command.split_whitespace().nth(1).unwrap().replace("./", "");
    let new_dir_path = path.join(next_dir);

    if new_dir_path.is_dir() {
        assert!(env::set_current_dir(&new_dir_path).is_ok());
        return;
    }

    // for dirs in path {
    //     let replaced_dir = &mut next_dir.replace("./", "");
    //     println!("{} and new {}", , replaced_dir.to_string());
    //     if dirs.to_string_lossy() == replaced_dir.to_string() {
    //         if new_dir_path.is_dir() {
    //             assert!(env::set_current_dir(&new_dir_path).is_ok());
    //             return;
    //         }
    //     }
    // }

    println!(
        "cd: {}: No such file or directory",
        command.split_whitespace().nth(0).unwrap()
    )
}

fn pwd() {
    let curr_dir = env::current_dir().unwrap();
    println!("{}", curr_dir.display())
}

fn get_exe(command: &str) -> Option<String> {
    if let Ok(paths) = env::var("PATH") {
        for dir in env::split_paths(&paths) {
            let path = dir.join(command);
            return Some(path.to_string_lossy().to_string());
        }
    }
    None
}

pub fn execute_type(command: &str) {
    let cmd = command.split_whitespace().nth(1).unwrap_or("");
    if BUILTINS.contains(&cmd) {
        println!("{} is a shell builtin", cmd);
    } else if let Some(path) = is_command_in_path(cmd) {
        println!("{} is {}", cmd, path);
    } else {
        println!("{}: not found", cmd.trim());
    }
}

fn is_command_in_path(command: &str) -> Option<String> {
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
