In the [previous episode](https://ajanicij.hashnode.dev/mini-version-of-ls-implemented-in-rust),
we developed a minimal ls command implemented in Rust.
In this episode we are starting to work
on a Rust implementation of Linux command watch:

```
NAME
       watch - execute a program periodically, showing output fullscreen

SYNOPSIS
       watch [options] command

DESCRIPTION
       watch  runs  command  repeatedly, displaying its output and errors (the
       first screenfull).  This allows you to watch the program output  change
       over  time.   By default, command is run every 2 seconds and watch will
       run until interrupted.
```

For example, command

    watch -n 5 ls -l

will run command "ls -l" every 5 seconds and display its output, until interrupted.
Its output looks like this:

```
Every 5.0s: ls -l                               juliet: Sun Oct 22 10:47:35 2023

total 8
drwxrwxr-x 4 aleks aleks 4096 Oct 22 10:37 hello-curses
-rw-rw-r-- 1 aleks aleks  530 Oct 22 10:45 README.md
```

watch command has lots of command line flags and complex functionality, but in our
minimal version we will just implement "-n" option that allows us to set the
refresh period (the default is 2s).

In the first post we will explore how to control the whole shell window: clear it,
move cursor to a specified location and write text at that location. We will also
want to restore the previous window contents upon exit.

For this functionality, the library that is commonly used is
[ncurses](https://en.wikipedia.org/wiki/Ncurses):

```
ncurses (new curses) is a programming library providing an application programming interface
(API) that allows the programmer to write text-based user interfaces (TUI) in a
terminal-independent manner. It is a toolkit for developing "GUI-like" application software
that runs under a terminal emulator. It also optimizes screen changes, in order to reduce
the latency experienced when using remote shells.
```

There are several Rust crates that can be used for working with ncurses. I chose
[pancurses](https://docs.rs/pancurses/latest/pancurses/).

This is a minimal program that uses pancurses:

```
use pancurses::{initscr, endwin};

fn main() {
    let window = initscr();
    window.printw("Hello pancurses");
    window.refresh();
    window.getch();
    endwin();
}
```

initscr initializes the curses system and returns an instance of the Window struct.
We use that object to print text by calling the method printw.
Remember that we have to call refresh after writing to the window.

After refreshing the window, we call getch, which waits for the user to click on
any key. After the user has clicked on a key, we ignore that input, call
method endwin to clean up and then exit.

In the next episode, we will look into more functionality provided by
pancurses that we will need for our implementation of watch command.
