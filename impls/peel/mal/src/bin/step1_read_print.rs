mod reader;
mod printer;
mod types;

use std::io::{self, Write};

fn read(input: &str) -> types::Node {
    reader::read_str(input)
}

fn eval(input: types::Node) -> types::Node {
    input
}

fn print(mut input: types::Node ) -> String {
    let mut ret = String::from("");

    printer::print_preorder(&mut input, &mut ret, false);
    ret
}

fn rep(input: &str) -> String {
    let read_result = read(input);
    let eval_result = eval(read_result);
    print(eval_result)
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
