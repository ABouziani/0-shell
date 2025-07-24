use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

pub fn cat(args: &[&str]) {
    if args.is_empty() {
        parce();
    }
    for i in args {
        if i == &"-" {
            parce();
        } else {
            match read_file_simple(i) {
                Ok(content) => print!("{}", content),
                Err(e) => eprint!("\x1b[31mcat: {}: {}\x1b[0m", i, e),
            }
        }
    }
    println!()
}

fn parce() {
    loop {
        let mut input = String::new();
        if let Err(e) = std::io::stdin().read_line(&mut input) {
            eprintln!("\x1b[31mError reading input: {}\x1b[0m", e);
            return;
        }
        if input.trim().is_empty() {
            break;
        }
        println!("{}", input.trim());
    }
}
fn read_file_simple(path: &str) -> Result<String, std::io::Error> {
    
    let metadata = fs::metadata(path)?;
    if !metadata.is_file() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            format!("{}: Is not a file", path)
        ));
    }

    let file = File::open(path)?;
    let mut buf_reader = BufReader::new(file);
    let mut buffer = Vec::new();
    buf_reader.read_to_end(&mut buffer)?;

    Ok(String::from_utf8_lossy(&buffer).into_owned())
}