// use std::fs::{File, OpenOptions};
// use std::io::{self, Write};

pub fn echo(args: &Vec<&str>) {
    if args.is_empty() {
        println!();
        return;
    }

    // // redirection check
    // if let Some(pos) = args.iter().position(|x| x == ">" || x == ">>") {
    //     let (text_parts, redir_parts) = args.split_at(pos);
    //     let text = text_parts.join(" ");
    //     let append = redir_parts[0] == ">>";

    //     if redir_parts.len() < 2 {
    //         eprintln!("echo: no file specified");
    //         return;
    //     }

    //     let filename = &redir_parts[1];
    //     let result = if append {
    //         OpenOptions::new().create(true).append(true).open(filename)
    //     } else {
    //         File::create(filename)
    //     };

    //     match result {
    //         Ok(mut file) => {
    //             if let Err(e) = writeln!(file, "{}", text) {
    //                 eprintln!("echo: write error: {}", e);
    //             }
    //         }
    //         Err(e) => {
    //             eprintln!("echo: failed to open {}: {}", filename, e);
    //         }
    //     }
    // } else {
        println!("{}", args.join(" "));
    // }
}
