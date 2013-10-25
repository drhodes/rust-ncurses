// Copyright 2012 Derek A. Rhodes.  All rights reserved.
// Use of this source code is governed by the ncurses
// license that can be found in the LICENSE file.

use std::libc::types::os::arch::c95::{ c_char, c_int, c_short, c_long};
use std::libc::types::common::c95::{ c_void, FILE};

use nc = ncurses;
use c = ncurses::c;
use t = types;

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
pub fn initscr() -> Result<Window, ~str> {
    unsafe {
        match c::initscr() {
            t::ERR => Err(~"initscr failed to initialize"),
            p => Ok(Window{win: p as *t::WINDOW}),
        }
    }
}

/// Calling newwin creates and returns a pointer to a new window with the
/// given number of lines and columns. The upper left-hand corner of the
/// window is at line begin_y, column begin_x. If either nlines or ncols
/// is zero, they default to LINES - begin_y and COLS - begin_x. A new
/// full-screen window is created by calling newwin(0,0,0,0).
#[fixed_stack_segment]
pub fn newwin(nlines: int, ncols: int, 
              begin_y: int, begin_x: int) -> Result<Window, ~str> {
    unsafe {
        match c::initscr() {
            t::ERR => Err(~"initscr failed to initialize"),
            p => Ok(Window{win: p as *t::WINDOW}),
        }
    }
}

impl Window {       
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

    #[fixed_stack_segment]
    pub fn is_nodelay (&self) -> bool {
        unsafe {
            c::is_nodelay(self.win)
        }
    }

    #[fixed_stack_segment]
    pub fn is_notimeout (&self) -> bool {
        unsafe {
            c::is_notimeout(self.win)                
        }
    }

    #[fixed_stack_segment]
    pub fn is_scrollok(&self) -> bool {        
        unsafe { 
            c::is_scrollok(self.win) 
        }
    }

    #[fixed_stack_segment]
    pub fn is_cleared (&self) -> bool { 
        unsafe {
            c::is_cleared(self.win)
        } 
    }

    #[fixed_stack_segment]
    pub fn is_idcok (&self) -> bool { 
        unsafe {
            c::is_idcok(self.win)
        } 
    }

    #[fixed_stack_segment]
    pub fn is_idlok (&self) -> bool { 
        unsafe {
            c::is_idlok(self.win)
        } 
    }

    #[fixed_stack_segment]
    pub fn is_immedok (&self) -> bool { 
        unsafe {
            c::is_immedok(self.win)
        } 
    }

    #[fixed_stack_segment]
    pub fn is_keypad (&self) -> bool { 
        unsafe {
            c::is_keypad(self.win)
        } 
    }

    #[fixed_stack_segment]
    pub fn is_leaveok (&self) -> bool { 
        unsafe {
            c::is_leaveok(self.win)
        } 
    }

    #[fixed_stack_segment]
    pub fn is_syncok (&self) -> bool { 
        unsafe {
            c::is_syncok(self.win)
        } 
    }


    // returns the top and bottom rows for the scrolling margin as set in wsetscrreg.
    #[fixed_stack_segment]
    pub fn wgetscrreg (&self) -> (int, int) {
        unsafe {
            let mut top: c_int = 0;
            let mut bottom: c_int = 0;
            if c::wgetscrreg(self.win, &top, &bottom) != 0 {
                return (-1, -1); // need to look closer into traditional error handling
                // with ncurses
            }
            
            return (top as int, bottom as int);
        }
    }

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


    // #[fixed_stack_segment]
    // pub fn getbkgd (&self, win: *t::WINDOW) -> t::chtype {
    // }

    // #[fixed_stack_segment]
    // pub fn idcok (&self, win: *t::WINDOW, b1: bool) -> c_void {
    // }

    // #[fixed_stack_segment]
    // pub fn idlok (&self, win: *t::WINDOW, b1: bool) -> c_int {
    // }

    // #[fixed_stack_segment]
    // pub fn immedok (&self, win: *t::WINDOW, b1: bool) -> c_void {
    // }

    // #[fixed_stack_segment]
    // pub fn intrflush (&self, win: *t::WINDOW, b1: bool) -> c_int {
    // }

    // #[fixed_stack_segment]
    // pub fn is_linetouched (&self, win: *t::WINDOW, n1: c_int) -> bool {
    // }

    // #[fixed_stack_segment]
    // pub fn is_wintouched (&self, win: *t::WINDOW) -> bool {
    // }

    // #[fixed_stack_segment]
    // pub fn keypad (&self, win: *t::WINDOW, b1: bool) -> c_int {
    // }

    // #[fixed_stack_segment]
    // pub fn leaveok (&self, win: *t::WINDOW, b1: bool) -> c_int {
    // }

    // #[fixed_stack_segment]
    // pub fn meta (&self, win: *t::WINDOW, b1: bool) -> c_int {
    // }

    // #[fixed_stack_segment]
    // pub fn mvderwin (&self, win: *t::WINDOW, n1: c_int, c2: c_int) -> c_int {
    //     unsafe {
    //     }
    // }

    // #[fixed_stack_segment]
    // pub fn mvwaddch (&self, win: *t::WINDOW, n1: c_int, c2: c_int, ch3: t::chtype) -> c_int {
    // }

    // #[fixed_stack_segment]
    // pub fn mvwaddchnstr (&self, win: *t::WINDOW, n1: c_int, c2: c_int, ch3: *t::chtype, n4: c_int) -> c_int {
    // }

    // #[fixed_stack_segment]
    // pub fn mvwaddchstr (&self, win: *t::WINDOW, n1: c_int, c2: c_int, ch3: *t::chtype) -> c_int;    {
    // }

    // #[fixed_stack_segment]
    // pub fn mvwaddnstr (&self, win: *t::WINDOW, n1: c_int, c2: c_int, c3: *char, n4: c_int) -> c_int {
    // }

    // #[fixed_stack_segment]
    // pub fn mvwaddstr (&self, win: *t::WINDOW, n1: c_int, c2: c_int, c3: *char) -> c_int {
    // }

    // //#[fixed_stack_segment]
    // // args clipped pub fn mvwchgat (&self, win: *t::WINDOW, n1: c_int, c2: c_int, c3: c_int, at4: t::attr_t, s: c_short, v {
    // //}

    // #[fixed_stack_segment]
    // pub fn mvwdelch (&self, win: *t::WINDOW, n1: c_int, c2: c_int) -> c_int {
    // }
    // #[fixed_stack_segment]
    // pub fn mvwgetch (&self, win: *t::WINDOW, n1: c_int, c2: c_int) -> c_int {
    // }
    // #[fixed_stack_segment]
    // pub fn mvwgetnstr (&self, win: *t::WINDOW, n1: c_int, c2: c_int, c3: *char, n4: c_int) -> c_int {
    // }
}



/*
    pub fn mvwgetstr (win: *t::WINDOW, n1: c_int, c2: c_int, c3: *char) -> c_int; 
    pub fn mvwhline (win: *t::WINDOW, n1: c_int, c2: c_int, ch3: t::chtype, n4: c_int) -> c_int; 
    pub fn mvwin (win: *t::WINDOW, n1: c_int, c2: c_int) -> c_int; 
    pub fn mvwinch (win: *t::WINDOW, n1: c_int, c2: c_int) -> t::chtype; 
    pub fn mvwinchnstr (win: *t::WINDOW, n1: c_int, c2: c_int, ch3: *t::chtype, n4: c_int) -> c_int; 
    pub fn mvwinchstr (win: *t::WINDOW, n1: c_int, c2: c_int, ch3: *t::chtype) -> c_int; 
    pub fn mvwinnstr (win: *t::WINDOW, n1: c_int, c2: c_int, c3: *char, n4: c_int) -> c_int; 
    pub fn mvwinsch (win: *t::WINDOW, n1: c_int, c2: c_int, ch3: t::chtype) -> c_int; 
    pub fn mvwinsnstr (win: *t::WINDOW, n1: c_int, c2: c_int, c3: *char, n4: c_int) -> c_int; 
    pub fn mvwinsstr (win: *t::WINDOW, n1: c_int, c2: c_int, c3: *char) -> c_int; 
    pub fn mvwinstr (win: *t::WINDOW, n1: c_int, c2: c_int, c3: *char) -> c_int; 
        //pub fn mvwprc_intw (win0: *t::WINDOW, n1: c_int, c2: c_int, c3: *char四...); 
        pub fn notimeout (win: *t::WINDOW, b1: bool) -> c_int; 
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
        pub fn getmaxx (win: *t::WINDOW) -> c_int; 
        pub fn getmaxy (win: *t::WINDOW) -> c_int; 
        pub fn getparx (win: *t::WINDOW) -> c_int; 
        pub fn getpary (win: *t::WINDOW) -> c_int; 
        pub fn use_window (win: *t::WINDOW, c1: t::WINDOW_CB, v2: *c_void) -> c_int; 
        pub fn wresize (win: *t::WINDOW, n1: c_int, c2: c_int) -> c_int; 
        pub fn wgetparent (win: *t::WINDOW) -> *t::WINDOW; 
        pub fn is_cleared (win: *t::WINDOW) -> bool; 
        pub fn is_idcok (win: *t::WINDOW) -> bool; 
        pub fn is_idlok (win: *t::WINDOW) -> bool; 
        pub fn is_immedok (win: *t::WINDOW) -> bool; 
        pub fn is_keypad (win: *t::WINDOW) -> bool; 
        pub fn is_leaveok (win: *t::WINDOW) -> bool; 
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

// #[test]
// fn hello2() {
//     let win = NewWindow();
//     unsafe {
//         win.attrset(A_BOLD);
//         win.addstr(1, 1, ~"你好 Hello");
        
//         for n in range(2, 9) {
//             attrset(REVERSE);
//             win.addstr(n as int, (n as int)*2, ~" World !!!");
//             ncurses::refresh();/* Print it on to the real screen */
//         }
//         ncurses::getch();/* Wait for user input */
//         ncurses::endwin();/* End curses mode  */
//     }
// }



    // -----------------------------------------------------------------------------
    //         note
    //         A character means a C character (an ASCII code), rather than a
    //         Python character (a string of length 1). (This note is true
    //         whenever the documentation mentions a character.) The built-in
    //         ord() is handy for conveying strings to codes.

    // -----------------------------------------------------------------------------
    // #[doc = "Paint at most n characters of the string s at (y, x) with \
    //          attributes attrs, overwriting anything previously on the display"]

    // fn addnstr(y: int, x: int, s: str, attrs: [NCURSES_ATTR_T]) {        
    //     self.move(y, x);        

    //     for attrs.each {|a|
    //         ncurses::attron(a)
    //     }
    //     str::as_c_str(s, {|x| ncurses::addnstr(c: *char, n1: c_int)})
    // }


    // -----------------------------------------------------------------------------
    // window.bkgd(ch[, attr])
    //         Set the background property of the window to the character ch,
    //         with attributes attr. The change is then applied to every
    //         character position in that window:
    //            * The attribute of every character in the window is changed to
    //              the new background attribute.
    //            * Wherever the former background character appears, it is
    //              changed to the new background character.

    // -----------------------------------------------------------------------------
    // window.bkgdset(ch[, attr])
    // Set the window’s background. A window’s background consists of a
    // character and any combination of attributes. The attribute part of
    // the background is combined (OR’ed) with all non-blank characters
    // that are written into the window. Both the character and attribute
    // parts of the background are combined with the blank characters.
    // The background becomes a property of the character and moves with
    // the character through any scrolling and insert/delete
    // line/character operations.

    // -----------------------------------------------------------------------------
    // window.border([ls[, rs[, ts[, bs[, tl[, tr[, bl[, br]]]]]]]])
    //         Draw a border around the edges of the window. Each parameter
    //         specifies the character to use for a specific part of the border;
    //         see the table below for more details. The characters can be
    //         specified as integers or as one-character strings.
    //         Note
    //         A 0 value for any parameter will cause the default character to be
    //         used for that parameter. Keyword parameters can not be used. The
    //         defaults are listed in this table:
    //
    //         +----------------------------------------------------------------+
    //         |  Parameter  |       Description       |     Default value      |
    //         |-------------+-------------------------+------------------------|
    //         | ls          | Left side               | ACS_VLINE              |
    //         |-------------+-------------------------+------------------------|
    //         | rs          | Right side              | ACS_VLINE              |
    //         |-------------+-------------------------+------------------------|
    //         | ts          | Top                     | ACS_HLINE              |
    //         |-------------+-------------------------+------------------------|
    //         | bs          | Bottom                  | ACS_HLINE              |
    //         |-------------+-------------------------+------------------------|
    //         | tl          | Upper-left corner       | ACS_ULCORNER           |
    //         |-------------+-------------------------+------------------------|
    //         | tr          | Upper-right corner      | ACS_URCORNER           |
    //         |-------------+-------------------------+------------------------|
    //         | bl          | Bottom-left corner      | ACS_LLCORNER           |
    //         |-------------+-------------------------+------------------------|
    //         | br          | Bottom-right corner     | ACS_LRCORNER           |
    //         +----------------------------------------------------------------+




    // -----------------------------------------------------------------------------
    // window.box([vertch, horch])
    // Similar to border(), but both ls and rs are vertch and both ts and
    // bs are horch. The default corner characters are always used by
    // this function.

    // -----------------------------------------------------------------------------
    // window.chgat([y, x, ] [num,] attr)
    // Set the attributes of num characters at the current cursor
    // position, or at position (y, x) if supplied. If no value of num is
    // given or num = -1, the attribute will be set on all the characters
    // to the end of the line. This function does not move the cursor.
    // The changed line will be touched using the touchline() method so
    // that the contents will be redisplayed by the next window refresh.

    // -----------------------------------------------------------------------------
    // window.clear()
    // Like erase(), but also cause the whole window to be repainted upon
    // next call to refresh().

    // -----------------------------------------------------------------------------
    // window.clearok(yes)
    // If yes is 1, the next call to refresh() will clear the window
    // completely.

    // -----------------------------------------------------------------------------
    // window.clrtobot()
    // Erase from cursor to the end of the window: all lines below the
    // cursor are deleted, and then the equivalent of clrtoeol() is
    // performed.

    // -----------------------------------------------------------------------------
    // window.clrtoeol()
    // Erase from cursor to the end of the line.

    // -----------------------------------------------------------------------------
    // window.cursyncup()
    // Update the current cursor position of all the ancestors of the
    // window to reflect the current cursor position of the window.

    // -----------------------------------------------------------------------------
    // window.delch([y, x])
    // Delete any character at (y, x).

    // -----------------------------------------------------------------------------
    // window.deleteln()
    // Delete the line under the cursor. All following lines are moved up
    // by one line.

    // -----------------------------------------------------------------------------
    // window.derwin([nlines, ncols], begin_y, begin_x)
    // An abbreviation for 'derive window', derwin() is the same as
    // calling subwin(), except that begin_y and begin_x are relative to
    // the origin of the window, rather than relative to the entire
    // screen. Return a window object for the derived window.

    // -----------------------------------------------------------------------------
    // window.echochar(ch[, attr])
    // Add character ch with attribute attr, and immediately call
    // refresh() on the window.

    // -----------------------------------------------------------------------------
    // window.enclose(y, x)
    // Test whether the given pair of screen-relative character-cell
    // coordinates are enclosed by the given window, returning True or
    // False. It is useful for determining what subset of the screen
    // windows enclose the location of a mouse event.

    // -----------------------------------------------------------------------------
    // window.erase()
    // Clear the window.

    // -----------------------------------------------------------------------------
    // window.getbegyx()
    // Return a tuple (y, x) of co-ordinates of upper-left corner.

    // -----------------------------------------------------------------------------
    // window.getbkgd()
    // Return the given window’s current background character/attribute
    // pair.

    // -----------------------------------------------------------------------------
    // window.getch([y, x])
    // Get a character. Note that the integer returned does not have to
    // be in ASCII range: function keys, keypad keys and so on return
    // numbers higher than 256. In no-delay mode, -1 is returned if there
    // is no input, else getch() waits until a key is pressed.

    // -----------------------------------------------------------------------------
    // window.getkey([y, x])
    // Get a character, returning a string instead of an integer, as
    // getch() does. Function keys, keypad keys and so on return a
    // multibyte string containing the key name. In no-delay mode, an
    // exception is raised if there is no input.

    // -----------------------------------------------------------------------------
    // window.getmaxyx()
    // Return a tuple (y, x) of the height and width of the window.

    // -----------------------------------------------------------------------------
    // window.getparyx()
    // Return the beginning coordinates of this window relative to its
    // parent window into two integer variables y and x. Return -1, -1 if
    // this window has no parent.

    // -----------------------------------------------------------------------------
    // window.getstr([y, x])
    // Read a string from the user, with primitive line editing capacity.

    // -----------------------------------------------------------------------------
    // window.getyx()
    // Return a tuple (y, x) of current cursor position relative to the
    // window’s upper-left corner.

    // -----------------------------------------------------------------------------
    // window.hline([y, x], ch, n)
    // Display a horizontal line starting at (y, x) with length n
    // consisting of the character ch.

    // -----------------------------------------------------------------------------
    // window.idcok(flag)
    // If flag is False, curses no longer considers using the hardware
    // insert/delete character feature of the terminal; if flag is True,
    // use of character insertion and deletion is enabled. When curses is
    // first initialized, use of character insert/delete is enabled by
    // default.

    // -----------------------------------------------------------------------------
    // window.idlok(yes)
    // If called with yes equal to 1, curses will try and use hardware
    // line editing facilities. Otherwise, line insertion/deletion are
    // disabled.

    // -----------------------------------------------------------------------------
    // window.immedok(flag)
    // If flag is True, any change in the window image automatically
    // causes the window to be refreshed; you no longer have to call
    // refresh() yourself. However, it may degrade performance
    // considerably, due to repeated calls to wrefresh. This option is
    // disabled by default.

    // -----------------------------------------------------------------------------
    // window.inch([y, x])
    // Return the character at the given position in the window. The
    // bottom 8 bits are the character proper, and upper bits are the
    // attributes.

    // -----------------------------------------------------------------------------
    // window.insch([y, x], ch[, attr])
    // Paint character ch at (y, x) with attributes attr, moving the line
    // from position x right by one character.

    // -----------------------------------------------------------------------------
    // window.insdelln(nlines)
    // Insert nlines lines into the specified window above the current
    // line. The nlines bottom lines are lost. For negative nlines,
    // delete nlines lines starting with the one under the cursor, and
    // move the remaining lines up. The bottom nlines lines are cleared.
    // The current cursor position remains the same.

    // -----------------------------------------------------------------------------
    // window.insertln()
    // Insert a blank line under the cursor. All following lines are
    // moved down by one line.

    // -----------------------------------------------------------------------------
    // window.insnstr([y, x], str, n[, attr])
    // Insert a character string (as many characters as will fit on the
    // line) before the character under the cursor, up to n characters.
    // If n is zero or negative, the entire string is inserted. All
    // characters to the right of the cursor are shifted right, with the
    // rightmost characters on the line being lost. The cursor position
    // does not change (after moving to y, x, if specified).

    // -----------------------------------------------------------------------------
    // window.insstr([y, x], str[, attr])
    // Insert a character string (as many characters as will fit on the
    // line) before the character under the cursor. All characters to the
    // right of the cursor are shifted right, with the rightmost
    // characters on the line being lost. The cursor position does not
    // change (after moving to y, x, if specified).

    // -----------------------------------------------------------------------------
    // window.instr([y, x] [, n])
    // Return a string of characters, extracted from the window starting
    // at the current cursor position, or at y, x if specified.
    // Attributes are stripped from the characters. If n is specified,
    // instr() returns a string at most n characters long (exclusive of
    // the trailing NUL).

    // -----------------------------------------------------------------------------
    // window.is_linetouched(line)
    // Return True if the specified line was modified since the last call
    // to refresh(); otherwise return False. Raise a curses.error
    // exception if line is not valid for the given window.

    // -----------------------------------------------------------------------------
    // window.is_wintouched()
    // Return True if the specified window was modified since the last
    // call to refresh(); otherwise return False.

    // -----------------------------------------------------------------------------
    // window.keypad(yes)
    // If yes is 1, escape sequences generated by some keys (keypad,
    // function keys) will be interpreted by curses. If yes is 0, escape
    // sequences will be left as is in the input stream.

    // -----------------------------------------------------------------------------
    // window.leaveok(yes)
    // If yes is 1, cursor is left where it is on update, instead of
    // being at 'cursor position.' This reduces cursor movement where
    // possible. If possible the cursor will be made invisible.
    // If yes is 0, cursor will always be at 'cursor position' after an
    // update.

    // -----------------------------------------------------------------------------
    // window.mvderwin(y, x)
    // Move the window inside its parent window. The screen-relative
    // parameters of the window are not changed. This routine is used to
    // display different parts of the parent window at the same physical
    // position on the screen.

    // -----------------------------------------------------------------------------
    // window.mvwin(new_y, new_x)
    // Move the window so its upper-left corner is at (new_y, new_x).

    // -----------------------------------------------------------------------------
    // window.nodelay(yes)
    // If yes is 1, getch() will be non-blocking.

    // -----------------------------------------------------------------------------
    // window.notimeout(yes)
    // If yes is 1, escape sequences will not be timed out.
    // If yes is 0, after a few milliseconds, an escape sequence will not
    // be interpreted, and will be left in the input stream as is.

    // -----------------------------------------------------------------------------
    // window.noutrefresh()
    // Mark for refresh but wait. This function updates the data
    // structure representing the desired state of the window, but does
    // not force an update of the physical screen. To accomplish that,
    // call doupdate().

    // -----------------------------------------------------------------------------
    // window.overlay(destwin[, sminrow, smincol, dminrow, dmincol, dmaxrow,
    // dmaxcol])

    // Overlay the window on top of destwin. The windows need not be the
    // same size, only the overlapping region is copied. This copy is
    // non-destructive, which means that the current background character
    // does not overwrite the old contents of destwin.

    // To get fine-grained control over the copied region, the second
    // form of overlay() can be used. sminrow and smincol are the
    // upper-left coordinates of the source window, and the other
    // variables mark a rectangle in the destination window.

    // -----------------------------------------------------------------------------
    // window.overwrite(destwin[, sminrow, smincol, dminrow, dmincol, dmaxrow,
    // dmaxcol])

    // Overwrite the window on top of destwin. The windows need not be
    // the same size, in which case only the overlapping region is
    // copied. This copy is destructive, which means that the current
    // background character overwrites the old contents of destwin.

    // To get fine-grained control over the copied region, the second
    // form of overwrite() can be used. sminrow and smincol are the
    // upper-left coordinates of the source window, the other variables
    // mark a rectangle in the destination window.

    // -----------------------------------------------------------------------------
    // window.putwin(file)
    // Write all data associated with the window into the provided file
    // object. This information can be later retrieved using the getwin()
    // function.

    // -----------------------------------------------------------------------------
    // window.redrawln(beg, num)
    // Indicate that the num screen lines, starting at line beg, are
    // corrupted and should be completely redrawn on the next refresh()
    // call.

    // -----------------------------------------------------------------------------
    // window.redrawwin()
    // Touch the entire window, causing it to be completely redrawn on
    // the next refresh() call.

    // -----------------------------------------------------------------------------
    // window.refresh([pminrow, pmincol, sminrow, smincol, smaxrow, smaxcol])
    // Update the display immediately (sync actual screen with previous
    // drawing/deleting methods).
    // The 6 optional arguments can only be specified when the window is
    // a pad created with newpad(). The additional parameters are needed
    // to indicate what part of the pad and screen are involved. pminrow
    // and pmincol specify the upper left-hand corner of the rectangle to
    // be displayed in the pad. sminrow, smincol, smaxrow, and smaxcol
    // specify the edges of the rectangle to be displayed on the screen.
    // The lower right-hand corner of the rectangle to be displayed in
    // the pad is calculated from the screen coordinates, since the
    // rectangles must be the same size. Both rectangles must be entirely
    // contained within their respective structures. Negative values of
    // pminrow, pmincol, sminrow, or smincol are treated as if they were
    // zero.

    // -----------------------------------------------------------------------------
    // window.resize(nlines, ncols)
    // Reallocate storage for a curses window to adjust its dimensions to
    // the specified values. If either dimension is larger than the
    // current values, the window’s data is filled with blanks that have
    // the current background rendition (as set by bkgdset()) merged into
    // them.

    // -----------------------------------------------------------------------------
    // window.scroll([lines=1])
    // Scroll the screen or scrolling region upward by lines lines.

    // -----------------------------------------------------------------------------
    // window.scrollok(flag)
    // Control what happens when the cursor of a window is moved off the
    // edge of the window or scrolling region, either as a result of a
    // newline action on the bottom line, or typing the last character of
    // the last line. If flag is false, the cursor is left on the bottom
    // line. If flag is true, the window is scrolled up one line. Note
    // that in order to get the physical scrolling effect on the
    // terminal, it is also necessary to call idlok().

    // -----------------------------------------------------------------------------
    // window.setscrreg(top, bottom)
    // Set the scrolling region from line top to line bottom. All
    // scrolling actions will take place in this region.

    // -----------------------------------------------------------------------------
    // window.standend()
    // Turn off the standout attribute. On some terminals this has the
    // side effect of turning off all attributes.

    // -----------------------------------------------------------------------------
    // window.standout()
    // Turn on attribute A_STANDOUT.

    // -----------------------------------------------------------------------------
    // window.subpad([nlines, ncols], begin_y, begin_x)
    // Return a sub-window, whose upper-left corner is at (begin_y,
    // begin_x), and whose width/height is ncols/nlines.

    // -----------------------------------------------------------------------------
    // window.subwin([nlines, ncols], begin_y, begin_x)
    // Return a sub-window, whose upper-left corner is at (begin_y,
    // begin_x), and whose width/height is ncols/nlines.
    // By default, the sub-window will extend from the specified position
    // to the lower right corner of the window.

    // -----------------------------------------------------------------------------
    // window.syncdown()
    // Touch each location in the window that has been touched in any of
    // its ancestor windows. This routine is called by refresh(), so it
    // should almost never be necessary to call it manually.

    // -----------------------------------------------------------------------------
    // window.syncok(flag)
    // If called with flag set to True, then syncup() is called
    // automatically whenever there is a change in the window.

    // -----------------------------------------------------------------------------
    // window.syncup()
    // Touch all locations in ancestors of the window that have been
    // changed in the window.

    // -----------------------------------------------------------------------------
    // window.timeout(delay)
    // Set blocking or non-blocking read behavior for the window. If
    // delay is negative, blocking read is used (which will wait
    // indefinitely for input). If delay is zero, then non-blocking read
    // is used, and -1 will be returned by getch() if no input is
    // waiting. If delay is positive, then getch() will block for delay
    // milliseconds, and return -1 if there is still no input at the end
    // of that time.

    // -----------------------------------------------------------------------------
    // window.touchline(start, count[, changed])
    // Pretend count lines have been changed, starting with line start.
    // If changed is supplied, it specifies whether the affected lines
    // are marked as having been changed (changed=1) or unchanged
    // (changed=0).

    // -----------------------------------------------------------------------------
    // window.touchwin()
    // Pretend the whole window has been changed, for purposes of drawing
    // optimizations.

    // -----------------------------------------------------------------------------
    // window.untouchwin()
    // Mark all lines in the window as unchanged since the last call to
    // refresh().
    
    // -----------------------------------------------------------------------------
    // window.vline([y, x], ch, n)
    // Display a vertical line starting at (y, x) with length n
    // consisting of the character ch.

