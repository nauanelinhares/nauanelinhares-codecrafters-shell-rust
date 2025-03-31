use std::env;
#[allow(unused_imports)]
use std::io::{self, Write};
use std::path::Path;

static BUILT_IN_COMMANDS: &[&str] = &["echo", "exit", "type", "pwd"];

fn find_in_path(command: &str) -> Option<String> {
    env::var("PATH").ok().and_then(|path_var| {
        path_var.split(':').find_map(|dir| {
            let full_path = format!("{}/{}", dir, command);
            Path::new(&full_path).exists().then_some(full_path)
        })
    })
}

fn get_type(command: &str) {
    if BUILT_IN_COMMANDS.contains(&command) {
        println!("{} is a shell builtin", command);
    } else if let Some(path) = find_in_path(command) {
        println!("{} is {}", command, path)
    } else {
        println!("{}: not found", command);
    }
}

fn execute_file(cmd: &str, args: Vec<&str>) {
    let mut executable = std::process::Command::new(&cmd);
    if args.len() > 1 {
        let output = executable
            .args(&args[1..])
            .output()
            .expect("Failed to execute command");
        io::stdout().write_all(&output.stdout).unwrap();
        io::stderr().write_all(&output.stderr).unwrap();
    } else {
        let output = executable.output().expect("Failed to execute command");
        io::stdout().write_all(&output.stdout).unwrap();
        io::stderr().write_all(&output.stderr).unwrap();
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
            Some(&"pwd") => {
                let current_dir = env::current_dir().unwrap();
                println!("{}", current_dir.display());
            }
            Some(cmd) => {
                if let Some(_path) = find_in_path(cmd) {
                    execute_file(cmd, args);
                } else {
                    println!("{}: command not found", input.trim())
                }
            }
            None => continue,
        }
    }
}
