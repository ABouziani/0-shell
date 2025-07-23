use std::error::Error;
use std::fs;
use std::path::Path;

pub fn cp(args: &Vec<&str>) {
    if args.len() < 2 {
        println!("No files or directories provided.");
        return;
    }

    let sources = &args[..args.len() - 1];
    let dest = args.last().unwrap();
    let dest_path = Path::new(dest);
    if ((dest_path.exists() && !dest_path.is_dir()) || !dest_path.exists()) && sources.len() > 1 {
        println!("Destination '{}' not dir.", dest);
        return;
    }
    if sources.is_empty() {
        println!("No files or directories provided.");
        return;
    }
    for source in sources {
        if let Err(e) = copy_file(source, dest) {
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
    if check_links(source, dest).is_err() {
        return Err(format!("cp: {} and {} are the same file", source, dest).into());
    }

    if source_path.is_file() {
        let final_dest = {
            if dest_path.exists() && dest_path.is_dir() {
                dest_path.join(source_path.file_name().ok_or("Missing filename")?)
            } else if dest_path.file_name() == Some(source_path.file_name().unwrap_or_default()) {
                return Err(format!("cp: {} and {} are the same file", dest, source).into());
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


fn check_links(source: &str, dest: &str) -> Result<(), String> {
    let source_link = match fs::read_link(source) {
        Ok(target) => target.to_string_lossy().into_owned(),
        Err(_) => source.to_string(),
    };

    let dest_link = match fs::read_link(dest) {
        Ok(target) => target.to_string_lossy().into_owned(),
        Err(_) => dest.to_string(),
    };

    if dest_link == source_link {
        return Err(format!(
            "cp: {} and {} are the same file",
            source, dest
        ));
    }

    Ok(())
}