use std::fs;
use std::path::Path;
use std::error::Error;

pub fn rm(args: &Vec<&str>){
    if args.len() < 1 {
        eprintln!("\x1b[31mNo files or directories provided.\x1b[0m");
        return;
    }

    let is_recursive = args.contains(&"-r");
    let paths: Vec<_> = args.iter().filter(|&&arg| arg != "-r").collect();
    if paths.iter().any(|&arg| *arg == "." || *arg == "..") {
        eprintln!("\x1b[31mrefusing to remove '.' or '..' directory: skipping '.'\x1b[0m");
        return;
    }
    if paths.is_empty() {
        eprintln!("\x1b[31mNo files or directories provided.\x1b[0m");
        return;
    }
    for path in paths {
        if let Err(e) = remove_file_or_dir(path, is_recursive){
            eprintln!("\x1b[31m{e}\x1b[0m");
            return;
        }
    }
}


fn remove_file_or_dir(path: &str, is_recursive: bool) -> Result<(), Box<dyn Error>> {
    let path_obj = Path::new(path);

    if !path_obj.exists() && !path_obj.is_symlink() {
        return Err(format!("cannot remove '{}': No such file or directory", path).into());
    }

    if path_obj.is_file() || path_obj.is_symlink() {
        fs::remove_file(path_obj)?;
    } else if path_obj.is_dir() {
        if !is_recursive {
            return Err(format!("cannot remove '{}': Is a directory", path).into());
        }
        fs::remove_dir_all(path_obj)?;
    }
    Ok(())
}


