use std::env;

pub fn cd(args: &Vec<&str>) {
    let target = if args.is_empty() { 
        match env::var("HOME") {
            Ok(path) => path,
            Err(_) => {
                eprintln!("cd: HOME not set");
                return;
            }
        }
    } else {
        args[0].to_string().clone()  
    };

    if let Err(e) = env::set_current_dir(&target) {
        eprintln!("cd: {}: {}", target, e);
    }
}
