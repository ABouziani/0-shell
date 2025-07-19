mod commands;

use std::io::{self, Write};
use std::process;

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        let bytes_read = io::stdin().read_line(&mut input).unwrap();

        if bytes_read == 0 {
            println!();
            break;
        }

        let input = input.trim();
        if input.is_empty() {
            continue;
        }

    
        let tokens: Vec<&str> = input.split_whitespace().collect();
        let command = tokens[0];
        let args = &tokens[1..];

        
        match command {
            "ls" => commands::ls::ls(args),
            // "echo" => commands::echo::echo(args),
            // "cd" => commands::cd::cd(args),
            // "pwd" => commands::pwd::pwd(args),
            // "cat" => commands::cat::cat(args),
            // "cp" => commands::cp::cp(args),
            // "rm" => commands::rm::rm(args),
            "mv" => commands::mv::mv(args),
            "mkdir" => commands::mkdir::mkdir(args),
            // "exit" => process::exit(0),
            _ => eprintln!("Command '{}' not found", command),
        }
    }
}
