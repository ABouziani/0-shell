use std::fs;

pub fn mkdir(args: &[&str]) {
    if args.is_empty() {
        eprintln!("mkdir: missing operand");
        return;
    }

    for path in args {
        match fs::create_dir(path) {
            Ok(_) => {}
            Err(e) => {
                eprintln!("mkdir: cannot create directory '{}': {}", path, e);
            }
        }
    }
}