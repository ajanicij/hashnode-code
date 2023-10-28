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
