use std::env;

pub fn echo(args: &Vec<&str>) {
    if args.is_empty() {
        println!();
    } else {
        let home = env::var("HOME").unwrap_or_default();
        let expanded_args: Vec<String> = args
            .iter()
            .map(|arg| {
                if arg == &"~" {
                    home.clone()
                } else if arg.starts_with("~/") {
                    format!("{}/{}", home, &arg[2..])
                } else {
                    arg.to_string()
                }
            })
            .collect();

        println!("{}", expanded_args.join(" "));
    }
}
