// ------------------------------------------------------------------
// Copyright 2012 Derek A. Rhodes.  All rights reserved.
// Use of this source code is governed by the ncurses
// license that can be found in the LICENSE file.
// ------------------------------------------------------------------
// Function Comments Copyright
// (c) 1998-2007,2008 Free Software Foundation, Inc.
// ------------------------------------------------------------------

use std::libc::types::os::arch::c95::{ c_char, c_int, c_short, c_long};
use std::libc::types::common::c95::{ c_void, FILE};
use std::rt::io::Writer;

use nc = ncurses;
use c = ncurses::c;
use t = types;
use types::{TRUE, FALSE, OK, ERR};
use std::char::from_u32;
use std::rt::io;

pub struct Window {
    win: *t::WINDOW
}

/// Drop the window and cleanup the underlying C memory.
impl Drop for Window {
    #[fixed_stack_segment]
    fn drop(&mut self) {
        unsafe {
            error!("Deleting window.")
            c::delwin(self.win);
        }
    }
}

/// initscr() is used to initialize the ncurses data structures and to
/// read the proper terminfo file. Memory for and will be allocated. If an
/// error occurs, initscr will return Err, otherwise a pointer to will be
/// returned. Additionally, the screen will be erased and and will be
/// initialized.
#[fixed_stack_segment]
pub fn initscr() -> Window {
    unsafe {
        Window{win: c::initscr()}
    }
}

/// Calling newwin creates and returns a pointer to a new window with the
/// given number of lines and columns. The upper left-hand corner of the
/// window is at line begin_y, column begin_x. If either nlines or ncols
/// is zero, they default to LINES - begin_y and COLS - begin_x. A new
/// full-screen window is created by calling newwin(0,0,0,0).
#[fixed_stack_segment]
pub fn newwin(nlines: i32, ncols: i32, 
              begin_y: i32, begin_x: i32) -> Window {
    unsafe {
        Window{win: c::newwin(nlines, ncols, begin_y, begin_x)}
    }
}

impl Window {       
    //
    pub fn screenShot(&self, filename: ~str) {
        let bottom = self.getmaxy();        
        let right = self.getmaxx();
        
        let p = &Path::new(filename);
        let mut of = io::file::open(p, io::Create, io::Write).unwrap();

        error!("{}, {}", bottom, right);
        
        for row in range(0, bottom) {
            for col in range(0, right) {
                let mut bs: ~[u8] = ~[];
                match self.mvwinch(row, col) {
                    Some(c) => {
                        bs.push(c as u8);
                    }                    
                    None => {
                        bs.push('?' as u8);
                        }
                }
                of.write(bs);
            }
        }
    }   
    
    #[fixed_stack_segment]    
    pub fn addch(&self, ch: t::chtype) -> int {        
        unsafe {
            c::addch(ch) as int
        }
    }

    #[fixed_stack_segment] 
    /// Endwin will clean up all allocated resources from ncurses and restore
    /// the tty modes to the status they had before calling initscr(). It must
    /// be called before any other function from the ncurses library and
    /// endwin() must be called before your program exits. When you want to do
    /// output to more than one terminal, you can use newterm(...) instead of
    /// initscr().
    pub fn endwin(&self) { 
        unsafe { c::endwin(); }
    }

    /// Write the characters of the string str on the window starting at
    /// the current or specified position using the background rendition.
    #[fixed_stack_segment] 
    pub fn addstr(&self, y: int, x: int, s: ~str) {       
        unsafe {
            self.mv(y, x);        
            let cs = s.to_c_str().unwrap();
            c::addstr(cs);
        }
    }

    /// Move to a point in the window. The coordinate y always refers to
    /// the row (of the window), and x always refers to the column. The
    /// upper left-hand corner is always (0,0), not (1,1).
    #[fixed_stack_segment] 
    pub fn mv (&self, y: int, x: int) -> int {
        unsafe {
            c::wmove(self.win, y as c_int, x as c_int) as int
        }
    }


    /// Getter for C structure field: nodelay
    #[fixed_stack_segment]
    pub fn is_nodelay (&self) -> bool {
        unsafe {
            c::is_nodelay(self.win)
        }
    }

    /// Getter for C WINDOW field: notimeout
    #[fixed_stack_segment]
    pub fn is_notimeout (&self) -> bool {
        unsafe {
            c::is_notimeout(self.win)                
        }
    }

    /// Getter for C WINDOW field: scrollok
    #[fixed_stack_segment]
    pub fn is_scrollok(&self) -> bool {        
        unsafe { 
            c::is_scrollok(self.win) 
        }
    }

    /// Getter for C WINDOW field: cleared
    #[fixed_stack_segment]
    pub fn is_cleared (&self) -> bool { 
        unsafe {
            c::is_cleared(self.win)
        } 
    }

    /// Getter for C WINDOW field: idcok
    #[fixed_stack_segment]
    pub fn is_idcok (&self) -> bool { 
        unsafe {
            c::is_idcok(self.win)
        } 
    }

    /// Getter for C WINDOW field: idlok
    #[fixed_stack_segment]
    pub fn is_idlok (&self) -> bool { 
        unsafe {
            c::is_idlok(self.win)
        } 
    }

    /// Getter for C WINDOW field: immedok
    #[fixed_stack_segment]
    pub fn is_immedok (&self) -> bool { 
        unsafe {
            c::is_immedok(self.win)
        } 
    }

    /// Getter for C WINDOW field: keypad
    #[fixed_stack_segment]
    pub fn is_keypad (&self) -> bool { 
        unsafe {
            c::is_keypad(self.win)
        } 
    }

    /// Getter for C WINDOW field: leaveok
    #[fixed_stack_segment]
    pub fn is_leaveok (&self) -> bool { 
        unsafe {
            c::is_leaveok(self.win)
        } 
    }

    /// Getter for C WINDOW field: syncok
    #[fixed_stack_segment]
    pub fn is_syncok (&self) -> bool { 
        unsafe {
            c::is_syncok(self.win)
        } 
    }


    /// returns the top and bottom rows for the scrolling margin as set in wsetscrreg.
    #[fixed_stack_segment]
    pub fn wgetscrreg (&self) -> (int, int) {
        unsafe {
            let mut top: c_int = 0;
            let mut bottom: c_int = 0;
            let err = c::wgetscrreg(self.win, &top, &bottom);
            if err != OK {
                error!("window.wgetscreg produced an error.");
                error!("Err: {}", err);
            }
            return (top as int, bottom as int);
        }
    }

    /// box(win, verch, horch) is a shorthand for the following call:
    /// wborder(win, verch, verch, horch, horch, 0, 0, 0, 0).
    #[fixed_stack_segment]
    pub fn box (&self, c1: int, c2: int) -> int {
        unsafe {
            let n1 = c1 as t::chtype;
            let n2 = c2 as t::chtype;
            return c::box(self.win, n1, n2) as int;
        }
    }
    
    /// Calling derwin is the same as calling subwin, except that begin_y and
    /// begin_x are relative to the origin of the window orig rather than the
    /// screen. There is no difference between the subwindows and the derived
    /// windows.
    #[fixed_stack_segment]
    pub fn derwin(&self, nlines: i32, ncols: i32, 
                  begin_y: i32, begin_x: i32) -> Window {
        unsafe {
            Window{win: c::derwin(self.win, nlines, ncols, begin_y, begin_x)}
        }  
    }

    /// Duplicate the window, deep copy.
    #[fixed_stack_segment]
    pub fn dupwin (&self) -> Window {
        unsafe {
            let w = c::dupwin(self.win);
            return Window{win: w};            
        }
    }

    /// Get the window's current background character/attribute pair.
    #[fixed_stack_segment]
    pub fn getbkgd (&self) -> t::chtype {
        unsafe {
            c::getbkgd(self.win)
        }
    }

    /// If idcok is called with FALSE as second argument, curses no longer
    /// considers using the hardware insert/delete character feature of
    /// terminals so equipped. Use of character insert/delete is enabled by
    /// default. Calling idcok with TRUE as second argument re-enables use of
    /// character insertion and deletion.
    #[fixed_stack_segment]
    pub fn idcok (&self, b: bool) {
        unsafe { c::idcok(self.win, b); }
    }

    /// If idlok is called with TRUE as second argument, curses considers
    /// using the hardware insert/delete line feature of terminals so
    /// equipped. Calling idlok with FALSE as second argument disables use of
    /// line insertion and deletion. This option should be enabled only if the
    /// application needs insert/delete line, for example, for a screen
    /// editor. It is disabled by default because insert/delete line tends to
    /// be visually annoying when used in applications where it is not really
    /// needed. If insert/delete line cannot be used, curses redraws the
    /// changed portions of all lines.
    #[fixed_stack_segment]
    pub fn idlok (&self, b: bool) {
        unsafe { c::idlok(self.win, b); }
    }

    /// If immedok is called with TRUE as argument, any change in the window
    /// image, such as the ones caused by waddch, wclrtobot, wscrl, etc.,
    /// automatically cause a call to wrefresh. However, it may degrade
    /// performance considerably, due to repeated calls to wrefresh. It is
    /// disabled by default.
    #[fixed_stack_segment]
    pub fn immedok (&self, b: bool) {
        unsafe { c::immedok(self.win, b); }
    }

    /// If the intrflush option is enabled, (b is TRUE), when an interrupt
    /// key is pressed on the keyboard (interrupt, break, quit) all output in
    /// the tty driver queue will be flushed, giving the effect of faster
    /// response to the interrupt, but causing curses to have the wrong idea
    /// of what is on the screen. Disabling (b is FALSE), the option prevents
    /// the flush. The default for the option is inherited from the tty driver
    /// settings. The window argument is ignored.
    #[fixed_stack_segment]
    pub fn intrflush (&self, b: bool) -> i32 {
        unsafe { c::intrflush(self.win, b) }
    }

    /// is_linetouched() returns true if any of the specified lines have
    /// been touched since the last refresh operation, else return false.
    #[fixed_stack_segment]
    pub fn is_linetouched (&self, n: i32) -> bool {
        unsafe { c::is_linetouched(self.win, n as c_int) }
    }

    /// is_windtouched() returns true if any of the specified windows have
    /// been touched since the last refresh operation, else return false.
    #[fixed_stack_segment]
    pub fn is_wintouched (&self) -> bool {
        unsafe { c::is_wintouched(self.win) }
    }

    /// The keypad option enables the keypad of the user's terminal. If
    /// enabled (b is TRUE), the user can press a function key (such as an
    /// arrow key) and wgetch returns a single value representing the function
    /// key, as in KEY_LEFT. If disabled (bf is FALSE), curses does not treat
    /// function keys specially and the program has to interpret the escape
    /// sequences itself. If the keypad in the terminal can be turned on (made
    /// to transmit) and off (made to work locally), turning on this option
    /// causes the terminal keypad to be turned on when wgetch is called. The
    /// default value for keypad is false.
    #[fixed_stack_segment]
    pub fn keypad (&self, b: bool) -> i32 {
        unsafe { c::keypad(self.win, b) }
    }

    /// The leaveok option allows the cursor to be left wherever the update
    /// happens to leave it. It is useful for applications where the cursor is
    /// not used, since it reduces the need for cursor motions.
    #[fixed_stack_segment]
    pub fn leaveok (&self, b: bool) -> i32 {
        unsafe { c::leaveok(self.win, b) }
    }

    /// Initially, whether the terminal returns 7 or 8 significant bits on
    /// input depends on the control mode of the tty driver. To force 8 bits
    /// to be returned, invoke meta(win, TRUE); this is equivalent, under
    /// POSIX, to setting the CS8 flag on the terminal. To force 7 bits to be
    /// returned, invoke meta(win, FALSE); this is equivalent, under POSIX, to
    /// setting the CS8 flag on the terminal. The window argument, win, is
    /// always ignored. If the terminfo capabilities smm (meta_on) and rmm
    /// (meta_off) are defined for the terminal, smm is sent to the terminal
    /// when meta(win, TRUE) is called and rmm is sent when meta(win, FALSE)
    /// is called.
    #[fixed_stack_segment]
    pub fn meta (&self, b: bool) -> i32 {
        unsafe {c::meta(self.win, b)}
    }

    /// Calling mvderwin moves a derived window (or subwindow) inside its
    /// parent window. The screen-relative parameters of the window are not
    /// changed. This routine is used to display different parts of the parent
    /// window at the same physical position on the screen.    
    #[fixed_stack_segment]
    pub fn mvderwin (&self,  n1: i32, c2: i32) -> i32 {
        unsafe {
            c::mvderwin(self.win,  n1, c2)
        }
    }
    
    /// mvwaddch puts the character c into the window at its current
    /// window position, which is then advanced.   
    #[fixed_stack_segment]
    pub fn mvwaddch (&self,  y: i32, x: i32, c: char) -> i32 {
        unsafe{
            c::mvwaddch (self.win, y, x, c as u32)
        }
    }

    // /// move to (y, x) and add up to n chars from the string `s`
    // #[fixed_stack_segment]
    // pub fn mvwaddchnstr (&self, y: i32, x: i32, s: ~str, n: i32) -> i32 {
    //     unsafe {            
    //         // mvwaddchnstr (win: *t::WINDOW, n1: c_int, c2: c_int, ch3: *t::chtype, n4: c_int) -> c_int; 
    //         //c::mvwaddchnstr(self.win, y, x, s.to_c_str().unwrap() as *u32, n)
    //         let cstr = s.to_c_str().unwrap();
    //         error!("%S", cstr);
    //         return c::mvwaddchnstr(self.win, y, x, cstr, n);            

    //         // fn with_ref<T>(&self, f: &fn(*c_char) -> T) -> T
    //         s.slice(0, n)
    //     }
    // }


    // Not working correctly, only shows the first couple characters,
    // then garbage.
    #[fixed_stack_segment]
    pub fn mvwaddchstr (&self,  y: i32, x: i32, s: ~str) -> i32 {
        unsafe{
            let cs = s.to_c_str().unwrap();
            c::mvwaddchstr(self.win, y, x, cs)
        }
    }
    #[fixed_stack_segment]
    pub fn mvwaddnstr (&self,  n1: i32, c2: i32, c3: *char, n4: i32) -> i32 {
        unsafe{
            c::mvwaddnstr (self.win, n1, c2, c3, n4)
        }
    }

    #[fixed_stack_segment]
    pub fn mvwaddstr (&self,  n1: i32, c2: i32, c3: *char) -> i32 {
        unsafe{
            c::mvwaddstr(self.win,  n1, c2, c3)
        }
    }

    // NEED TO FIX THIS ONE 
    // //#[fixed_stack_segment]
    // // args clipped pub fn mvwchgat (&self,  n1: i32, c2: i32, c3: i32, at4: t::attr_t, s: c_short, v {
    // unsafe{
    // c::mvwchgat (self.win,  n1: i32, c2: i32, c3: i32, at4: t::attr_t, s: c_short, v {
    // }
    // //}

    #[fixed_stack_segment]
    pub fn mvwdelch (&self,  n1: i32, c2: i32) -> i32 {
        unsafe {
            c::mvwdelch(self.win, n1, c2)
        }
    }

    #[fixed_stack_segment]
    pub fn mvwgetch (&self, n1: i32, c2: i32) -> i32 {
        unsafe{
            c::mvwgetch(self.win, n1, c2)
        }
    }

    /// mvwgetnstr, moves to y, x and reads at most n characters, thus
    /// preventing a possible over flow of the input buffer. Any attempt to
    /// enter more characters (other than the terminating newline or carriage
    /// return) causes a beep. Function keys also cause a beep and are
    /// ignored.
    #[fixed_stack_segment]
    pub fn mvwgetnstr (&self,  n1: i32, c2: i32, c3: *char, n4: i32) -> i32 {
        unsafe{
            c::mvwgetnstr(self.win, n1, c2, c3, n4)
        }
    }

    /// input a single-byte character and rendition from a window 
    #[fixed_stack_segment]
    pub fn mvwinch (&self, y: i32, x: i32) -> Option<char> {
        unsafe {
            let ch = c::mvwinch(self.win, y, x);
            from_u32(ch)
        }
    }

    #[fixed_stack_segment]
    pub fn getmaxx (&self) -> i32 {
        unsafe {
            c::getmaxx(self.win) as i32
        }
    }

    #[fixed_stack_segment]
    pub fn getmaxy (&self) -> i32 {
        unsafe {
            c::getmaxy(self.win) as i32
        }
    }

    #[fixed_stack_segment]
    pub fn mvwgetstr (&self, n1: i32, c2: i32, c3: *char) -> i32 {
        unsafe {
            c::mvwgetstr(self.win, n1, c2, c3)
        }
    }
    

    // Same as whline except that wmove() is called to move the cursor
    // to the position specified by y, x before the line is drawn on the
    // window.
    #[fixed_stack_segment]
    pub fn mvwhline (&self, n1: i32, c2: i32, ch3: t::chtype, n4: i32) -> i32 {
        unsafe {
            c::mvwhline(self.win, n1, c2, ch3, n4)
        }
    }

    /// Calling mvwin moves the window so that the upper left-hand corner is
    /// at position (x, y). If the move would cause the window to be off the
    /// screen, it is an error and the window is not moved. Moving subwindows
    /// is allowed, but should be avoided.
    #[fixed_stack_segment]
    pub fn mvwin (&self, y: i32, x: i32) -> i32 {
        unsafe {
            c::mvwin(self.win, y, x)
        }
    }

    #[fixed_stack_segment]
    pub fn mvwinsch (&self, n1: i32, c2: i32, ch3: t::chtype) -> i32 {
        unsafe {
            c::mvwinsch(self.win, n1, c2, ch3)
        }
    }

    /// These routines insert a character string, as many characters as
    /// will fit on the line, before the character under the cursor.All
    /// characters to the right of the cursor are shifted right with the
    /// possibility of the rightmost characters on the line being lost.
    /// The cursor position does not change (after moving to y, x, if
    /// specified).The functions with nas the last argument insert a
    /// leading substring of at most n characters.  If n<=0, then the
    /// entire string is inserted.

    #[fixed_stack_segment]
    pub fn mvwinchstr (&self, n1: i32, c2: i32, ch3: *t::chtype) -> i32 {
        unsafe {
            c::mvwinchstr(self.win, n1, c2, ch3)
        }
    }

    #[fixed_stack_segment]
    pub fn mvwinnstr (&self, n1: i32, c2: i32, c3: *char, n4: i32) -> i32 {
        unsafe {
            c::mvwinnstr(self.win, n1, c2, c3, n4)
        }
    }
    
    #[fixed_stack_segment]
    pub fn mvwinchnstr (&self, c1: i32, c2: i32,  ch3: *t::chtype, n4: i32) -> i32 {
        unsafe {
            c::mvwinchnstr(self.win, c1, c2, ch3, n4)
        }
    }

    #[fixed_stack_segment]
    pub fn mvwinsnstr (&self, n1: i32, c2: i32, c3: *char, n4: i32) -> i32 {
        unsafe {
            c::mvwinsnstr(self.win, n1, c2, c3, n4)
        }
    }


    #[fixed_stack_segment]
    pub fn mvwinsstr (&self, n1: i32, c2: i32, c3: *char) -> i32 {
        unsafe {
            c::mvwinsstr(self.win, n1, c2, c3)
        }
    }

    #[fixed_stack_segment]
    pub fn mvwinstr (&self, n1: i32, c2: i32, c3: *char) -> i32 {
        unsafe {
            c::mvwinstr(self.win, n1, c2, c3)
        }
    }

}



/*
        //pub fn mvwprc_intw (win0: *t::WINDOW, n1: c_int, c2: c_int, c3: *char四...); 
        pub fn overlay (win0: *t::WINDOW, win: *t::WINDOW) -> c_int; 
        pub fn overwrite (win0: *t::WINDOW, win: *t::WINDOW) -> c_int; 
        pub fn pechochar (win: *t::WINDOW, c1: t::chtype) -> c_int; 
        pub fn pnoutrefresh (win0: *t::WINDOW, n1: c_int, c2: c_int, c3: c_int, 
        pub fn prefresh (win: *t::WINDOW, n1: c_int, c2: c_int, c3: c_int, 
        // pub fn extern NCURSES_EXPORT(n0: c_int) putwin (win: *t::WINDOW一*FILE) -> c_int; 
        pub fn redrawwin (win: *t::WINDOW) -> c_int; 
        //pub fn ripoffline (n0: c_int, n1: c_int (*)(win: *t::WINDOW, c2: c_int)) -> c_int;  todo functio        pub fn scroll (win: *t::WINDOW) -> c_int; 
        pub fn scrollok (win: *t::WINDOW, b1: bool) -> c_int; 
        pub fn subpad (win: *t::WINDOW, n1: c_int, c2: c_int, c3: c_int, n4: c_int) -> *t::WINDOW; 
        pub fn subwin (win: *t::WINDOW, n1: c_int, c2: c_int, c3: c_int, n4: c_int) -> *t::WINDOW; 
        pub fn syncok (win: *t::WINDOW, b1: bool) -> c_int; 
        pub fn touchline (win: *t::WINDOW, n1: c_int, c2: c_int) -> c_int; 
        pub fn touchwin (win: *t::WINDOW) -> c_int; 
        pub fn untouchwin (win: *t::WINDOW) -> c_int; 
        pub fn vwprintw (win: *t::WINDOW, c1: *char, va2: t::va_list) -> c_int; 
        pub fn vw_printw (win: *t::WINDOW, c1: *char, va2: t::va_list) -> c_int; 
        pub fn vwscanw (win: *t::WINDOW, c1: *char, va2: t::va_list) -> c_int; 
        pub fn vw_scanw (win: *t::WINDOW, c1: *char, va2: t::va_list) -> c_int; 
        pub fn waddch (win: *t::WINDOW, c1: t::chtype) -> c_int; 
        pub fn waddchnstr (win: *t::WINDOW, ch1: *t::chtype, c2: c_int) -> c_int; 
        pub fn waddchstr (win: *t::WINDOW, ch1: *t::chtype) -> c_int; 
        pub fn waddnstr (win: *t::WINDOW, c1: *char, c2: c_int) -> c_int; 
        pub fn waddstr (win: *t::WINDOW, c1: *char) -> c_int; 
        pub fn wattron (win: *t::WINDOW, n1: c_int) -> c_int; 
        pub fn wattroff (win: *t::WINDOW, n1: c_int) -> c_int; 
        pub fn wattrset (win: *t::WINDOW, n1: c_int) -> c_int; 
        pub fn wattr_get (win: *t::WINDOW, at1: *t::attr_t, s2: *c_short, v3: *c_void) -> c_int; 
        pub fn wattr_on (win: *t::WINDOW, at2: t::attr_t, v2: *c_void) -> c_int; 
        pub fn wattr_off (win: *t::WINDOW, at2: t::attr_t, v2: *c_void) -> c_int; 
        pub fn wattr_set (win: *t::WINDOW, at2: t::attr_t, s2: c_short, v3: *c_void) -> c_int; 
        pub fn wbkgd (win: *t::WINDOW, c1: t::chtype) -> c_int; 
        pub fn wbkgdset (win: *t::WINDOW, c1: t::chtype) -> c_void; 
        pub fn wborder (win: *t::WINDOW, c1: t::chtype, c2: t::chtype, ch3: t::chtype, ch4: t::chtype,
        pub fn wchgat (win: *t::WINDOW, n1: c_int, at: t::attr_t, s3: c_short, v: *c_void) -> c_int; 
        pub fn wclear (win: *t::WINDOW) -> c_int; 
        pub fn wclrtobot (win: *t::WINDOW) -> c_int; 
        pub fn wclrtoeol (win: *t::WINDOW) -> c_int; 
        pub fn wcolor_set (win0: *t::WINDOW, s1: c_short, v2: *c_void) -> c_int; 
        pub fn wcursyncup (win: *t::WINDOW) -> c_void; 
        pub fn wdelch (win: *t::WINDOW) -> c_int; 
        pub fn wdeleteln (win: *t::WINDOW) -> c_int; 
        pub fn wechochar (win: *t::WINDOW, c1: t::chtype) -> c_int; 
        pub fn werase (win: *t::WINDOW) -> c_int; 
        pub fn wgetch (win: *t::WINDOW) -> c_int; 
        pub fn wgetnstr (win: *t::WINDOW, c1: *char, c2: c_int) -> c_int; 
        pub fn wgetstr (win: *t::WINDOW, c1: *char) -> c_int; 
        pub fn whline (win: *t::WINDOW, c1: t::chtype, c2: c_int) -> c_int; 
        pub fn winch (win: *t::WINDOW) -> t::chtype; 
        pub fn winchnstr (win: *t::WINDOW, ch1: *t::chtype, c2: c_int) -> c_int; 
        pub fn winchstr (win: *t::WINDOW, ch1: *t::chtype) -> c_int; 
        pub fn winnstr (win: *t::WINDOW, c1: *char, c2: c_int) -> c_int; 
        pub fn winsch (win: *t::WINDOW, c1: t::chtype) -> c_int; 
        pub fn winsdelln (win: *t::WINDOW, n1: c_int) -> c_int; 
        pub fn winsertln (win: *t::WINDOW) -> c_int; 
        pub fn winsnstr (win: *t::WINDOW, c1: *char, c2: c_int) -> c_int; 
        pub fn winsstr (win: *t::WINDOW, c1: *char) -> c_int; 
        pub fn winstr (win: *t::WINDOW, c1: *char) -> c_int; 
        pub fn wmove (win: *t::WINDOW, n1: c_int, c2: c_int) -> c_int; 
        pub fn wnoutrefresh (win: *t::WINDOW) -> c_int; 
        //pub fn wprc_intw (win: *t::WINDOW, c1: *char二...); 
        pub fn wredrawln (win: *t::WINDOW, n1: c_int, c2: c_int) -> c_int; 
        pub fn wrefresh (win: *t::WINDOW) -> c_int; 
        //pub fn wscanw (win: *t::WINDOW, c1: *char二...); 
        pub fn wscrl (win: *t::WINDOW, n1: c_int) -> c_int; 
        pub fn wsetscrreg (win: *t::WINDOW, n1: c_int, c2: c_int) -> c_int; 
        pub fn wstandout (win: *t::WINDOW) -> c_int; 
        pub fn wstandend (win: *t::WINDOW) -> c_int; 
        pub fn wsyncdown (win: *t::WINDOW) -> c_void; 
        pub fn wsyncup (win: *t::WINDOW) -> c_void; 
        pub fn wtimeout (win: *t::WINDOW, n1: c_int) -> c_void; 
        pub fn wtouchln (win: *t::WINDOW, n1: c_int, c2: c_int, c3: c_int) -> c_int; 
        pub fn wvline (win: *t::WINDOW, c1: t::chtype, c2: c_int) -> c_int; 
        pub fn getattrs (win: *t::WINDOW) -> c_int; 
        pub fn getcurx (win: *t::WINDOW) -> c_int; 
        pub fn getcury (win: *t::WINDOW) -> c_int; 
        pub fn getbegx (win: *t::WINDOW) -> c_int; 
        pub fn getbegy (win: *t::WINDOW) -> c_int; 
        pub fn getparx (win: *t::WINDOW) -> c_int; 
        pub fn getpary (win: *t::WINDOW) -> c_int; 
        pub fn use_window (win: *t::WINDOW, c1: t::WINDOW_CB, v2: *c_void) -> c_int; 
        pub fn wresize (win: *t::WINDOW, n1: c_int, c2: c_int) -> c_int; 
        pub fn wgetparent (win: *t::WINDOW) -> *t::WINDOW; 
*/






/*


    /// "Set the 'background' set of attributes to attr. This set is
    /// initially 0 (no attributes)."]
    fn attrset(self, attr: u32) {
        unsafe {
            ncurses::attrset(attr as c_int);
        }
    }
}
*/
