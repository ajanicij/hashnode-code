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

