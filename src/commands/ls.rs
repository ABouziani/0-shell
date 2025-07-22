use std::fs::{self, DirEntry, Metadata};
use std::os::unix::fs::{MetadataExt, PermissionsExt, FileTypeExt};
use std::time::SystemTime;
use chrono::{DateTime, Local, Duration};
use std::io;

use users::{get_user_by_uid, get_group_by_gid};

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

    let mut all_entries: Vec<(String, Metadata)> = Vec::new();

    if show_all {
        for special in &[".", ".."] {
            if let Ok(md) = fs::symlink_metadata(format!("{}/{}", path, special)) {
                all_entries.push((special.to_string(), md));
            }
        }
    }

    for entry in entries {
        let file_name = entry.file_name();
        let file_name_str = file_name.to_string_lossy();

        if !show_all && file_name_str.starts_with('.') {
            continue;
        }

        if let Ok(md) = fs::symlink_metadata(entry.path()) {
            all_entries.push((file_name_str.into_owned(), md));
        }
    }

    // Gather formatting info
    let mut user_width = 0;
    let mut group_width = 0;
    let mut nlink_width = 0;
    let mut size_width = 0;
    let mut is_device = Vec::new();

    let mut enriched_entries = Vec::new();

    for (name, md) in &all_entries {
        let user = get_user_by_uid(md.uid())
            .map(|u| u.name().to_string_lossy().to_string())
            .unwrap_or_else(|| md.uid().to_string());
        let group = get_group_by_gid(md.gid())
            .map(|g| g.name().to_string_lossy().to_string())
            .unwrap_or_else(|| md.gid().to_string());

        let nlink = md.nlink().to_string().len();

        user_width = user_width.max(user.len());
        group_width = group_width.max(group.len());
        nlink_width = nlink_width.max(nlink);

        let ft = md.file_type();
        let size_or_dev = if ft.is_char_device() || ft.is_block_device() {
            is_device.push(true);
            // 3 for major + comma + 4 for minor
            4 + 5
        } else {
            is_device.push(false);
            md.size().to_string().len()
        };

        size_width = size_width.max(size_or_dev);

        enriched_entries.push((name.clone(), md.clone(), user, group));
    }

    if long_list {
        let total_blocks: u64 = enriched_entries.iter()
            .map(|(_, md, _, _)| md.blocks())
            .sum();
        println!("total {}", total_blocks);
    }

    for ((file_name, metadata, user, group), dev) in enriched_entries.into_iter().zip(is_device) {
        if long_list {
            print_long_format(&metadata, &user, &group, user_width, group_width, nlink_width, size_width, dev);
        }

        print!("{}", file_name);

        if classify {
            let ft = metadata.file_type();
            if ft.is_dir() {
                print!("/");
            } else if ft.is_symlink() {
                print!("@");
            } else if metadata.permissions().mode() & 0o111 != 0 {
                print!("*");
            }
        }

        println!();
    }

    Ok(())
}

fn print_long_format(
    metadata: &Metadata,
    user: &str,
    group: &str,
    user_width: usize,
    group_width: usize,
    nlink_width: usize,
    size_width: usize,
    is_device: bool
) {
    fn mode_string(metadata: &Metadata) -> String {
        let mode = metadata.permissions().mode();
        let ft = metadata.file_type();

        let file_type_char = if ft.is_dir() {
            'd'
        } else if ft.is_symlink() {
            'l'
        } else if ft.is_char_device() {
            'c'
        } else if ft.is_block_device() {
            'b'
        } else if ft.is_socket() {
            's'
        } else if ft.is_fifo() {
            'p'
        } else {
            '-'
        };

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

        let perm_str: String = perms.iter().map(|(b, c)| if *b { *c } else { '-' }).collect();

        format!("{}{}", file_type_char, perm_str)
    }

    fn major(dev: u64) -> u64 {
        (dev >> 8) & 0xfff
    }

    fn minor(dev: u64) -> u64 {
        (dev & 0xff) | ((dev >> 12) & 0xfff00)
    }

    let mode_str = mode_string(metadata);
    let nlink = metadata.nlink();

    let mtime = metadata.modified().unwrap_or(SystemTime::UNIX_EPOCH);
    let datetime: DateTime<Local> = DateTime::<Local>::from(mtime) + Duration::hours(1);
    let time_str = datetime.format("%b %e %H:%M").to_string();

    print!(
        "{} {:>nlink_width$} {:<user_width$} {:<group_width$} ",
        mode_str,
        nlink,
        user,
        group,
        nlink_width = nlink_width,
        user_width = user_width,
        group_width = group_width
    );

    if is_device {
        let rdev = metadata.rdev();
        let major_num = major(rdev);
        let minor_num = minor(rdev);
        print!("{:>3}, {:>4} ", major_num, minor_num);
    } else {
        print!("{:>size_width$} ", metadata.size(), size_width = size_width);
    }

    print!("{} ", time_str);
}
