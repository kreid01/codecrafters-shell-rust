pub fn format_string_command(command: &str) -> String {
    let mut in_single_quotes = false;
    let mut in_double_quotes = false;
    let mut formatted_string = String::new();
    let mut unformatted_string = String::new();

    for x in command.chars() {
        let escaped = last_char_is_escape(&unformatted_string);
        let literal = last_char_is_escape(&formatted_string);

        match x {
            '\'' => {
                if !in_double_quotes && !escaped {
                    in_single_quotes = !in_single_quotes;
                } else {
                    formatted_string.push(x);
                }
            }
            '"' => {
                if !in_single_quotes {
                    if escaped && !literal {
                        formatted_string.push(x);
                    } else {
                        in_double_quotes = !in_double_quotes;
                    }
                } else {
                    formatted_string.push(x);
                }
            }
            '\\' => {
                if in_single_quotes || escaped {
                    formatted_string.push('\\');
                }
            }
            c if c.is_whitespace() => {
                if in_single_quotes || in_double_quotes {
                    formatted_string.push(c);
                } else if last_char_is_not_whitespace(&formatted_string) || escaped {
                    formatted_string.push(' ');
                }
            }
            _ => formatted_string.push(x),
        }

        unformatted_string.push(x);
    }

    formatted_string
}

fn last_char_is_not_whitespace(str: &str) -> bool {
    str.chars()
        .last()
        .map(|ch| !ch.is_whitespace())
        .unwrap_or(true)
}

fn last_char_is_escape(str: &str) -> bool {
    str.chars().last().map(|ch| ch == '\\').unwrap_or(false)
}

pub fn get_formatted_args(command: &str) -> Vec<String> {
    let mut args = Vec::new();
    let mut current = String::new();
    let mut in_single = false;
    let mut in_double = false;
    let mut chars = command.chars().peekable();

    while let Some(c) = chars.next() {
        match c {
            '\'' if !in_double => {
                in_single = !in_single;
            }
            '"' if !in_single => {
                in_double = !in_double;
            }
            '\\' => {
                if in_double {
                    if let Some(&next) = chars.peek() {
                        match next {
                            '"' | '\\' => {
                                current.push(next);
                                chars.next();
                            }
                            _ => {
                                current.push('\\');
                            }
                        }
                    } else {
                        current.push('\\');
                    }
                } else if let Some(next) = chars.next() {
                    current.push(next);
                }
            }
            c if c.is_whitespace() && !in_single && !in_double => {
                if !current.is_empty() {
                    args.push(current.clone());
                    current.clear();
                }
            }
            _ => current.push(c),
        }
    }

    if !current.is_empty() {
        args.push(current);
    }

    args
}

pub fn parse_execute_command(command: &str) -> (String, Vec<&str>) {
    if command.starts_with('\'') || command.starts_with("\"") {
        let command = command.trim();
        let commands_split: Vec<&str> = command.split_whitespace().collect();
        let file = commands_split.iter().next_back().unwrap();

        let exe = format_string_command(command)
            .replace(file, "")
            .replace("'", "\\'");

        let args: Vec<&str> = vec![file];

        (exe, args)
    } else {
        let exe = command.split_whitespace().next().unwrap();
        let args: Vec<&str> = command
            .split_whitespace()
            .filter(|x| !x.contains(exe))
            .collect();
        (exe.to_string(), args)
    }
}
