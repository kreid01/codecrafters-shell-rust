use std::path::{self, Path, PathBuf};

use crate::{
    executor::{self, BuiltInCommand},
    parser, redirect, writer,
};

pub fn cat(command: &str) {
    let command_wo_cat = str::replace(&command, "cat ", "");

    if command.contains("2>") {
        redirect::redirect_stderr(&command_wo_cat, write_cat);
    }

    if command.contains("1>") {
        redirect::redirect_stdout(&command_wo_cat, write_cat);
    }

    let args = parser::get_formatted_args(&command);
    if let Some(call) = Some(execute_cat(command.to_string(), args, false)) {
        println!("{}", call.unwrap());
    }
}

pub fn write_cat(command: &str, output_path: PathBuf, args: Vec<String>) {
    for path in args {
        if let Some(call) = Some(execute_cat(command.to_string(), vec![path], true)) {
            writer::write(output_path.to_owned(), vec![call.unwrap()]).ok();
        }
    }
}

pub fn execute_cat(path: String, args: Vec<String>, redirect: bool) -> Option<String> {
    if let Some(not_found) = check_file_not_found("cat".to_string(), &path, redirect) {
        return Some(not_found);
    }

    return executor::execute_command(BuiltInCommand::Cat, &path, args);
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
