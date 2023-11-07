In the
[previous episode](https://ajanicij.hashnode.dev/running-commands-in-a-subprocess),
we used
[Command](https://doc.rust-lang.org/std/process/struct.Command.html)
to run a command in a subprocess and capture its output. Before that, we
explored how to use
[pancurses](https://docs.rs/pancurses/latest/pancurses/)
([here](https://ajanicij.hashnode.dev/programming-curses-in-rust) and
[here](https://ajanicij.hashnode.dev/programming-curses-in-rust-part-2))
to get a full control of the terminal window.

Now we are ready to write watch-rs, a minimal Rust version of
[watch command] (https://linux.die.net/man/1/watch).

Our program can be run, for example, like this:

    cargo run -- -n5 ls -l

which runs command

    ls -l

repeatedly every 5 seconds.

First, I want to describe how we process command line arguments. I will confess
that this part required the most thought. I couldn't use clap, because I didn't
want clap to process command line options that should be passed to the command
we are running in the subprocess (-l in the example above). Also, I wanted to
be able to parse option

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
    name: Option<String>,
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
name: Some("n"),
value: "5",
length: 2
```

Note that the first field, name, is not just String, but Option<String>.
We do it like this so that we can distinguish a situation when no option
argument was found.

The function that processes command line arguments is declared like this:

    fn get_option(mut i: usize, args: &[String]) -> Result<CmdOption, String>

