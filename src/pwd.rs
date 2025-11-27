use std::env;

use crate::{cat::CommandResult, Command};

pub struct Pwd;
impl Command for Pwd {
    fn name(&self) -> &'static str {
        "pwd"
    }
    fn run(&self, _cmd: &str) -> CommandResult {
        pwd()
    }
}

pub fn pwd() -> CommandResult {
    let curr_dir = env::current_dir().unwrap();
    let output = format!("{}", curr_dir.display());

    println!("{}", output);
    return CommandResult::Success;
}
