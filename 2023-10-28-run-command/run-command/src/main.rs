use std::process::Command;
use std::str;

fn main() {
    println!("Hello, world!");
    let cmd = vec!["ls", "-l", "-a"];
    run_command(&cmd);
}

fn run_command(cmd: &Vec<&str>) {
    assert!(cmd.len() > 0);
    let command_name = cmd[0];
    let mut command = &mut Command::new(command_name);
    for arg in cmd.iter().skip(1) {
        // println!("arg: {}", arg);
        command = command.arg(arg);
    }
    let output = command.output().expect(&format!("Command {} failed", command_name));

    println!("Status: {}", output.status);
    if let Some(output_stderr) = str::from_utf8(&output.stderr).ok() {
        println!("stderr: {:?}", output_stderr);
    }
    
    let output_stdout = match str::from_utf8(&output.stdout) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    };
    println!("stdout:\n{}", output_stdout);
}
