extern mod ncurses;
use ncurses::window::{newwin, initscr};

fn main() {
    initscr(); // Start curses mode            
    let win = newwin(10, 20, 0, 0);

    for r in range(0, 10) {
        for c in range(0, 20) {
            win.mv(r, c);
            win.waddch('.');
        }
    }

    win.screenShot(~"output");
    win.endwin();        // End curses mode  
}

