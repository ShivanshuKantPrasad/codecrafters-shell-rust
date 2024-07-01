#[allow(unused_imports)]
use std::io::{self, Write};

fn parse_commands(input: String) {
    if input.is_empty() {
        return;
    }
    let input = input.trim();
    println!("{input}: command not found");
}

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();
        parse_commands(input);
    }
}
