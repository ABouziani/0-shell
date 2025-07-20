pub fn echo(args: &[&str]) {
    let output = args.join(" ");
    println!("{}", output);
}
