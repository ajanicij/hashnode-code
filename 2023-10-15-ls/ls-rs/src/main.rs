use clap::{arg, command, Arg, ArgAction};
use std::fs;
use std::io::Error;
use std::fs::Metadata;
use std::os::linux::fs::MetadataExt;
use std::os::unix::fs::PermissionsExt;
use libc;
use chrono::Local;
use systime_converter;
use colored::Colorize;
use std::path::PathBuf;

#[derive(Debug)]
struct LsOptions {
    long: bool,
    all: bool,
}

#[derive(Debug)]
struct MyDirEntry {
    file: String,
    path: PathBuf,
    metadata: Metadata,
}

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

fn main() {
    match run() {
        Ok(_) => return,
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
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

    // Get positional arguments.
    let mut files: Vec<String>;
    files = matches.get_many::<String>("FILE")
        .unwrap_or_default()
        .map(|v| v.as_str().to_owned())
        .collect();
    if files.len() == 0 {
        files.push(".".to_owned());
    }

    // only_one: we display the directory name only if we have more
    //           than one entries on command line.
    let only_one = files.len() == 1;
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

    Ok(())
}

fn show_directory_name(entry: &MyDirEntry) -> Result<(), Box<dyn std::error::Error>>
{
    println!("{}:", entry.file);
    Ok(())
}

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

fn show_file(entry: &MyDirEntry) {
    if entry.metadata.is_dir() {
        println!("{}", entry.file.blue());
    } else if is_executable(&entry.metadata) {
        println!("{}", entry.file.green());
    } else {
        println!("{}", entry.file);
    }
}

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

fn file_type_char(metadata: &fs::Metadata) -> char {
    if metadata.is_dir() {
        'd'
    } else if metadata.is_symlink() {
        'l'
    } else {
        '-'
    }
}

fn is_executable(metadata: &fs::Metadata) -> bool {
    let permissions = metadata.permissions();
    let mode = permissions.mode();
    return (mode & libc::S_IXUSR) != 0;
}

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
