use std::env;

pub fn pwd() -> String {
    match env::current_dir() {
        Ok(path) => {
            if let Some(path_str) = path.to_str() {
                return path_str.to_string();
            } else {
                return "".to_string();
            }
        }
        Err(_) => {
            return "".to_string();
        }
    }
}

pub fn get_pwd(args: &[&str], path: &str) {
    if args.len() != 0 {
        println!("pwd: too many arguments");
        return;
    }
    if path.is_empty() {
        return;
    }
    println!("{}", path)
}
