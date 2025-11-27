use crate::{cat::CommandResult, exe::get_exe_path, Command, BUILTINS};

pub struct Type;
impl Command for Type {
    fn name(&self) -> &'static str {
        "type"
    }
    fn run(&self, cmd: &str) -> CommandResult {
        return execute_type(&cmd);
    }
}

pub fn execute_type(command: &str) -> CommandResult {
    let cmd = command.split_whitespace().nth(1).unwrap_or("");
    let output: String;

    if BUILTINS.contains(&cmd) {
        output = format!("{} is a shell builtin", cmd);
    } else if let Some(path) = get_exe_path(cmd) {
        output = format!("{} is {}", cmd, path);
    } else {
        output = format!("{}: not found", cmd.trim());
    }

    println!("{}", output);
    return CommandResult::Success;
}
