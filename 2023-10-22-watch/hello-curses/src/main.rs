use pancurses::{initscr, endwin};

fn main() {
    let window = initscr();
    window.printw("Hello pancurses");
    window.refresh();
    window.getch();
    endwin();
}
