use std::env;
use std::process::exit;
use std::fs;
use fs::File;
use std::io;
use std::io::BufRead;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: read_file <input-file>");
        exit(0);
    }

    let file_path = &args[1];
    let file = File::open(file_path).unwrap();
    let reader = io::BufReader::new(file);
    let lines = reader.lines();
    for line in lines {
        let s = line.unwrap();
        println!("Got line: {}", s);
    }
}
