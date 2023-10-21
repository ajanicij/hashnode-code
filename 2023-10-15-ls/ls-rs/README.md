Finally we have all the elements we need to implement a mini version of
ls command in Rust.

Function main delegates to run. We do this in order to catch any errors
and display a more user-friendly error message:

```
fn main() {
    match run() {
        Ok(_) => return,
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
}
```

Function run will look like this:

```
fn run() -> Result<(), Box<dyn std::error::Error>> {
...
}
```

In the function, first we process the command line:

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
            arg!([FILE])
                .action(ArgAction::Append),
        )
        .get_matches();
```

We covered this in
[Reading command line arguments with clap](https://ajanicij.hashnode.dev/reading-command-line-arguments-with-clap).

Because we will pass the options around, we define a structure for all
options:

```
#[derive(Debug)]
struct LsOptions {
    long: bool,
    all: bool,
}
```

In the run function, after we have processed command line options, we
create an instance of LsOptions:

```
    let options = LsOptions {
        long: matches.get_flag("long"),
        all: matches.get_flag("all"),
    };
    
    if options.long {
        println!("Printing long information for each file...");
    }
    if options.all {
        println!("Including files whose name starts with '.' in the list...");
    }
```

Next, we collect the positional arguments:

```
    let mut files: Vec<String>;
    files = matches.get_many::<String>("FILE")
        .unwrap_or_default()
        .map(|v| v.as_str().to_owned())
        .collect();
    if files.len() == 0 {
        files.push(".".to_owned());
    }
```

The behaviour of ls command is slightly different if there is only one
positional argument: in that case it doesn't display the directory name
before the directory entries. For example:

```
$ ls . src
.:
Cargo.lock  Cargo.toml  message.sh  README.md  src  target

src:
main.rs  main.rs.old

$ ls .
Cargo.lock  Cargo.toml  message.sh  README.md  src  target
```

For that reason, we check if we have only one argument:

    let only_one = files.len() == 1;

Next, we look at each argument in a loop:

```
    for file in files {
        if let Some(entry) = MyDirEntry::new(file) {
            if entry.metadata.is_dir() {
                if !only_one {
                    show_directory_name(&entry)?;
                }
                let _ = show_directory_entries(&entry, &options);
            } else {
                show_file(&entry);
            }
        }
    }
```

Note how we use only_one to decide if we call show_directory_name or
not.

Function show_directory_entries lists directory entries. Its operation
depends on command line options:

- -a means we show all entries, including the entries whose name starts
  with '.'. It is a Unix convention that those entries are not shown
  by default .
- -l means we show more information about each entry.

```
fn show_directory_entries(entry: &MyDirEntry, options: &LsOptions) ->
    Result<(), Box<dyn std::error::Error>>
{
    let dir_entry_list = fs::read_dir(&entry.path)?;
    let mut entries: Vec<MyDirEntry> = Vec::new();
    for dir_entry in dir_entry_list {
        if let Ok(dir_entry) = dir_entry {
            let path = dir_entry.path();
            if let Some(entry) = MyDirEntry::new_from_path(path) {
                entries.push(entry);
            }
        }
    }
    entries.sort_by(|a, b| a.file.to_lowercase().partial_cmp(&b.file.to_lowercase()).unwrap());
    for entry in &entries {
        // If file name begins with '.' and -a was not used in the command,
        // we skip the file.
        if let Some(file_name) = entry.path.file_name() {
            if let Some(file_name) = file_name.to_str() {
                if (file_name.len() > 0) && (file_name.chars().nth(0).unwrap() == '.') &&
                    !options.all
                {
                    // Skip.
                    continue;
                }
            }
        }

        let _ = show_entry(entry, options);
    }
    Ok(())
}
```

In writing this function, it took me a lot of negotiations with the compiler:

- [fs::read_dir](https://doc.rust-lang.org/std/fs/fn.read_dir.html) returns
  Result<ReadDir>, forcing us to check if it failed.
- [ReadDir](https://doc.rust-lang.org/std/fs/struct.ReadDir.html) is an iterator
  whose item is Result<DirEntry, Error>, which again forces us to check if it
  failed.

We can use DirEntry to get the path, and from the path we can get the file's
metadata. In order to separate that code for better readability, we have
a structure that keeps file, path and metadata together:

```
#[derive(Debug)]
struct MyDirEntry {
    file: String,
    path: PathBuf,
    metadata: Metadata,
}
```

There are two ways to create an instance of MyDirEntry: from file or from path:

```
impl MyDirEntry {
    fn new(file: String) -> Option<MyDirEntry> {
        let path = PathBuf::from(&file);
        let metadata = match fs::metadata(file.clone()) {
            Ok(metadata) => metadata,
            _ => return None,
        };
        Some(MyDirEntry { file, path, metadata })
    }

    fn new_from_path(path: PathBuf) -> Option<MyDirEntry> {
        let file = path.into_os_string();
        if let Ok(file) = file.into_string() {
            return MyDirEntry::new(file);
        }
        None
    }
}
```

Function show_entry displays a directory entry:

```
fn show_entry(entry: &MyDirEntry, options: &LsOptions) -> Result<(), Error> {
    let metadata = &entry.metadata;
    let permissions = metadata.permissions();
    let mode = permissions.mode();
    let modified = metadata.modified()?;
    let now = Local::now();
    let tz = now.timezone();
    let datetime = systime_converter::convert(modified, tz);
    if options.long {
        println!("{} {} {} {} {} {} {}", mode_str(mode, &metadata), metadata.st_nlink(), metadata.st_uid(),
            metadata.st_gid(), metadata.len(), datetime.format("%Y %b %e %H:%M"),
            entry.file);
    } else {
        show_file(&entry);
    }
    Ok(())
}
```

Function show_file is called when we want to display just the file or directory
name without any other information:

```
fn show_file(entry: &MyDirEntry) {
    if entry.metadata.is_dir() {
        println!("{}", entry.file.blue());
    } else if is_executable(&entry.metadata) {
        println!("{}", entry.file.green());
    } else {
        println!("{}", entry.file);
    }
}
```

In this function, we used crate [colorize](https://docs.rs/colorize/latest/colorize/)
to give the entry different colors: if it is a directory, we paint it blue, and
if it is an executable file, we paint it green.

Note how this is done: trait [AnsiColor](https://docs.rs/colorize/latest/colorize/trait.AnsiColor.html)
has methods blue and green (and many others) to use escape sequences to give strings
a color. The crate colorize implements this trait for String and &str, so using it
is dead simple: for example, printing expression "xyz".blue() will render text "xyz"
in blue.

If the command has flag -l, we have to provide mode string: for example, command

    ls -l

in the project's root directory will show

```
-rw-rw-r-- 1 aleks aleks 12833 Oct 15 12:50 Cargo.lock
-rw-rw-r-- 1 aleks aleks   323 Oct 15 12:50 Cargo.toml
-rw-rw-r-- 1 aleks aleks  7622 Oct 21 12:29 README.md
drwxrwxr-x 2 aleks aleks  4096 Oct 15 15:43 src
drwxrwxr-x 3 aleks aleks  4096 Oct 15 11:00 target
```

The mode string contains entry type and permissions:

drwxrwxr-x is read like this:

- "d" is for directory; "l" would mean a symbolic link and "-" means a regular file
- rwx are flags for "read", "write" and "execute"; in case of a directory, "x" means
  "providing access to the directory". For more detailed explanation, see
  [here](https://www.redhat.com/sysadmin/linux-file-permissions-explained).
- There are three groups of permissions: for owner, group and others.

Here's the code:

```
fn mode_str(mode: u32, metadata: &fs::Metadata) -> String {
    let file_type = file_type_char(metadata);
    let mode_ur = perm_char(mode & libc::S_IRUSR, 'r');
    let mode_uw = perm_char(mode & libc::S_IWUSR, 'w');
    let mode_ux = perm_char(mode & libc::S_IXUSR, 'x');

    let mode_gr = perm_char(mode & libc::S_IRGRP, 'r');
    let mode_gw = perm_char(mode & libc::S_IWGRP, 'w');
    let mode_gx = perm_char(mode & libc::S_IXGRP, 'x');

    let mode_or = perm_char(mode & libc::S_IROTH, 'r');
    let mode_ow = perm_char(mode & libc::S_IWOTH, 'w');
    let mode_ox = perm_char(mode & libc::S_IXOTH, 'x');

    format!("{}{}{}{}{}{}{}{}{}{}",
        file_type,
        mode_ur, mode_uw, mode_ux,
        mode_gr, mode_gw, mode_gx,
        mode_or, mode_ow, mode_ox).to_string()

}

fn perm_char(flag: u32, ch: char) -> char {
    if flag != 0 {
        ch
    } else {
        '-'
    }
}
```

Get the code at
[2023-10-15-ls](https://github.com/ajanicij/hashnode-code/tree/master/2023-10-15-ls/ls-rs).
