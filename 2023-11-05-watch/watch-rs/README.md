In the
[previous episode](https://ajanicij.hashnode.dev/running-commands-in-a-subprocess),
we used
[Command](https://doc.rust-lang.org/std/process/struct.Command.html)
to run a command in a subprocess and capture its output. Before that, we
explored how to use
[pancurses](https://docs.rs/pancurses/latest/pancurses/)
([here](https://ajanicij.hashnode.dev/programming-curses-in-rust) and
[here](https://ajanicij.hashnode.dev/programming-curses-in-rust-part-2))
to get full control of the terminal window.

Now we are ready to write watch-rs, a minimal Rust version of
[watch command](https://linux.die.net/man/1/watch).

Our program can be run, for example, like this:

    cargo run -- -n5 ls -l

which runs command

    ls -l

repeatedly every 5 seconds.

First, I want to describe how we process command line arguments. I will confess
that this part required the most thought. I couldn't use clap, because I didn't
want clap to process command line options that should be passed to the command
we are running in the subprocess (-l in the example above). Also, I wanted to
be able to parse the option

    -n5

and

    -n 5

equally, so command

    cargo run -- -n 5 ls -l

works equally as the one above.

## Argument processing

Structure that describes one command line option is:

```
struct CmdOption {
    name: String,
    value: String,
    length: usize,
}
```

where:

- name is the option name, for example "n"
- value is the option value, for example "5"
- length is the number of command line arguments processed

For example, if the command line running our program (using cargo) is

    cargo run -- -n 5 ls -l

then our program has to process the following arguments:

- "-n"
- "5"
- "ls"
- "-l"

Of these, the first two together specify option argument "-n" and value
"5", so the value of CmdOption is

```
name: "n",
value: "5",
length: 2
```

The function that processes command line arguments is declared like this:

    fn get_option(mut i: usize, args: &[String]) -> Result<Option<CmdOption>, String>

Note that the Ok type of the result is Option<CmdOption>. We want to distinguish the
case when we found no option.

Here's the function get_option:

```
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
```

We first match the ith argument against the regular expression

    -([[:alpha:]])(.*)$

If we have a match, then the argument is an option, in the form

    -n2

or

    -n

If it is the first form, then this argument contains both the option name
and option value. We construct a CmdOption and return it.

If it is the second form (-n, for example), then we have to get the option
value from the next argument. If this is the last argument, it is an error
condition (missing option value). Otherwise, we get the value from the next
argument, construct a CmdOption and return it.

If the argument didn't match the regular expression, that means we are done
processing options and the rest of the arguments form the command that we
will run.

We call get_option from the run function repeatedly until we run out of options:

```
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
```

In our case, of course, there is only one option that we accept
(-n), but I found it easier and cleaner to structure the code like
this, for two reasons:

- It is easier to understand.
- It is easier to add more options. The real watch command supports a lot
  more options, and adding code that parses them would be straightforward.

At the end of the code snippet above, cmd is a slice of strings that form
the command we are running, option_n is the sleep interval (in seconds)
between to executions of the command. Here's how we run the command:

```
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
```

## Running the command in a subprocess

In the
[previous episode](https://ajanicij.hashnode.dev/running-commands-in-a-subprocess),
we showed how to run a command in a subprocess. Function run_command does that
and outputs the result in the ncurses window.

In the code above, in a loop we call run_command and then wait for either timeout
or window resize event. In either case, we call run_command again, which will
redraw the window.

We showed how to work with ncurses
[here](https://ajanicij.hashnode.dev/programming-curses-in-rust)
and [here](https://ajanicij.hashnode.dev/programming-curses-in-rust-part-2).
Here's the function run_command:

```
fn run_command(window: &Window, n: u32, cmd: &[String]) {
    assert!(cmd.len() > 0);
    window.clear();

    let command_name = &cmd[0];
    let mut command = &mut Command::new(command_name);
    for arg in cmd.iter().skip(1) {
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
```

The function runs the command in a subprocess and captures its standard
output. We completely ignore standard error and the return status of
the command; the real watch command has options for dealing with them.

After we captured the standard output of the command, we display it in
the window. The first line of output is the heading, which looks like
this:

    Every 10s: ls -l                                juliet: Wed Nov  8 09:08:50 2023

I made this match what the watch command displays:

- "Every 10s:" - show the interval between two runs of the command
- "ls -l" - the command
- "juliet" - host name
- "Wed Nov  8 09:08:50 2023" - date and time

Note the code that displays the host and date-time:

```
    let heading_message = format!("{}: {}", hostname, date_str);
    let pos = width - heading_message.len() as i32;
    window.mv(0, pos);
    window.printw(heading_message);
```

This code uses the window width to right-align the text.

Get the complete code at
[2023-11-05-watch/watch-rs](https://github.com/ajanicij/hashnode-code/tree/master/2023-11-05-watch/watch-rs).
