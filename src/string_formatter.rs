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
                } else {
                    if last_char_is_not_whitespace(&formatted_string) || escaped {
                        formatted_string.push(' ');
                    }
                }
            }
            _ => formatted_string.push(x),
        }

        unformatted_string.push(x);
    }

    formatted_string
}

fn last_char_is_not_whitespace(str: &String) -> bool {
    return str
        .chars()
        .last()
        .map(|ch| !ch.is_whitespace())
        .unwrap_or(true);
}

fn last_char_is_escape(str: &String) -> bool {
    return str.chars().last().map(|ch| ch == '\\').unwrap_or(false);
}

pub fn get_formatted_args(command: &str) -> Vec<String> {
    let mut in_single_quotes = false;
    let mut in_double_quotes = false;

    let mut args: Vec<String> = Vec::new();
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
                    let truly_escaped = escaped && !literal;
                    if truly_escaped {
                        formatted_string.push(x);
                    } else {
                        in_double_quotes = !in_double_quotes;
                    }
                } else {
                    formatted_string.push(x);
                }
            }
            '\\' => {
                if escaped || in_single_quotes {
                    formatted_string.push('\\');
                }
            }
            c if c.is_whitespace() => {
                if in_single_quotes || in_double_quotes {
                    formatted_string.push(c);
                } else if !formatted_string.is_empty() {
                    args.push(formatted_string.clone());
                    formatted_string.clear();
                }
            }
            _ => {
                formatted_string.push(x);
            }
        }

        unformatted_string.push(x);
    }

    if !formatted_string.is_empty() {
        args.push(formatted_string);
    }

    args
}
