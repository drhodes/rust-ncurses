extern mod ncurses;

use nc = ncurses::ncurses;
use t = ncurses::types;
use ncurses::window::{initscr};
use ncurses::window::Window;

fn status(line: int, win: &Window, b: bool, s: ~str) {
    if b {
        win.addstr(line, 2, ~" True: " + s);
    } else {
        win.addstr(line, 2, ~"False: " + s);
    }
}


fn main() {
    // Start curses mode                                                      
    let win = initscr();

    nc::printw(~"你好 Hello");
    nc::printw(~" World !!!");

    status(3, &win, win.is_nodelay(), ~"delay");
    status(4, &win, win.is_notimeout(), ~"timeout");
    status(5, &win, win.is_scrollok(), ~"scroll ok");
    status(6, &win, win.is_cleared(), ~"is_cleared");
    status(7, &win, win.is_idcok(), ~"is_idcok");
    status(8, &win, win.is_idlok(), ~"is_idlok");
    status(9, &win, win.is_immedok(), ~"is_immedok");
    status(10, &win, win.is_keypad(), ~"is_keypad");
    status(11, &win, win.is_leaveok(), ~"is_leaveok");
    status(12, &win, win.is_nodelay(), ~"is_nodelay");
    status(13, &win, win.is_syncok(), ~"is_syncok");

    let (top, bot) = win.wgetscrreg();       
    let msg = format!("Margin: top: {}, bottom: {}", top, bot);
    win.addstr(14, 2, msg);

    nc::refresh();       // Print it on to the real screen 
    nc::getch();         // Wait for user input 
    win.endwin();        // End curses mode  
}

