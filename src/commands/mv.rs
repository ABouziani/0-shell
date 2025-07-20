use std::fs;

use std::fs::File;

use std::path::Path;
pub fn mv(args: &[&str]) {
    match std::env::current_dir() {
        Ok(path) => println!("{}", path.display()),
        Err(e) => eprintln!("pwd: {}", e),
    }
    if args.len() < 2 {
        println!("Error: argement in valid");
        return;
    }
    let new_path = args[args.len() - 1];
    for i in args {
        let old_path = Path::new(i);
        println!("{:#?}",old_path);
        let new_path = Path::new(&new_path);
        if !old_path.exists() || !new_path.exists() {
            println!("path in valid",); 
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

            _ = fs::remove_file(old_path);
        } else {
           _= fs::rename(old_path, new_path);
        }
    }
}
