use std::fs;
use std::path::Path;
use std::error::Error;

pub fn rm(args: &Vec<&str>){
    if args.len() < 1 {
        println!("No files or directories provided.");
        return;
    }

    let is_recursive = args.contains(&"-r");
    let paths: Vec<_> = args.iter().skip(1).filter(|&&arg| arg != "-r").collect();
    if paths.is_empty() {
        println!("No files or directories provided.");
        return;
    }
    for path in paths {
        if let Err(e) = remove_file_or_dir(path, is_recursive){
            println!("{e}");
            return;
        }
    }
}


fn remove_file_or_dir(path: &str, is_recursive: bool) -> Result<(), Box<dyn Error>> {
    let path_obj = Path::new(path);

    if !path_obj.exists() {
        return Err(format!("cannot remove '{}': No such file or directory", path).into());
    }

    if path_obj.is_file() {
        fs::remove_file(path_obj)?;
    } else if path_obj.is_dir() {
        if !is_recursive {
            return Err(format!("cannot remove '{}': Is a directory", path).into());
        }
        remove_directory_recursive(path_obj)?;
    }
    Ok(())
}

fn remove_directory_recursive(dir: &Path) -> Result<(), Box<dyn Error>> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        
        if path.is_file() {
            fs::remove_file(&path)?;
        } else if path.is_dir() {
            remove_directory_recursive(&path)?;
        }
    }
    
    fs::remove_dir(dir)?;
    Ok(())
}

