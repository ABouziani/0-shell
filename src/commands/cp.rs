use std::fs;
use std::path::Path;
use std::error::Error;

fn cp(args: &Vec<&str>) -> Result<(),Box<dyn Error>> {
    if args.len() <= 2 {
        return Err("No files or directories provided.".into());
    }

    let sources = &args[1..args.len() - 1];
    let dest = args.last().unwrap();
    if sources.is_empty() {
        return Err("No files or directories provided.".into());
    }
    for source in sources {
        copy_file(source, dest)?;
    }

    Ok(())
}
fn copy_file(source: &str, dest: &str) -> Result<(), Box<dyn Error>> {
    let source_path = Path::new(source);
    let dest_path = Path::new(dest);

    if !source_path.exists() {
        return Err(format!("cannot stat '{}': No such file or directory", source).into());
    }

    if source_path.is_file() {
        let final_dest = {
            if dest.exists() && dest.is_dir() {
                dest.join(source.file_name().ok_or("Missing filename")?)
            } else if !dest.exists(){
                fs::create_dir_all(dest)?;
                dest.join(source.file_name().ok_or("Missing filename")?)
            } else {
                dest.to_path_buf()
            }
        };
    
        if let Some(parent) = final_dest.parent() {
            if !parent.exists() {
                return Err(format!("Destination folder {:?} does not exist", parent).into());
            }
        }
        fs::copy(source, &final_dest)?;
        return Ok(());
    } 
    return Err(format!("'{}' is a directory (not copied)", source).into());
}


