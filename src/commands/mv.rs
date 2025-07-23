use std::fs;
use std::path::Path;
use std::io;


pub fn mv(args: &Vec<&str>) {
    if args.len() < 2 {
        eprintln!("mv: missing file operand");
        return;
    }

    let dest_path = Path::new(args[args.len() - 1]);
    let sources = &args[0..args.len() - 1];

    if sources.len() > 1 && !dest_path.is_dir() {
        eprintln!("mv: target '{}' is not a directory", dest_path.display());
        return;
    }

    for source_str in sources {
        let source_path = Path::new(source_str);
        
        if let Err(e) = move_single_item(source_path, dest_path) {
            eprintln!("mv: cannot move '{}' to '{}': {}", 
                     source_path.display(), dest_path.display(), e);
        }
    }
}

fn move_single_item(source: &Path, dest: &Path) -> io::Result<()> {
    if !source.exists() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("No such file or directory: '{}'", source.display())
        ));
    }

    let final_dest = if dest.is_dir() {
        let filename = source.file_name()
            .ok_or_else(|| io::Error::new(
                io::ErrorKind::InvalidInput,
                "Invalid source file name"
            ))?;
        dest.join(filename)
    } else {
        dest.to_path_buf()
    };

    match fs::rename(source, &final_dest) {
        Ok(()) => Ok(()),
        Err(e) => {
            match e.kind() {
                io::ErrorKind::PermissionDenied => Err(e),
                _ => {
                    copy_and_delete(source, &final_dest)
                }
            }
        }
    }
}


fn copy_and_delete(source: &Path, dest: &Path) -> io::Result<()> {
    if source.is_file() {
        fs::copy(source, dest)?;
        fs::remove_file(source)?;
    } else if source.is_dir() {
        copy_dir_recursive(source, dest)?;
        fs::remove_dir_all(source)?;
    } else {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Source is neither a file nor a directory"
        ));
    }
    
    Ok(())
}

fn copy_dir_recursive(src: &Path, dst: &Path) -> io::Result<()> {
    if !dst.exists() {
        fs::create_dir_all(dst)?;
    }

    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());

        if src_path.is_dir() {
            copy_dir_recursive(&src_path, &dst_path)?;
        } else {
            fs::copy(&src_path, &dst_path)?;
        }
    }

    Ok(())
}
