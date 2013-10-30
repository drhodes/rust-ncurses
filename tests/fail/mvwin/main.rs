extern mod ncurses;
use nc = ncurses::ncurses;
use ncurses::window::{newwin, initscr};

fn main() {
    let scr = initscr(); // Start curses mode            
    let win = newwin(5, 10, 0, 0);
    
    win.mvwin(1,1);
    for r in range(0, 5) {
        for c in range(0, 10) {
            win.wmove(r as i32, c as i32);
            win.waddch('.');
        }
    }

    // win.wrefresh();
    nc::refresh();
    let temp = nc::getch();         // Wait for user input 
    scr.endwin();        // End curses mode  

}
