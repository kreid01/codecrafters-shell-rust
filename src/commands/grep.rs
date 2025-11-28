use crate::commands::{Command, CommandResult};
use crate::utils::parser::get_formatted_args;

pub struct Grep;
impl Command for Grep {
    fn name(&self) -> &'static str {
        "grep"
    }
    fn run(&self, cmd: &str) -> CommandResult {
        let command = cmd.replace("grep ", "");
        grep(&command)
    }
}

pub fn grep(command: &str) -> CommandResult {
    let formatted_args = get_formatted_args(command);
    let search = formatted_args.first().unwrap();
    let mut result: Vec<String> = Vec::new();
    let args = formatted_args.iter().skip(1);

    for arg in args {
        if arg.contains(search) || arg == search {
            result.push(arg.to_owned());
        }
    }

    CommandResult::Output(result.join("\n"))
}
