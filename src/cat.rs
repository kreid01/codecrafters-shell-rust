use std::path::{self, PathBuf};

use crate::{
    executor::{self, execute_with_redirect, BuiltInCommand},
    parser, writer,
};

pub fn cat(command: &str) {
    let command_wo_cat = str::replace(&command, "cat ", "");
    execute_with_redirect(&command_wo_cat, write_cat, default_cat, BuiltInCommand::Cat);
}

pub fn default_cat(command: &str) {
    let args = parser::get_formatted_args(&command);
    if let Some(call) = Some(execute_cat(args, false)) {
        println!("{}", call.unwrap());
    }
}

pub fn write_cat(output_path: PathBuf, args: Vec<String>) {
    for path in args {
        if let Some(result) = Some(execute_cat(vec![path], true)) {
            writer::write(output_path.to_owned(), vec![result.unwrap()]).ok();
        }
    }
}

pub fn execute_cat(args: Vec<String>, redirect: bool) -> Option<String> {
    if let Some(not_found) = check_file_not_found("cat".to_string(), &args[0], redirect) {
        return Some(not_found);
    }

    return executor::execute_command(BuiltInCommand::Cat, args);
}

fn check_path(path: &String) -> bool {
    return !path::Path::new(path).is_file();
}

fn check_file_not_found(command: String, path: &String, redirect: bool) -> Option<String> {
    if check_path(&path) {
        let result = format!("{}: {}: No such file or directory", command, path);
        if redirect {
            return Some(result);
        }

        println!("{}", result);
        return None;
    }

    return None;
}
