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

pub fn cd_command(path: &str) {
    let current_dir = env::current_dir().unwrap();

    if path == "~" {
        let home = env::var("HOME").unwrap();
        let _ = env::set_current_dir(home);
    } else {
        let new_path = current_dir.join(path);
        let result = env::set_current_dir(new_path);

        if result.err().is_some() {
            println!("cd: {}: No such file or directory", path)
        }
    }
}
