use pancurses::{initscr, endwin};
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
