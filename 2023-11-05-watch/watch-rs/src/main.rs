use std::env;
use regex::Regex;
use std::process::Command;
use std::str;
use pancurses::{initscr, Input, Window};
use pancurses;
use chrono::Local;

#[derive(Debug)]
struct CmdOption {
    name: String,
    value: String,
    length: usize,
}

fn main() {
    match run() {
        Ok(()) => {
            std::process::exit(0);
        }
        Err(err) => {
            eprintln!("Error: {}", err);
            std::process::exit(1);
        }
    }
}

fn run() -> Result<(), String> {
    let mut option_n: u32 = 2; // The default is 2s.
    let args: Vec<String> = env::args().skip(1).collect();
    let mut i = 0;
    while i < args.len() {
        let cmd_options = get_option(i, &args)?;
        if cmd_options.is_none() {
            break;
        }
        let cmd_options = cmd_options.unwrap();
        let name = cmd_options.name;
        if name == "n" {
            match cmd_options.value.parse() {
                Ok(value) => option_n = value,
                Err(e) => return Err(format!("{}", e)),
            }
        } else {
            return Err(format!("{}: unknown option", name));
        }
        i = i + cmd_options.length;
    }
    let cmd: &[String] = &args[i..];
    let window = initscr();
    pancurses::curs_set(0);
    window.timeout(option_n as i32 * 1000);
    window.clear();
    window.refresh();
    let mut first = true;
    loop {
        if first {
            run_command(&window, option_n, cmd);
        }
        first = false;
        match window.getch() {
            Some(ch) => {
                if ch == Input::KeyResize {
                    run_command(&window, option_n, cmd);
                }
            }
            None => {
                run_command(&window, option_n, cmd);
            }
        }
    }
}

fn get_option(mut i: usize, args: &[String]) -> Result<Option<CmdOption>, String> {
    let re = Regex::new(r"-([[:alpha:]])(.*)$").unwrap();
    assert!(i < args.len());
    let arg = &args[i];
    if let Some(caps) = re.captures(arg) {
        let name: &str = &caps[1];
        let mut value: &str = &caps[2];
        if value.len() == 0 {
            i = i + 1;
            if i < args.len() {
                value = &args[i];
                return Ok(Some(CmdOption {
                    name: name.to_owned(),
                    value: value.to_owned(),
                    length: 2,
                }));
            } else {
                return Err("Missing option value".to_owned());
            }
        } else {
            return Ok(Some(CmdOption {
                name: name.to_owned(),
                value: value.to_owned(),
                length: 1,
            }));
        }
    } else {
        return Ok(None);
    }
}

fn run_command(window: &Window, n: u32, cmd: &[String]) {
    assert!(cmd.len() > 0);
    window.clear();

    let command_name = &cmd[0];
    let mut command = &mut Command::new(command_name);
    for arg in cmd.iter().skip(1) {
        // println!("arg: {}", arg);
        command = command.arg(arg);
    }
    let output = command.output().expect(&format!("Command {} failed", command_name));

    let output_stdout = match str::from_utf8(&output.stdout) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    };
    let width = window.get_max_x();

    window.mv(0, 0);
    let command_line = cmd.join(" ");
    window.printw(format!("Every {}s: {}", n, command_line));

    let date = Local::now();
    let date_str = date.format("%a %b %e %H:%M:%S %Y").to_string();
    let hostname = gethostname::gethostname();
    let hostname = hostname.to_string_lossy();
    let heading_message = format!("{}: {}", hostname, date_str);
    let pos = width - heading_message.len() as i32;
    window.mv(0, pos);
    window.printw(heading_message);

    window.mv(3, 0);
    window.printw(output_stdout);
    window.refresh();
}
