use std::env;

pub fn cd(args: &[String]) {
    let target = if args.is_empty() { // which mean change dir to home
        match env::var("HOME") {
            Ok(path) => path,
            Err(_) => {
                eprintln!("cd: HOME not set");
                return;
            }
        }
    } else {
        args[0].clone() // take 
    };

    if let Err(e) = env::set_current_dir(&target) {
        eprintln!("cd: {}: {}", target, e);
    }
}
