mod reader;
mod printer;
mod types;

use std::io::{self, Write};

fn read(input: &str) -> &str {
    // let tokens_vec =
    reader::read_str(input);
    input
    // tokens_vec
}

fn eval(input: &str) -> &str{
    input
}

// fn print(input: &Vec<types::Node>) -> String {
//     let tokens = printer::pr_str(input);
//     tokens
// }

fn rep(input: &str) -> &str {
    let read_result = read(input);
    let eval_result = eval(read_result);
    eval_result
    // print(&eval_result)
}

fn main() {
    let stdin = io::stdin();
    let input = &mut String::new();

    loop {
        print!("{}", "user> ");
        io::stdout().flush().unwrap();
        input.clear();
        let result = stdin.read_line(input);
        match result {
            Err(e)  => {println!("error parsing header: {:?}", e);}
            _ => {}
        }
        println!("{}", rep(input.as_ref()));
    }
}
