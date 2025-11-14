use std::env;
use std::io::{self, Write};
use std::os::unix::fs::PermissionsExt;
use std::process::Command;
use std::process::ExitCode;

mod change_directory;

const BUILTINS: [&str; 6] = ["exit", "echo", "type", "pwd", "cd", "cat"];

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
            "cat" => cat(&command),
            _ => execute(&command),
        };
    }
}

pub fn cat(command: &str) {
    let command_wo_echo = str::replace(&command, "cat ", "");
    let mut return_commnd = command_wo_echo
        .split("'")
        .filter(|x| x.to_owned() != "".to_owned());
    let mut final_command = String::new();

    for x in return_commnd {
        let input = match Command::new("cat").arg(x).output() {
            Ok(output) => output,
            Err(_) => {
                println!("{}: command not found", command.trim());
                return;
            }
        };

        let output = String::from_utf8_lossy(&input.stdout);
        final_command.push_str(" ");
        final_command.push_str(&output);
    }
}

pub fn echo(command: String) {
    let command_wo_echo = str::replace(&command, "echo ", "");
    let mut return_commnd = command_wo_echo
        .split("'")
        .filter(|x| x.to_owned() != "".to_owned());

    let mut final_command = String::new();
    if return_commnd.clone().count() == 1 {
        for x in return_commnd.nth(0).unwrap().split_whitespace() {
            final_command.push_str(" ");
            final_command.push_str(x.trim());
        }
    } else {
        for x in return_commnd {
            final_command.push_str(x);
        }
    }

    println!("{}", &final_command.trim())
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
