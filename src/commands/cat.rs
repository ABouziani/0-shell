use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

pub fn cat(args: &[&str]) {
    if args.len() < 1 {
        println!("Error: argement in valid");
        return;
    }
    for i in args {
        let metadata = fs::metadata(i);

        match metadata {
            Ok(metadata) => {
                if !metadata.is_file() {
                    println!("{}:Is not fole", i);
                    return;
                }
            }
            Err(e) => {
                println!("{}", e);
                return;
            }
        }

        let file = File::open(i).unwrap();
        let mut buf_reader = BufReader::new(file);
        let mut contents = String::new();
        buf_reader.read_to_string(&mut contents).unwrap();
        println!("{}", contents)
    }
}
