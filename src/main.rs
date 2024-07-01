#[allow(unused_imports)]
use std::io::{self, Write};

fn parse_commands(input: String) {
    if input.is_empty() {
        return;
    }
    let input = input.trim();
    if input.starts_with("exit") {
        match input.split_once(' ') {
            Some((_, code)) => {
                let code = code.parse::<i32>().unwrap();
                std::process::exit(code);
            }
            None => println!("Invalide command {input}"),
        }
        std::process::exit(0);
    }
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
