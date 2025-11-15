pub fn format_string_command(command: &str) -> String {
    let mut in_single_quotes = false;
    let mut in_double_quotes = false;
    let mut formatted_string = String::new();
    let mut unformatted_string = String::new();

    for x in command.chars() {
        match x {
            '\'' => {
                if !in_double_quotes
                    && (unformatted_string.is_empty() || !last_char_is_escape(&unformatted_string))
                {
                    in_single_quotes = !in_single_quotes;
                } else {
                    formatted_string.push(x);
                }
            }
            '"' => {
                if !in_single_quotes
                    && (unformatted_string.is_empty() || !last_char_is_escape(&unformatted_string))
                {
                    in_double_quotes = !in_double_quotes;
                } else {
                    formatted_string.push(x);
                }
            }
            '\\' => {}
            c if c.is_whitespace() => {
                if in_single_quotes || in_double_quotes {
                    formatted_string.push(c);
                } else {
                    if last_char_is_not_whitespace(&formatted_string)
                        || last_char_is_escape(&unformatted_string)
                    {
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
    return str.chars().last().map(|ch| ch == '\\').unwrap_or(true);
}

pub fn get_formatted_args(command: &str) -> Vec<String> {
    let mut in_single_quotes = false;
    let mut in_double_quotes = false;

    let mut args: Vec<String> = Vec::new();
    let mut formatted_string = String::new();
    let mut unformatted_string = String::new();

    for x in command.chars() {
        match x {
            '\'' => {
                if !in_double_quotes || !last_char_is_escape(&unformatted_string) {
                    in_single_quotes = !in_single_quotes;

                    args.push(formatted_string);

                    formatted_string = String::new();
                    unformatted_string = String::new();
                    continue;
                } else {
                    formatted_string.push(x);
                }
            }
            '"' => {
                if !in_single_quotes || !last_char_is_escape(&unformatted_string) {
                    in_double_quotes = !in_double_quotes;

                    args.push(formatted_string);

                    formatted_string = String::new();
                    unformatted_string = String::new();
                    continue;
                } else {
                    formatted_string.push(x);
                }
            }
            '\\' => {
                if last_char_is_escape(&unformatted_string) {
                    formatted_string.push('\\');
                }
            }
            _ => {
                formatted_string.push(x);
            }
        }
    }

    return args;
}
