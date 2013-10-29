extern mod ncurses;

use nc = ncurses::ncurses;
// use t = ncurses::types;
use ncurses::window::{newwin, initscr};
use ncurses::window::Window;

fn main() {
    // Start curses mode            
    initscr();
    let win = newwin(10, 20, 0, 0);
    win.screenShot(~"output");
    nc::refresh();       // Print it on to the real screen 
    win.endwin();        // End curses mode  
}

