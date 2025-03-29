#[allow(unused_imports)]
use std::io::{self, Write};

static COMMANDS: &[&str] = &["echo", "exit", "type"];

fn get_type(command: &str) {
    if COMMANDS.contains(&command) {
        println!("{} is a shell builtin", command);
    } else {
        println!("{}: not found", command);
    }
}

fn main() {
    // Uncomment this block to pass the first stage
    let stdin = io::stdin();
    // Wait for user input
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
        let mut input: String = String::new();
        stdin.read_line(&mut input).unwrap();

        let args: Vec<&str> = input.trim().split_whitespace().collect();

        match args.get(0) {
            Some(&"type") => {
                get_type(args[1]);
            }
            Some(&"exit") => {
                if args.len() == 1 || args[1] == "0" {
                    return;
                }
            }
            Some(&"echo") => {
                println!("{}", args[1..].join(" "));
            }
            Some(_cmd) => {
                println!("{}: command not found", input.trim())
            }
            None => continue,
        }
    }
}
