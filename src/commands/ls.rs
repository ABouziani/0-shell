use std::fs::{self, DirEntry, Metadata};
use std::os::unix::fs::{MetadataExt, PermissionsExt};
use std::time::SystemTime;
use chrono::{DateTime, Local};
use std::io;

pub fn ls(args: &[&str]) {
    let mut show_all = false;
    let mut long_list = false;
    let mut classify = false;
    let mut targets: Vec<&str> = vec![];

    for arg in args {
        if arg.starts_with('-') {
            for c in arg.chars().skip(1) {
                match c {
                    'a' => show_all = true,
                    'l' => long_list = true,
                    'F' => classify = true,
                    _ => {
                        eprintln!("ls: invalid option -- '{}'", c);
                        return;
                    }
                }
            }
        } else {
            targets.push(arg);
        }
    }

    if targets.is_empty() {
        targets.push(".");
    }

    for (i, target) in targets.iter().enumerate() {
        if targets.len() > 1 {
            if i > 0 {
                println!();
            }
            println!("{}:", target);
        }

        if let Err(e) = list_dir(target, show_all, long_list, classify) {
            eprintln!("ls: cannot access '{}': {}", target, e);
        }
    }
}

fn list_dir(path: &str, show_all: bool, long_list: bool, classify: bool) -> io::Result<()> {
    let mut entries = fs::read_dir(path)?
        .filter_map(Result::ok)
        .collect::<Vec<DirEntry>>();

    entries.sort_by_key(|e| e.file_name());

    for entry in entries {
        let file_name = entry.file_name();
        let file_name_str = file_name.to_string_lossy();

        if !show_all && file_name_str.starts_with('.') {
            continue;
        }

        let metadata = entry.metadata()?;
        let file_type = metadata.file_type();

        if long_list {
            print_long_format(&metadata);
        }

        print!("{}", file_name_str);

        if classify {
            if file_type.is_dir() {
                print!("/");
            } else if file_type.is_symlink() {
                print!("@");
            } else if metadata.permissions().mode() & 0o111 != 0 {
                print!("*");
            }
        }

        println!();
    }

    Ok(())
}

fn print_long_format(metadata: &Metadata) {
    fn mode_string(is_dir: bool, is_symlink: bool, mode: u32) -> String {
        let file_type = if is_dir { "d" } else if is_symlink { "l" } else { "-" };
        let perms = [
            (mode & 0o400 != 0, 'r'),
            (mode & 0o200 != 0, 'w'),
            (mode & 0o100 != 0, 'x'),
            (mode & 0o040 != 0, 'r'),
            (mode & 0o020 != 0, 'w'),
            (mode & 0o010 != 0, 'x'),
            (mode & 0o004 != 0, 'r'),
            (mode & 0o002 != 0, 'w'),
            (mode & 0o001 != 0, 'x'),
        ];
        file_type.to_string() + &perms.iter().map(|(b, c)| if *b { *c } else { '-' }).collect::<String>()
    }

    let mode = metadata.permissions().mode();
    let nlink = metadata.nlink();
    let uid = metadata.uid();
    let gid = metadata.gid();
    let size = metadata.size();
    let mtime = metadata.modified().unwrap_or(SystemTime::UNIX_EPOCH);
    let datetime: DateTime<Local> = mtime.into();
    let time_str = datetime.format("%b %e %H:%M").to_string();

    let is_dir = metadata.is_dir();
    let is_symlink = metadata.is_symlink();
    let mode_str = mode_string(is_dir, is_symlink, mode);

    print!(
        "{} {:>2} {:>5} {:>5} {:>8} {} ",
        mode_str, nlink, uid, gid, size, time_str
    );
}
