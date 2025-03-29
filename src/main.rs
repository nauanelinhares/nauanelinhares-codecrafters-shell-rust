#[allow(unused_imports)]
use std::io::{self, Write};

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

        if args[0] == "exit" {
            if args.len() == 1 {
            } else if args[1] == "0" {
                return;
            }
        } else if args[0] == "echo" {
            println!("{}", args[1..].join(" "));
        } else {
            println!("{}: command not found", input.trim())
        }
    }
}
