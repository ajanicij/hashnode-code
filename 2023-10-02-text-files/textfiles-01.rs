use std::env;
use std::process::exit;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: read_file <input-file>");
        exit(0);
    }

    let file_path = &args[1];
    let s = fs::read_to_string(file_path).unwrap();
    println!("File contents:\n{}", s);
}
