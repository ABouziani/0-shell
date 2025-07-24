use std::env;
use std::path::PathBuf;
use std::sync::{Mutex, OnceLock};

static PREV_DIR: OnceLock<Mutex<PathBuf>> = OnceLock::new();

pub fn cd(args: &Vec<&str>, path: &mut String) {
    let home_dir = match env::var("HOME") {
        Ok(path) => path,
        Err(_) => {
            eprintln!("\x1b[31mcd: HOME not set\x1b[0m");
            return;
        }
    };
    let current_dir = env::current_dir().unwrap_or(PathBuf::from("/"));
    let prev_mutex = PREV_DIR.get_or_init(|| Mutex::new(current_dir.clone()));

    let target = if args.is_empty() {
        home_dir.clone()
    } else if args[0] == "-" {
        let mut prev = prev_mutex.lock().unwrap();
        println!("{}", prev.display());

        std::mem::replace(&mut *prev, current_dir.clone())
            .to_string_lossy()
            .to_string()
    } else {
        let input = args[0];
        if input.starts_with("~") {
            input.replacen("~", &home_dir, 1)
        } else {
            input.to_string()
        }
    };
    if let Err(e) = env::set_current_dir(&target) {
        eprintln!("\x1b[31mcd: {}: {}\x1b[0m", target, e);
    } else {
        let mut prev = prev_mutex.lock().unwrap();
        *prev = current_dir;
        *path = env::current_dir()
            .unwrap_or(PathBuf::from("/"))
            .display()
            .to_string();
    }
}
