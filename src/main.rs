mod parser;

use parser::*;
use std::io::{self, Write};

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
            Ok(v)=> { 
                clear = true; 
                println!("-> {:?}",v)
            },
            Err(s) => { 
                clear= false;
                input.pop();
                input.push_str("'$'\n'");
                print!("{}",s)
            }

        };
    }
}

