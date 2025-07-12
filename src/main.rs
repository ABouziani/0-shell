use std::io::{self, Write};
use std::process;

mod commands; 

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        let bytes = io::stdin().read_line(&mut input);

        //  ctr+ d (EOF)
        match bytes {
            Ok(0) => {
                println!();
                break;
            }
            Ok(_) => {}
            Err(e) => {
                eprintln!("Error reading input: {}", e);
                continue;
            }
        }
        let input = input.trim();
        if input.is_empty() {
            continue;
        }

        let parts: Vec<String> = input.split_whitespace().map(|s| s.to_string()).collect();
        let command = &parts[0];
        let args = &parts[1..];
        //commmands match
        match command.as_str() {
            "mkdir" => commands::mkdir::mkdir(args),
            "exit" => {
                println!("Bye!");
                process::exit(0);
            }
            _ => eprintln!("Command '{}' not found", command),
        }
    }
}
