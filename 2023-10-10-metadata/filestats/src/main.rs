use std::fs;
use std::process::exit;
use std::env;
use std::os::unix::fs::PermissionsExt;
use chrono::Local;
use std::os::linux::fs::MetadataExt;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: filestats <input-file>");
        exit(0);
    }
    let file = &args[1];
    println!("file: {}", file);
    let metadata = fs::metadata(file)?;
    println!("{:?}", metadata.file_type());
    let is_dir = metadata.is_dir();
    println!("is_dir: {}", is_dir);
    let is_file = metadata.is_file();
    println!("is_file: {}", is_file);
    let permissions = metadata.permissions();
    let mode = permissions.mode();
    println!("permissions: {:o}", permissions.mode());
    println!("S_IRUSR: {}", (mode & libc::S_IRUSR != 0));
    println!("mode: {}{}", file_type_char(&metadata), mode_str(mode));
    let modified = metadata.modified()?;
    println!("modified: {:?}", modified);
    let now = Local::now();
    let tz = now.timezone();
    let datetime = systime_converter::convert(modified, tz);
    println!("strftime: {}", datetime.format("%Y %b %e %H:%M"));
    println!("number of hard links: {}", metadata.st_nlink());
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

fn mode_str(mode: u32) -> String {
    let mode_ur = perm_char(mode & libc::S_IRUSR, 'r');
    let mode_uw = perm_char(mode & libc::S_IWUSR, 'w');
    let mode_ux = perm_char(mode & libc::S_IXUSR, 'x');

    let mode_gr = perm_char(mode & libc::S_IRGRP, 'r');
    let mode_gw = perm_char(mode & libc::S_IWGRP, 'w');
    let mode_gx = perm_char(mode & libc::S_IXGRP, 'x');

    let mode_or = perm_char(mode & libc::S_IROTH, 'r');
    let mode_ow = perm_char(mode & libc::S_IWOTH, 'w');
    let mode_ox = perm_char(mode & libc::S_IXOTH, 'x');

    format!("{}{}{}{}{}{}{}{}{}",
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
