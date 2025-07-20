use std::fs;
use std::fs::*;
use std::path::Path;

pub fn cp(args: &[&str]) {
    let new_path = args[args.len() - 1];
    for i in args {
        let old_path = Path::new(i);
        let new_path = Path::new(&new_path);
        if old_path.exists() || new_path.exists() {
            println!("err");
            continue;
        }




        if new_path.is_dir() && old_path.is_file() {
            let v = new_path.join(old_path);

            let _ = match File::create(&v) {
                Ok(v) => v,
                Err(err) => {
                    println!("Error: {}", err);
                    return;
                }
            };

            _ = fs::copy(old_path, new_path);
        }
    }
}