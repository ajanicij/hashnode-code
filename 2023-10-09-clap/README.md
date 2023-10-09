This post is the first in a series that will develop a small rewrite of
ls command with a small subset of features of the official Linux ls
command.

For a Rust implementation of ls that aspires to implement the full 
rewrite, take a look at
[coreutils](https://github.com/uutils/coreutils/tree/main/src/uu/ls/src).

Here we are going to develop a small subset, just to try out some features
and Rust crates.

In this first post, we are going to read command line parameters. For that,
we will use [clap](https://docs.rs/clap/latest/clap/), a fantastic crate
that supports all the features we need.

We will only support two flags, -l and -a, but clap will give us for free
two more: -h (for help) and -V (for printing program version). When we
run cargo run -- -h, it will print out the help text:

```
Mini ls implemented in Rust

Usage: ls-rs [OPTIONS] [FILE]...

Arguments:
  [FILE]...  

Options:
  -l             use a long listing format
  -a, --all      do not ignore entries starting with .
  -h, --help     Print help
  -V, --version  Print version
```

## Creating the new crate

Run the following command to create the new crate:

    cargo new ls-rs

Then cd into the directory ls-rs and add clap to Cargo.toml:

    cargo add clap

At the time of writing, when I do that, get this in the [dependencies]
section of Cargo.toml:

```
[dependencies]
clap = { version = "4.4.6", features = ["derive", "cargo"] }
```

## Reading the arguments

To process the command line arguments, we need to tell clap what
flags we will accept. Also, we need to tell it that we will accept zero
or more positional arguments, which will be the list of files or
directories.

```
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
```

Note: I am assuming that you do this on Linux, and the shell on any
Linux processes the command line in the same way: it first expands any
wildcards before passing the command line arguments to the program.
That means that, for example, if we run

    my-program *

the program my-program is not going to see "\*" as is only argument. The shell
will first expand "\*" to a list of all entries in the current directory,
and then pass that on to the program. If we are in the root directory of the
crate (and if we have already executed cargo build, so it has created Cargo.lock),
then the root directory contains the following entries:

    Cargo.lock  Cargo.toml  src  target

so the expanded program line will be:

    my-program Cargo.lock  Cargo.toml  src  target

## Getting the information from parsing the command line

After we have parsed the command line, we have matches variable which holds
all the information. We get a flag by calling get_flag, and the list of
positional arguments by calling get_many.

```
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
```

Here we a just printing the information; in the next post we will start getting
the properties of each entry.

For more information on how to use clap, go to its documentation page.
It is complete and has examples, but I have to say it took me some trial and
error to figure it out.

