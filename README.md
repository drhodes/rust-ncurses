Bindings for ncurses (libncurses 5)
=============
These aren't ready yet :)

Hello World
-----------

Hello world is working on linux x86

$ make test

<pre>
fn hello() {
    ncurses::initscr();       // Start curses mode  
    printw("Hello World!");

    ncurses::refresh();       // Display 
    ncurses::getch();         // Wait for user input 
    ncurses::endwin();        // End curses mode 
}
</pre>