extern mod ncurses;

use nc = ncurses::ncurses;
// use t = ncurses::types;
use ncurses::window::{initscr};
use ncurses::window::Window;

fn main() {
    // Start curses mode                                                      
    let win = initscr();
    

    win.screenShot(~"output");
    nc::refresh();       // Print it on to the real screen 
    win.endwin();        // End curses mode  
}

