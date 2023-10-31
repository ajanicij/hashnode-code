In the previous two episodes
([here](https://ajanicij.hashnode.dev/programming-curses-in-rust) and 
[here](https://ajanicij.hashnode.dev/programming-curses-in-rust-part-2)),
we discussed using the ncurses library via Rust crate
[pancurses](https://docs.rs/pancurses/latest/pancurses/).
In this episode, we are going to run a shell command in a subprocess and
capture its output. We will use
[Command](https://doc.rust-lang.org/std/process/struct.Command.html),
which is a structure in
[std::process](https://doc.rust-lang.org/std/process/index.html),
a standard Rust module for working with processes.

It is so easy:

- Create a new instance of Command by calling Command::new
- Add command line arguments by calling the method arg
- Spawn a subprocess by calling the method output
- Code will wait for the subprocess to exit and get the returned value from
  output, which contains the return status, standard output and standard error.

From the process module's documentation:

```
use std::process::Command;

let output = Command::new("echo")
                     .arg("Hello world")
                     .output()
                     .expect("Failed to execute command");

assert_eq!(b"Hello world\n", output.stdout.as_slice());
```

Here, we create a new instance of Command for running echo command, add
one argument (string "Hello world"), call method output and verify that
the command succeeded and that the string it wrote to the standard
output is "Hello world\n".

So here's now a little program that demonstrates how to use Command.
We hard code execution of command

    ls -l -a

in a subprocess and we capture the command's status, standard error and
standard output.

```
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
```

