use std::env;

pub fn pwd(_args: &[&str]) {
    match env::current_dir() {
        Ok(path) => {
            if let Some(path_str) = path.to_str() {
                println!("{}", path_str);
            } else {
                eprintln!("pwd: failed to convert path to UTF-8");
            }
        }
        Err(e) => {
            eprintln!("pwd: {}", e);
        }
    }
}
