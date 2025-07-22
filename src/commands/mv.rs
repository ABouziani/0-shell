use std::fs;
use std::fs::File;
use std::path::Path;
pub fn mv(args: &[&str]) {
    if args.len() < 2 {
        println!("Error: argement in valid");
        return;
    } else if args.len() > 2 {
        let path = Path::new(args[args.len() - 1]);
        if path.is_file() {
            println!("mv: target {:?} is not a directory", path);
            return;
        }
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
        } else if old_path.is_dir() && new_path.is_dir() {
            if let Some(last) = last_dir_in_path(old_path) {
                let new_path = new_path.join(last);

                // Create the new directory (and parents if needed)
                if !new_path.exists() {
                    _=fs::create_dir_all(&new_path);
                    _=fs::rename(old_path, new_path);
                    // println!("Created directory {:?}", new_path);
                } else {
                    println!("Directory {:?} already exists", new_path);
                }
            }
        } else {
            println!("{:?}", old_path.is_dir());
            println!("{:?}", new_path.is_dir());
            if let Err(err) = fs::rename(&old_path, &new_path) {
                println!(
                    "mv: failed to move {:?} to {:?}: {}",
                    old_path, new_path, err
                );
            }
        }
    }
}



fn last_dir_in_path(path: &Path) -> Option<&str> {
    println!("{:?}", path);
    path.file_name().and_then(|os_str| os_str.to_str())
}
