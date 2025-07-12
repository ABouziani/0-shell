use std::fs;
use std::io::{self, Write};

fn mkdir(args: &[String]) {
    if args.is_empty() {
        eprintln!("mkdir: missing operand");
        return;
    }

    for path in args {
        match fs::create_dir(path) {
            Ok(_) => {}
            Err(ref e) if e.kind() == std::io::ErrorKind::AlreadyExists => {
                eprintln!("mkdir: cannot create directory '{}': File exists", path);
            }
            Err(e) => {
                eprintln!("mkdir: cannot create directory '{}': {}", path, e);
            }
        }
    }
}