pub fn print_lines(lines: Vec<String>) {
    for line in lines {
        println!("\r{}", line);
    }
}
