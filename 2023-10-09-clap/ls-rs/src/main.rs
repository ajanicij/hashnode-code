use clap::{arg, command, Arg, ArgAction};

fn main() {
    let matches = command!("ls") // requires `cargo` feature
        .version("0.1")
        .author("Aleksandar J. <ajanicij@yahoo.com>")
        .about("Mini ls implemented in Rust")
        .arg(
            Arg::new("long")
                .short('l')
                .action(ArgAction::SetTrue)
                .help("use a long listing format"),
        )
        .arg(
            Arg::new("all")
                .short('a')
                .long("all")
                .action(ArgAction::SetTrue)
                .help("do not ignore entries starting with .")
        )
        .arg(
            // Arg::new("file")
            arg!([FILE])
                .action(ArgAction::Append),
        )
        .get_matches();

    println!("long: {:?}", matches.get_flag("long"));
    println!("all: {:?}", matches.get_flag("all"));
    if matches.get_flag("long") {
        println!("Printing long information for each file...");
    }
    if matches.get_flag("all") {
        println!("Including files whose name starts with '.' in the list...");
    }

    // Try to get all positional arguments.
    let files: Vec<String>;
    files = matches.get_many::<String>("FILE")
        .unwrap_or_default()
        .map(|v| v.as_str().to_owned())
        .collect();
    println!("files: {:?}", files);
    for file in &files {
        println!("{}", file);
    }
}
