In the
[previous episode](https://ajanicij.hashnode.dev/programming-curses-in-rust),
we talked about ncurses and started exploring
[pancurses](https://docs.rs/pancurses/latest/pancurses/). Now we will
introduce the rest of the functionality in pancurses that we need for
mini watch:

- positioning the cursor
- reacting to a window resizing event
- detecting a timeout

## Positioning the cursor

For positioning the cursor, we use
[mv](https://docs.rs/pancurses/latest/pancurses/struct.Window.html#method.mv):

    pub fn mv(&self, y: i32, x: i32)

For clearing the window, we use
[clear](https://docs.rs/pancurses/latest/pancurses/struct.Window.html#method.clear):

    pub fn clear(&self) -> i32

Here's a little program that utilizes both:

```
use pancurses::{initscr, endwin, curs_set};
use pancurses;

fn main() {
    let window = initscr();
    window.mv(0, 0);
    window.printw("Hello Rust");
    window.refresh();

    window.getch();
    window.mv(10, 0);
    window.printw("hello again!");
    window.refresh();

    window.getch();
    window.mv(10, 20);
    window.printw("and again!");
    window.refresh();

    window.getch();
    window.clear();
    window.refresh();

    window.getch();
    window.mv(0, 20);
    window.printw("bye now!");
    window.refresh();

    window.getch();
    endwin();
}
```

Note the order of arguments in mv: it's mv(y, x), where y is the line, counting from the top,
and x is the distance from the left edge of the window.

## Reacting to a window resizing event

We will need to be able to detect a screen resize event and to react to it. The way it is
done in pancurses (and ncurses) is to call method getch check the value it returns: if the
window was resized, the value will be
[Input::KeyResize](https://docs.rs/pancurses/latest/pancurses/enum.Input.html#variant.KeyResize).

Here is a simple code that demonstrates this:

```
use pancurses::{initscr, endwin, curs_set, Input};
use pancurses;

fn main() {
    let window = initscr();
    window.printw("Hello Rust");
    pancurses::curs_set(0);
    window.refresh();

    loop {
        let ch = window.getch().unwrap();
        if ch == Input::KeyResize {
            window.clear();
            let width = window.get_max_x();
            let height = window.get_max_y();
            let x = height/2;
            let y = width/2;
            window.mv(x, y);
            window.printw(format!("resized: {}x{}", width, height));
            window.refresh();
        } else {
            break;
        }
    }

    endwin();
}
```

In this code, when we get a KeyResize event, we get the new window width
and height and print a message in the middle of the window.

## Detecting a timeout

We set the timeout by calling the timeout method. Then, when we call getch,
it will return Some(code) if the user clicked on a key, or None if we
timed out.

This sample keeps moving an "x" in the window, one step to the right every
second:

```
use pancurses::{initscr, endwin, curs_set, Input};
use pancurses;

fn main() {
    let window = initscr();
    pancurses::curs_set(0);
    window.timeout(1000);
    window.refresh();

    let mut x = 0;
    loop {
        match window.getch() {
            Some(_) => break,
            None => { // timeout
                x = x + 1;
                if x == 20 {
                    x = 0;
                }
                window.clear();
                window.mv(5, x);
                window.printw("x");
                window.refresh();
            }
        }
    }

    endwin();
}
```

Get the code at
[2023-10-22-watch](https://github.com/ajanicij/hashnode-code/tree/master/2023-10-22-watch).

In the next episode, we run a program in a subprocess and capture its standard
output.
