use crate::commands::grep::Grep;
use crate::commands::{
    cat::Cat, cd::Cd, echo::Echo, execute_type::Type, head::Head, ls::Ls, pwd::Pwd, tail::Tail,
    wc::Wc,
};

pub mod cat;
pub mod cd;
pub mod echo;
pub mod exe;
pub mod execute_type;
pub mod grep;
pub mod head;
pub mod history;
pub mod ls;
pub mod pwd;
pub mod tail;
pub mod wc;

pub enum CommandResult {
    Output(String),
    Success,
    Failed,
}

pub trait Command {
    fn name(&self) -> &'static str;
    fn run(&self, cmd: &str) -> CommandResult;
}

pub fn get_commands() -> Vec<Box<dyn Command>> {
    let commands: Vec<Box<dyn Command>> = vec![
        Box::new(Echo),
        Box::new(Pwd),
        Box::new(Cd),
        Box::new(Cat),
        Box::new(Type),
        Box::new(Ls),
        Box::new(Wc),
        Box::new(Tail),
        Box::new(Head),
        Box::new(Grep),
    ];

    commands
}
