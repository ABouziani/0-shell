use std::fs;
use std::path::Path;
use std::error::Error;

pub fn cp(args: &Vec<&str>) {
    if args.len() < 2 {
        println!("No files or directories provided.");
        return;
    }

    let sources = &args[..args.len() - 1];
    let dest = args.last().unwrap();
    let dest_path = Path::new(dest);
    if dest_path.exists() && !dest_path.is_dir() && sources.len() > 1 {
        println!("Destination '{}' not dir.", dest);
        return;
    }
    if sources.is_empty() {
        println!("No files or directories provided.");
        return;
    }
    for source in sources {
        if let Err(e) = copy_file(source, dest){
            println!("{e}");
            return;
        }
    }
}
fn copy_file(source: &str, dest: &str) -> Result<(), Box<dyn Error>> {
    let source_path = Path::new(source);
    let dest_path = Path::new(dest);
    

    if !source_path.exists() {
        return Err(format!("cannot stat '{}': No such file or directory", source).into());
    }

    if source_path.is_file() {
        let final_dest = {
            if dest_path.exists() && dest_path.is_dir() {
                dest_path.join(source_path.file_name().ok_or("Missing filename")?)
            } else {
                dest_path.to_path_buf()
            }
        };
    
        if let Some(parent) = final_dest.parent() {
            if dest.contains("/") && !parent.exists() {
                return Err(format!("Destination folder {:?} does not exist", parent).into());
            }
        }
        fs::copy(source_path, final_dest)?;
        return Ok(());
    } 
    return Err(format!("'{}' is a directory (not copied)", source).into());
}