use std::fs::File;
use std::io::Read;

use regex::Regex;

use crate::commands::{Command, CommandResult};
use crate::utils::parser::get_formatted_args;

pub struct Head;
impl Command for Head {
    fn name(&self) -> &'static str {
        "head"
    }
    fn run(&self, cmd: &str) -> CommandResult {
        let command = cmd.replace("head", "");
        return head(&command);
    }
}

pub fn head(command: &str) -> CommandResult {
    return heads_or_tails(command, true);
}

pub fn heads_or_tails(command: &str, tails: bool) -> CommandResult {
    let args = get_formatted_args(command);
    let mut contents: Vec<String> = Vec::new();
    let count = get_count(&args);

    let command_wo_args = command.replace("-n ", "");
    let re = Regex::new(r"[0-9] ").unwrap();
    re.replace(&command_wo_args, "");

    match File::open(command) {
        Ok(mut file) => {
            let mut text = String::new();
            file.read_to_string(&mut text).expect("reading file");

            if tails {
                for line in text.lines().rev().take(count) {
                    contents.push(line.to_string());
                }
            } else {
                for line in text.lines().take(count) {
                    contents.push(line.to_string());
                }
            }
        }
        Err(_) => {
            for line in command_wo_args.lines().take(count) {
                contents.push(line.to_string());
            }
        }
    };

    return CommandResult::Output(contents.join("\n"));
}

fn get_count(args: &Vec<String>) -> usize {
    if args.first().unwrap() == "-n" {
        let count = args.iter().nth(1).unwrap();
        return count.parse::<usize>().unwrap_or(10);
    }

    return 10;
}
