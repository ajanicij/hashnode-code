use std::process::exit;
use std::env;
use std::fs;
use std::fs::DirEntry;
use std::io::{Error, ErrorKind};

fn extract_str_from_dir_entry(entry: &DirEntry) -> Result<String, Error> {
    let path = entry.path();
    let file_name = match path.file_name() {
        Some(file_name) => file_name,
        None => return Err(Error::new(ErrorKind::Other, "bad path name"))
    };
    let file_name = match file_name.to_str() {
        Some(file_name) => file_name,
        None => return Err(Error::new(ErrorKind::Other, "path name not a string"))
    };
    Ok(file_name.to_owned())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: readdir <dir-name>");
        exit(0);
    }
    let dir = &args[1];
    println!("Reading directory {}", dir);
    let metadata = fs::metadata(dir)?;
    let is_dir = metadata.is_dir();
    if !is_dir {
        eprintln!("{} is not a directory", dir);
        exit(1);
    }

    let pathlist = fs::read_dir(dir)?;
    let mut entries = Vec::new();
    for entry in pathlist {
        let entry = entry?;
        let file_name = extract_str_from_dir_entry(&entry)?;
        entries.push(file_name);
    }
    println!("Sorted:");
    entries.sort_by(|a, b| a.to_lowercase().partial_cmp(&b.to_lowercase()).unwrap());
    for entry in entries {
        println!("{}", entry);
    }
    Ok(())
}
