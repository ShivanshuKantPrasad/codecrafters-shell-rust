#[allow(unused_imports)]
use std::io::{self, Write};

fn parse_commands(input: String) {
    let input = input.trim();
    let mut split = input.split(' ');
    match split.next() {
        Some("exit") => {
            let code = split.next().unwrap().parse::<i32>().unwrap();
            std::process::exit(code);
        }
        Some("echo") => {
            println!("{}", split.collect::<Vec<_>>().join(" "));
        }
        Some(x) => {
            println!("{x}: command not found");
        }
        None => {}
    }
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
