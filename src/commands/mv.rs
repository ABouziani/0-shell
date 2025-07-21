use std::fs;

use std::fs::File;

use std::path::Path;
pub fn mv(args: &[&str]) {
    if args.len() < 2 {
        println!("Error: argement in valid");
        return;
    }
    let path = args[args.len() - 1];
    for i in &args[0..args.len() - 1] {
        let old_path = Path::new(i);
        let new_path = Path::new(&path);
        if path.contains("/") && !new_path.exists() {
            println!(
                "mv: cannot move {:?} to {:?}: Not a directory",
                old_path, new_path
            );
            continue;
        }
        if !old_path.exists() {
            let meta = match old_path.symlink_metadata() {
                Ok(m) => m,
                Err(_) => {
                    println!("mv: cannot stat {:?}: No such file or directory", old_path);
                    continue;
                }
            };
            if meta.is_symlink() {
                let name_file = old_path.file_name();
                let t = match name_file {
                    Some(name) => name,
                    None => {
                        eprintln!("Error: Invalid source file name.");
                        continue;
                    }
                };
                let v = new_path.join(t);

                let _ = match File::create(&v) {
                    Ok(v) => v,
                    Err(err) => {
                        println!("Error: {}", err);
                        return;
                    }
                };

                _ = fs::copy(v, new_path);

                _ = fs::remove_file(old_path);
            } else {
                println!(
                    "-- mv: cannot stat {:?}: No such file or directory",
                    old_path
                );
                continue;
            }
        }
        if new_path.is_dir() && old_path.is_file() {
            let name_file = old_path.file_name();
            let t = match name_file {
                Some(name) => name,
                None => {
                    eprintln!("Error: Invalid source file name.");
                    continue;
                }
            };
            let v = new_path.join(t);

            let _ = match File::create(&v) {
                Ok(v) => v,
                Err(err) => {
                    println!("Error: {}", err);
                    return;
                }
            };

            _ = fs::copy(v, new_path);

            _ = fs::remove_file(old_path);
        } else {
            _ = fs::rename(old_path, new_path);
        }
    }
}
