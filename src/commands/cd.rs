use std::env;
use std::path::PathBuf;
use std::sync::Mutex;

static mut PREV_DIR: Option<Mutex<PathBuf>> = None;

pub fn cd(args: &Vec<&str>, path: &mut String) {
    let home_dir = match env::var("HOME") {
        Ok(path) => path,
        Err(_) => {
            eprintln!("cd: HOME not set");
            return;
        }
    };

    let current_dir = match env::current_dir() {
        Ok(dir) => dir,
        Err(_) => PathBuf::from("/"),
    };

    unsafe {
        if PREV_DIR.is_none() {
            PREV_DIR = Some(Mutex::new(current_dir.clone()));
        }
    }

    let target = if args.is_empty() {
        home_dir.clone()
    } else if args[0] == "-" {
        unsafe {
            if let Some(ref prev_mutex) = PREV_DIR {
                let mut prev = prev_mutex.lock().unwrap();
                println!("{}", prev.display());
                std::mem::replace(&mut *prev, current_dir.clone()).to_string_lossy().to_string()
            } else {
                eprintln!("cd: OLDPWD not set");
                return;
            }
        }
    } else {
        let input = args[0];
        if input.starts_with("~") {
            input.replacen("~", &home_dir, 1)
        } else {
            input.to_string()
        }
    };

    if let Err(e) = env::set_current_dir(&target) {
        eprintln!("cd: {}: {}", target, e);
    } else {
        unsafe {
            if let Some(ref prev_mutex) = PREV_DIR {
                let mut prev = prev_mutex.lock().unwrap();
                *prev = current_dir;
            }
        }
        *path = match env::current_dir() {
            Ok(p) => p.display().to_string(),
            Err(_) => "/".to_string(),
        };
    }
}
