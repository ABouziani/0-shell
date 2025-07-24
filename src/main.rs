mod parser;
mod commands;
use parser::*;
use std::io::{self, Write};
use std::{process};

use crate::commands::pwd;

fn main() {
    
    let mut input = String::new();
    let mut clear = true;
    let mut path_pwd = pwd::pwd();
    loop {
        if clear { print!("\x1b[1;34m{}\x1b[0m $ ", path_pwd.split('/').last().filter(|s| !s.is_empty()).unwrap_or("/")); input.clear()};
        if let Err(_) = io::stdout().flush() {
            break;
        }     
        
        let n = io::stdin().read_line(&mut input).ok();

        if n == Some(0) {
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
                    "pwd" => commands::pwd::get_pwd(args ,&mut path_pwd ),
                    "echo" => commands::echo::echo(args),
                    "cd" => commands::cd::cd(args, &mut path_pwd),
                    "cat" => commands::cat::cat(args),
                    "cp" => commands::cp::cp(args),
                    "rm" => commands::rm::rm(args),
                    "mv" => commands::mv::mv(args),
                    "mkdir" => commands::mkdir::mkdir(args),
                    "exit" => process::exit(0),
                    _ => eprintln!("\x1b[31mCommand '{}' not found\x1b[0m", command),
                }
            },
            Err(s) => { 
                clear= false;
                print!("{}",s)
            }

        };
    }
}

