use std::env;

pub fn exit_command(args: Vec<&str>) -> bool {
    if args.len() == 1 || args[1] == "0" {
        return true;
    }
    return false;
}

pub fn echo_command(args: Vec<&str>) {
    println!("{}", args[1..].join(" "));
}

pub fn pwd_command() {
    let current_dir = env::current_dir().unwrap();
    println!("{}", current_dir.display());
}
