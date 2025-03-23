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

        println!("{}: command not found", input.trim())
    }
}
