#[allow(unused_imports)]
use std::io::{self, Write};
// use std::os::unix::fs::PermissionsExt;
use std::{path::PathBuf, process::Command};

static COMMANDS: &[&str] = &["type", "echo", "exit", "pwd", "cd"];

fn find_executable_in_dir(executable: &str, dir: PathBuf) -> Option<PathBuf> {
    for dir_item in std::fs::read_dir(dir).unwrap() {
        let dir_item = dir_item.unwrap();
        if dir_item.path().is_dir() {
            let executable_path = find_executable_in_dir(executable, dir_item.path());
            if executable_path.is_some() {
                return executable_path;
            }
        }
        // let metadata = dir_item.path().metadata().unwrap();
        // let permissions = metadata.permissions();
        // let is_executable = metadata.is_file() && permissions.mode() & 0o111 != 0;
        if dir_item.file_name() == executable {
            return Some(dir_item.path());
        }
    }
    None
}
fn find_executable_in_path(executable: &str) -> Option<PathBuf> {
    let path = std::env::var("PATH").unwrap();
    let paths = path
        .split(':')
        .map(std::path::Path::new)
        .collect::<Vec<_>>();

    for path in paths {
        if path.is_dir() {
            let executable_path = find_executable_in_dir(executable, PathBuf::from(path));
            if executable_path.is_some() {
                return executable_path;
            }
        }
    }
    None
}

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
        Some("pwd") => {
            println!("{}", std::env::current_dir().unwrap().display());
        }
        Some("type") => {
            if let Some(command) = split.next() {
                if COMMANDS.contains(&command) {
                    println!("{command} is a shell builtin");
                } else {
                    match find_executable_in_path(command) {
                        Some(path) => println!("{command} is {}", path.display()),
                        None => println!("{command}: not found"),
                    }
                }
            }
        }
        Some(x) => match find_executable_in_path(x) {
            Some(path) => {
                Command::new(path.as_os_str())
                    .args(split)
                    .status()
                    .expect("failed to execute process");
            }
            None => println!("{x}: command not found"),
        },
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
