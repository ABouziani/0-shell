pub fn pwd(args: &[&str]) {
    if args.len() > 1 {
        eprintln!("pwd: too many arguments");
        return;
    }

    match std::env::current_dir() {
        Ok(path) => println!("{}", path.display()),
        Err(e) => eprintln!("pwd: {}", e),
    }
}
