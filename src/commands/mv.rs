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

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use std::fs::File;
//     use std::io::Write;
//     use tempfile::TempDir;

//     #[test]
//     fn test_mv_file_to_file() {
//         let temp_dir = TempDir::new().unwrap();
//         let source = temp_dir.path().join("source.txt");
//         let dest = temp_dir.path().join("dest.txt");

//         // Create source file
//         let mut file = File::create(&source).unwrap();
//         writeln!(file, "test content").unwrap();

//         // Test move
//         let args = vec![source.to_str().unwrap(), dest.to_str().unwrap()];
//         assert!(mv(&args).is_ok());

//         // Verify
//         assert!(!source.exists());
//         assert!(dest.exists());
//     }

//     #[test]
//     fn test_mv_file_to_directory() {
//         let temp_dir = TempDir::new().unwrap();
//         let source = temp_dir.path().join("source.txt");
//         let dest_dir = temp_dir.path().join("dest_dir");
//         let expected_dest = dest_dir.join("source.txt");

//         // Create source file and destination directory
//         let mut file = File::create(&source).unwrap();
//         writeln!(file, "test content").unwrap();
//         fs::create_dir(&dest_dir).unwrap();

//         // Test move
//         let args = vec![source.to_str().unwrap(), dest_dir.to_str().unwrap()];
//         assert!(mv(&args).is_ok());

//         // Verify
//         assert!(!source.exists());
//         assert!(expected_dest.exists());
//     }

//     #[test]
//     fn test_mv_multiple_files_to_directory() {
//         let temp_dir = TempDir::new().unwrap();
//         let source1 = temp_dir.path().join("source1.txt");
//         let source2 = temp_dir.path().join("source2.txt");
//         let dest_dir = temp_dir.path().join("dest_dir");

//         // Create source files and destination directory
//         File::create(&source1).unwrap();
//         File::create(&source2).unwrap();
//         fs::create_dir(&dest_dir).unwrap();

//         // Test move
//         let args = vec![
//             source1.to_str().unwrap(),
//             source2.to_str().unwrap(),
//             dest_dir.to_str().unwrap()
//         ];
//         assert!(mv(&args).is_ok());

//         // Verify
//         assert!(!source1.exists());
//         assert!(!source2.exists());
//         assert!(dest_dir.join("source1.txt").exists());
//         assert!(dest_dir.join("source2.txt").exists());
//     }

//     #[test]
//     fn test_mv_nonexistent_file() {
//         let temp_dir = TempDir::new().unwrap();
//         let source = temp_dir.path().join("nonexistent.txt");
//         let dest = temp_dir.path().join("dest.txt");

//         let args = vec![source.to_str().unwrap(), dest.to_str().unwrap()];
//         assert!(mv(&args).is_err());
//     }
// }