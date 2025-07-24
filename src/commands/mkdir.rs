use std::fs;

pub fn mkdir(args: &Vec<&str>) {
    if args.is_empty() {
        eprintln!("\x1b[31mmkdir: missing operand\x1b[0m");
        return;
    }

    for path in args {
        match fs::create_dir(path) {
            Ok(_) => {}
            Err(e) => {
                eprintln!("\x1b[31mmkdir: cannot create directory '{}': {}\x1b[0m", path, e);
            }
        }
    }
}