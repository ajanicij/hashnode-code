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
