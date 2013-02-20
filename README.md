Bindings for ncurses (libncurses 5)

updated and tested on
rustc 0.6 (9143688 2013-02-14 14:44:12 -0800)
host: i686-unknown-linux-gnu

=============================================================================
These aren't ready yet, they need work, pull requests welcome :)

Warning: The source contained herein probably isn't up to snuff with 
current rust idioms.  I'm holding off implementing this until rust 
.99 or 1.0 . 

Hello World
-----------

$ make test

<pre>
fn hello2() {
    let win = NewWindow();
    unsafe {
        win.attrset(A_BOLD);
    	win.addstr(1, 1, ~"你好 Hello");
    
    	for int::range(2i, 9i) |n|{
                attrset(REVERSE);
                win.addstr(n as int, (n as int)*2, ~" World !!!");
                ncurses::refresh();/* paint to the display */
    			}
        ncurses::getch();/* Wait for user input */
        curses::endwin();/* End curses mode  */ 
    }
}
</pre>