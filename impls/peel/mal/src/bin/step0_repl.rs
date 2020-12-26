use std::io::{self, Read, Write};

fn READ(input: &str) -> &str {
    input
}

fn EVAL(input: &str) -> &str {
    input
}

fn PRINT(input: &str) -> &str {
    input
}

fn rep(input: &str) -> &str {
    let result = READ(input);
    let result = EVAL(result);
    let result = PRINT(result);
    result
}

fn main() {
    let mut stdin = io::stdin();
    let input = &mut String::new();

    loop {
        print!("{}", "user> ");
        io::stdout().flush().unwrap();
        input.clear();
        stdin.read_line(input);
        print!("{}", rep(input.as_ref()));
        io::stdout().flush().unwrap();
    }
}
