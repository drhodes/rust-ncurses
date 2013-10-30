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
    /// screenShot dumps a verbatim copy of the window to a file, with the
    /// exception of Newlines appended to the end of rows for human
    /// readability
    pub fn screenShot(&self, filename: ~str) {
        let bottom = self.getmaxy();        
        let right = self.getmaxx();
        
        let p = &Path::new(filename);
        let mut of = io::file::open(p, io::Create, io::Write).unwrap();

        // when chtype is understood better 
        // conform these to chars, not [u8]s
        let mut bs: ~[u8] = ~[];
        for row in range(0, bottom) {
            for col in range(0, right) {
                match self.mvwinch(row, col) {
                    Some(c) => {
                        bs.push(c as u8);
                    }                    
                    None => {
                        bs.push('?' as u8);
                    }
                }
            }
            bs.push('\n' as u8);           
        }
        of.write(bs);        
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


    /// Doesn't work, only shows the first couple characters, then garbage.
    /// almost like it's expecting 
    #[fixed_stack_segment]
    pub fn mvwaddchstr (&self,  y: i32, x: i32, s: ~str) -> i32 {
        unsafe{
            let cs = s.to_c_str().unwrap();
            c::mvwaddchstr(self.win, y, x, cs)
        }
    }

    
    #[fixed_stack_segment]
    pub fn mvwaddnstr (&self, n1: i32, c2: i32, s: ~str, n4: i32) -> i32 {
        unsafe{
            let cs = s.to_c_str().unwrap();
            c::mvwaddnstr (self.win, n1, c2, cs, n4)
        }
    }

    #[fixed_stack_segment]
    pub fn mvwaddstr (&self, y: i32, x: i32,  s: ~str) -> i32 {
        unsafe{
            let cs = s.to_c_str().unwrap();
            c::mvwaddstr(self.win, y, x, cs)
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
    pub fn mvwgetnstr (&self,  n1: i32, c2: i32, s: ~str, n4: i32) -> i32 {
        unsafe{
            let cs = s.to_c_str().unwrap();
            c::mvwgetnstr(self.win, n1, c2, cs, n4)
        }
    }

    /// insert a character into the window
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
    pub fn mvwgetstr (&self, n1: i32, c2: i32, s: ~str) -> i32 {
        unsafe {
            let cs = s.to_c_str().unwrap();
            c::mvwgetstr(self.win, n1, c2, cs)
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
    pub fn mvwinnstr (&self, n1: i32, c2: i32, s: ~str, n4: i32) -> i32 {
        unsafe {
            let cs = s.to_c_str().unwrap();           
            c::mvwinnstr(self.win, n1, c2, cs, n4)
        }
    }
    #[fixed_stack_segment]
    pub fn mvwinchnstr (&self, c1: i32, c2: i32,  ch3: *t::chtype, n4: i32) -> i32 {
        unsafe {
            c::mvwinchnstr(self.win, c1, c2, ch3, n4)
        }
    }
    #[fixed_stack_segment]
    pub fn mvwinsnstr (&self, n1: i32, c2: i32, c3: ~str, n4: i32) -> i32 {
        unsafe {
            let cs = c3.to_c_str().unwrap();
            c::mvwinsnstr(self.win, n1, c2, cs, n4)
        }
    }
    #[fixed_stack_segment]
    pub fn mvwinsstr (&self, y: i32, x: i32, s: ~str) -> i32 {
        unsafe {
            let cs = s.to_c_str().unwrap();
            c::mvwinsstr(self.win, y, x, cs as *c_char)
        }
    }
    #[fixed_stack_segment]
    pub fn mvwinstr (&self, y: i32, x: i32, s: ~str) -> i32 {
        unsafe {
            let cs = s.to_c_str().unwrap();
            c::mvwinstr(self.win, y, x , cs as *c_char)
        }
    }

    #[fixed_stack_segment] 
    pub fn getcurx(&self) -> i32 {
        unsafe {
            c::getcurx(self.win)
        }
    }
    #[fixed_stack_segment] 
    pub fn getcury(&self) -> i32 {
        unsafe {
            c::getcury(self.win)
        }
    }
    #[fixed_stack_segment] 
    pub fn getbegx(&self) -> i32 {
        unsafe {
            c::getbegx(self.win)
        }
    }
    #[fixed_stack_segment] 
    pub fn getbegy(&self) -> i32 {
        unsafe {
            c::getbegy(self.win)
        }
    }
    #[fixed_stack_segment] 
    pub fn getparx(&self) -> i32 {
        unsafe {
            c::getparx(self.win)
        }
    }
    #[fixed_stack_segment] 
    pub fn getpary(&self) -> i32 {
        unsafe {
            c::getpary(self.win)
        }
    }
    #[fixed_stack_segment] 
    pub fn wstandout(&self) -> i32 {
        unsafe {
            c::wstandout(self.win)
        }
    }
    #[fixed_stack_segment] 
    pub fn wstandend(&self) -> i32 {
        unsafe {
            c::wstandend(self.win)
        }
    }
    #[fixed_stack_segment] 
    pub fn wclear(&self) -> i32 {
        unsafe {
            c::wclear(self.win)
        }
    }
    #[fixed_stack_segment] 
    pub fn wclrtobot(&self) -> i32 {
        unsafe {
            c::wclrtobot(self.win)
        }
    }
    #[fixed_stack_segment] 
    pub fn wclrtoeol(&self) -> i32 {
        unsafe {
            c::wclrtoeol(self.win)
        }
    }
    #[fixed_stack_segment] 
    pub fn wdelch(&self) -> i32 {
        unsafe {
            c::wdelch(self.win)
        }
    }
    #[fixed_stack_segment] 
    pub fn wdeleteln(&self) -> i32 {
        unsafe {
            c::wdeleteln(self.win)
        }
    }
    #[fixed_stack_segment] 
    pub fn wrefresh(&self) -> i32 {
        unsafe {
            c::wrefresh(self.win)
        }
    }
    #[fixed_stack_segment] 
    pub fn redrawwin(&self) -> i32 {
        unsafe {
            c::redrawwin(self.win)
        }
    }
    #[fixed_stack_segment] 
    pub fn touchwin(&self) -> i32 {
        unsafe {
            c::touchwin(self.win)
        }
    }
    #[fixed_stack_segment] 
    pub fn untouchwin(&self) -> i32 {
        unsafe {
            c::untouchwin(self.win)
        }
    }
    #[fixed_stack_segment] 
    pub fn werase(&self) -> i32 {
        unsafe {
            c::werase(self.win)
        }
    }
    #[fixed_stack_segment] 
    pub fn wgetch(&self) -> i32 {
        unsafe {
            c::wgetch(self.win)
        }
    }
    #[fixed_stack_segment] 
    pub fn winsertln(&self) -> i32 {
        unsafe {
            c::winsertln(self.win)
        }
    }
    #[fixed_stack_segment] 
    pub fn wnoutrefresh(&self) -> i32 {
        unsafe {
            c::wnoutrefresh(self.win)
        }
    }

    #[fixed_stack_segment]
    pub fn overlay(&self, other: Window) -> i32 {
        unsafe {
            c::overlay(self.win, other.win)
        }
    }
    #[fixed_stack_segment]
    pub fn overwrite(&self, other: Window) -> i32 {
        unsafe {
            c::overwrite(self.win, other.win)
        }
    }

    // [u8]?
    // #[fixed_stack_segment]
    // pub fn wcolor_set(&self, s1: c_short, v2: *c_void) -> i32 {
    //     unsafe {
    //         c::wcolor_set(self.win, s1, )
    //     }
    // }

    #[fixed_stack_segment]
    pub fn pnoutrefresh(&self, n1: i32, c2: i32, c3: i32, c4: i32, c5: i32, c6: i32) -> i32 {
        unsafe {
            c::pnoutrefresh(self.win, n1, c2, c3, c4, c5, c6)
        }
    }


    #[fixed_stack_segment]
    pub fn pechochar(&self, c1: t::chtype) -> i32 {
        unsafe {
            c::pechochar(self.win, c1)
        }
    }

    #[fixed_stack_segment]
    pub fn prefresh(&self, n1: i32, c2: i32, c3: i32, c4: i32, c5: i32, c6: i32) -> i32 {
        unsafe {
            c::prefresh(self.win, n1, c2, c3, c4, c5, c6)
        }
    }

    #[fixed_stack_segment]
    pub fn scrollok(&self, b1: bool) -> i32 {
        unsafe {
            c::scrollok(self.win, b1)
        }
    }

    #[fixed_stack_segment]
    pub fn subpad(&self, n1: i32, c2: i32, c3: i32, n4: i32) -> Window {
        unsafe {
            Window{win: c::subpad(self.win, n1, c2, c3, n4)}
        }
    }

    #[fixed_stack_segment]
    pub fn subwin(&self, n1: i32, c2: i32, c3: i32, n4: i32) -> Window {
        unsafe {
            Window{win: c::subwin(self.win, n1,c2,c3,n4)}
        }
    }

    #[fixed_stack_segment]
    pub fn syncok(&self, b1: bool) -> i32 {
        unsafe {
            c::syncok(self.win, b1)
        }
    }

    #[fixed_stack_segment]
    pub fn touchline(&self, n1: i32, c2: i32) -> i32 {
        unsafe {
            c::touchline(self.win, n1, c2)
        }
    }

    // VARARGS CONUNDRUM
    /*

    // #[fixed_stack_segment]
    // pub fn vwprintw(&self, c1: *char, va2: t::va_list) -> i32 {
    //     unsafe {
    //         c::vwprintw(self.win, c1, va2)
    //     }
    // }

    // #[fixed_stack_segment]
    // pub fn vw_printw(&self, s: ~str, va2: t::va_list) -> i32 {
    //     unsafe {
    //         let cs = s.to_c_str().unwrap();
    //         c::vw_printw(self.win, cs, va2)
    //     }
    // }

    #[fixed_stack_segment]
    pub fn vwscanw(&self, s: ~str, va2: t::va_list) -> i32 {
        unsafe {
            let cs = s.to_c_str().unwrap();

            match len


            c::vwscanw(self.win, cs, va2)
        }
    }

    #[fixed_stack_segment]
    pub fn vw_scanw(&self, c1: *char, va2: t::va_list) -> i32 {
    unsafe {
    c::vw_scanw(self.win)
    }
    }

    -- END VARARGS CONUNDRUM
     */

    /*
    ok this is the ncurses source after preprocessor

    int waddch(WINDOW *win, const chtype ch) {
      int code = (-1);
      chtype wch;
  
      wch = (((ch) & (chtype)(((1U) << ((0) + 8)) - 1U))) | (((ch) & (chtype)((~(1U - 1U)) << ((0) + 8))));
      if (win && (waddch_nosync(win, wch) != (-1))) {
        _nc_synchook(win);
        code = (0);
      }
      return (code);
    }
    */
    

    /// The addch, waddch, mvaddch and mvwaddch routines put the character ch
    /// into the given window at its current window position, which is then
    /// advanced. They are analogous to putchar in stdio(3). If the advance is
    /// at the right margin, the cursor automatically wraps to the beginning
    /// of the next line. At the bottom of the current scrolling region, if
    /// scrollok is enabled, the scrolling region is scrolled up one line.



    #[fixed_stack_segment]
    pub fn waddch(&self, c1: char) -> i32 {
        unsafe {
            // // fn encode_utf8(&self, dst: &mut [u8]) -> uint;
            // let mut dst = [0,0,0,0];
            // c1.encode_utf8(dst);
            // //error!("{} {} {} {}", dst[0], dst[1], dst[2], dst[3]);
            // let r0 = dst[3] << 24;
            // let r1 = dst[2] << 16;
            // let r2 = dst[1] << 8;
            // let r3 = dst[0];
            // let mut r = r0 | r1 | r2 | r3;
            // error!("{}", r);
            c::waddch(self.win, c1 as t::chtype)
        }
    }


    #[fixed_stack_segment]
    pub fn waddchnstr(&self, ch1: *t::chtype, c2: i32) -> i32 {
        unsafe {
            c::waddchnstr(self.win, ch1, c2)
        }
    }

    #[fixed_stack_segment]
    pub fn waddchstr(&self, ch1: *t::chtype) -> i32 {
        unsafe {
            c::waddchstr(self.win, ch1)
        }
    }

    #[fixed_stack_segment]
    pub fn waddnstr(&self, s: ~str, c2: i32) -> i32 {
        unsafe {
            let cs = s.to_c_str().unwrap();
            c::waddnstr(self.win, cs, c2)
        }
    }

    #[fixed_stack_segment]
    pub fn waddstr(&self, s: ~str) -> i32 {
        unsafe {
            let cs = s.to_c_str().unwrap();
            c::waddstr(self.win, cs)
        }
    }

    #[fixed_stack_segment]
    pub fn wattron(&self, n1: i32) -> i32 {
        unsafe {
            c::wattron(self.win, n1)
        }
    }
    #[fixed_stack_segment]
    pub fn wattroff(&self, n1: i32) -> i32 {
        unsafe {
            c::wattroff(self.win, n1)
        }
    }

    #[fixed_stack_segment]
    pub fn wattrset(&self, n1: i32) -> i32 {
        unsafe {
            c::wattrset(self.win, n1)
        }
    }

    // how to deal with void? &mut [u8] ?
    // #[fixed_stack_segment]
    // pub fn wattr_get(&self, at1: *t::attr_t, s2: *c_short, v3: *c_void) -> i32 {
    //     unsafe {
    //         c::wattr_get(self.win, at1, s2, v3)
    //     }
    // }
    // #[fixed_stack_segment]
    // pub fn wattr_on(&self, at2: t::attr_t, v2: *c_void) -> i32 {
    //     unsafe {
    //         c::wattr_on(self.win, at2)
    //     }
    // }
    // #[fixed_stack_segment]
    // pub fn wattr_off(&self, at2: t::attr_t, v2: *c_void) -> i32 {
    //     unsafe {
    //         c::wattr_off(self.win)
    //     }
    // }
    // #[fixed_stack_segment]
    // pub fn wattr_set(&self, at2: t::attr_t, s2: c_short, v3: *c_void) -> i32 {
    //     unsafe {
    //         c::wattr_set(self.win)
    //     }
    // }

    #[fixed_stack_segment]
    pub fn wbkgd(&self, c1: t::chtype) -> i32 {
        unsafe {
            c::wbkgd(self.win, c1)
        }
    }
    #[fixed_stack_segment]
    pub fn wbkgdset(&self, c1: t::chtype) -> c_void {
        unsafe {
            c::wbkgdset(self.win, c1)
        }
    }

    // #[fixed_stack_segment]
    // pub fn wchgat(&self, n1: i32, at: t::attr_t, s3: c_short, v: *c_void) -> i32 {
    //     unsafe {
    //         c::wchgat(self.win, n1, at, s4)
    //     }
    // }

    #[fixed_stack_segment]
    pub fn wechochar(&self, c1: t::chtype) -> i32 {
        unsafe {
            c::wechochar(self.win, c1)
        }
    }
    #[fixed_stack_segment]
    pub fn wgetnstr(&self, s: ~str, c2: i32) -> i32 {
        unsafe {
            let cs = s.to_c_str().unwrap();
            c::wgetnstr(self.win, cs, c2)
        }
    }
    #[fixed_stack_segment]
    pub fn wgetstr(&self, s: ~str) -> i32 {
        unsafe {
            let cs = s.to_c_str().unwrap();
            c::wgetstr(self.win, cs)
        }
    }
    #[fixed_stack_segment]
    pub fn whline(&self, c1: t::chtype, c2: i32) -> i32 {
        unsafe {
            c::whline(self.win, c1, c2)
        }
    }
    #[fixed_stack_segment]
    pub fn winchnstr(&self, ch1: *t::chtype, c2: i32) -> i32 {
        unsafe {
            c::winchnstr(self.win, ch1, c2)
        }
    }
    #[fixed_stack_segment]
    pub fn winchstr(&self, ch1: *t::chtype) -> i32 {
        unsafe {
            c::winchstr(self.win, ch1)
        }
    }

    #[fixed_stack_segment]
    pub fn winnstr(&self, s: ~str, c2: i32) -> i32 {
        unsafe {
            let cs = s.to_c_str().unwrap();
            c::winnstr(self.win, cs, c2)
        }
    }

    #[fixed_stack_segment]
    pub fn winsch(&self, ch: char) -> i32 {
        unsafe {
            c::winsch(self.win, ch as t::chtype)
        }
    }
    #[fixed_stack_segment]
    pub fn winsdelln(&self, n1: i32) -> i32 {
        unsafe {
            c::winsdelln(self.win, n1)
        }
    }
    #[fixed_stack_segment]
    pub fn winsnstr(&self, s: ~str, c2: i32) -> i32 {
        unsafe {
            let cs = s.to_c_str().unwrap();
            c::winsnstr(self.win, cs,c2)
        }
    }
    #[fixed_stack_segment]
    pub fn winsstr(&self, s: ~str) -> i32 {
        unsafe {
            let cs = s.to_c_str().unwrap();
            c::winsstr(self.win, cs)
        }
    }
    #[fixed_stack_segment]
    pub fn winstr(&self, s: ~str) -> i32 {
        unsafe {
            let cs = s.to_c_str().unwrap();
            c::winstr(self.win, cs)
        }
    }

    /// These routines move the cursor associated with the window to line
    /// y and column x.  This routine does not move the physical cursor of
    /// the terminal until refresh is called.  The position specified is
    /// relative to the upper left-hand corner of the window, which is
    /// (0,0).
    #[fixed_stack_segment]
    pub fn wmove(&self, n1: i32, c2: i32) -> i32 {
        unsafe {
            c::wmove(self.win, n1, c2)
        }
    }

    #[fixed_stack_segment]
    pub fn wredrawln(&self, n1: i32, c2: i32) -> i32 {
        unsafe {
            c::wredrawln(self.win, n1, c2)
        }
    }
    #[fixed_stack_segment]
    pub fn wscrl(&self, n1: i32) -> i32 {
        unsafe {
            c::wscrl(self.win, n1)
        }
    }
    #[fixed_stack_segment]
    pub fn wsetscrreg(&self, n1: i32, c2: i32) -> i32 {
        unsafe {
            c::wsetscrreg(self.win, n1, c2)
        }
    }
    #[fixed_stack_segment]
    pub fn wtimeout(&self, n1: i32) -> c_void {
        unsafe {
            c::wtimeout(self.win, n1)
        }
    }
    #[fixed_stack_segment]
    pub fn wtouchln(&self, n1: i32, c2: i32, c3: i32) -> i32 {
        unsafe {
            c::wtouchln(self.win, n1, c2, c3)
        }
    }
    #[fixed_stack_segment]
    pub fn wvline(&self, c1: t::chtype, c2: i32) -> i32 {
        unsafe {
            c::wvline(self.win, c1,c2)
        }
    }

    // #[fixed_stack_segment]
    // pub fn mvwinstr (&self, n1: i32, c2: i32, c3: *char) -> i32 {
    //     unsafe {
    //         c::mvwinstr(self.win, n1, c2, c3)
    //     }
    // }

    //pub fn winch() -> t::chtype; 

    #[fixed_stack_segment]
    pub fn wsyncup(&self) {
        unsafe { c::wsyncup(self.win); }
    }
    
    #[fixed_stack_segment]
    pub fn wsyncdown(&self) {
        unsafe { c::wsyncdown(self.win); }
    }

    #[fixed_stack_segment]
    pub fn wcursyncup(&self) {
        unsafe { c::wcursyncup(self.win); }
    }
    
    //pub fn wborder(c1: t::chtype, c2: t::chtype, ch3: t::chtype, ch4: t::chtype, ch5: t::chtype, ch6: t::chtype, ch7: t::chtype, ch8: t::chtype) -> i32;     
    // pub fn use_window(c1: t::WINDOW_CB, v2: *c_void) -> i32; 
    // pub fn wresize(n1: i32, c2: i32) -> i32; 
    

    #[fixed_stack_segment]
    pub fn wgetparent(&self) -> Window {
        unsafe { 
            Window{win: c::wgetparent(self.win)}
        }
    }

    // get curses cursor and window coordinates, attributes
    #[fixed_stack_segment]
    pub fn getattrs(&self) -> i32 {
        unsafe { 
            c::getattrs(self.win) 
        }
    }
}

//pub fn ripoffline (n0: c_int, n1: c_int (*)(win: *t::WINDOW, c2: c_int)) -> c_int;  
// todo functio        pub fn scroll (win: *t::WINDOW) -> c_int; 
//pub fn wprc_intw (win: *t::WINDOW, c1: *char二...); 
//pub fn wscanw (win: *t::WINDOW, c1: *char二...); 


//pub fn mvwprc_intw (win0: *t::WINDOW, n1: c_int, c2: c_int, c3: *char四...); 
// pub fn extern NCURSES_EXPORT(n0: c_int) putwin (win: *t::WINDOW一*FILE) -> c_int; 





