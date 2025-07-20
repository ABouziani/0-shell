mod parser;
mod commands;
use parser::*;
use std::io::{self, Write};
use std::process;

fn main() {
    let mut input = String::new();
    let mut clear = true;
    loop {
        
        if clear { print!("$ "); input.clear()};
        if let Err(_) = io::stdout().flush() {
            break;
        }     
        
        let n = io::stdin().read_line(&mut input).unwrap();

        if n == 0 {
            println!();
            break;
        }

        let line = input.trim();    

        if line.is_empty() {
            continue;
        }
        match ShellParser::new(line.to_string()).parse(){
            Ok(tokens)=> { 
                clear = true; 
                let command = tokens[0].as_str();
                let args = &tokens[1..].iter().map(|el| el.as_str()).collect::<Vec<_>>();

                
                match command {
                    "ls" => commands::ls::ls(args),
                    "pwd" => commands::pwd::pwd(args),
                    // "echo" => commands::echo::echo(args),
                    // "cd" => commands::cd::cd(args),
                    // "cat" => commands::cat::cat(args),
                    // "cp" => commands::cp::cp(args),
                    // "rm" => commands::rm::rm(args),
                    // "mv" => commands::mv::mv(args),
                    // "mkdir" => commands::mkdir::mkdir(args),
                    "exit" => process::exit(0),
                    _ => eprintln!("Command '{}' not found", command),
                }
            },
            Err(s) => { 
                clear= false;
                // input.pop();
                // input.push_str("'$'\n'");
                print!("{}",s)
            }

        };
    }
}

