// Copyright 2012 Derek A. Rhodes.  All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.
// 

use std::libc::types::os::arch::c95::{ c_char, c_int, c_short, c_long};
use std::libc::types::common::c95::{ c_void, FILE};
use std::c_str::ToCStr;


use std::rt::io;
use std::str;
use t = types;

pub mod c {
    use std::libc::types::os::arch::c95::{ c_char, c_int, c_short, c_long};
    use std::libc::types::common::c95::{ c_void, FILE};
    use t = types;

    #[link_args = "-lncursesw"]
    extern {
        //extern NCURSES_EXPORT_VAR(t::chtype) acs_map[];  
        //fn wgetch_events (win: *t::WINDOW, nc: *_nc_eventlist) -> c_int; 
        //fn wgetnstr_events (win: *t::WINDOW, c: *char, n: c_int, nc: *_nc_eventlist); 
        pub fn addch (ch: t::chtype) -> c_int; 
        pub fn addchnstr (c: *t::chtype, n1: c_int) -> c_int; 
        pub fn addchstr (c: *t::chtype) -> c_int; 
        pub fn addnstr (c: *char, n1: c_int) -> c_int; 
        pub fn addstr (c: *c_char) -> c_int; 
        pub fn attroff (nc: t::NCURSES_ATTR_T) -> c_int; 
        pub fn attron (nc: t::NCURSES_ATTR_T) -> c_int; 
        pub fn attrset (nc: t::NCURSES_ATTR_T) -> c_int; 
        //pub fn attrset (nc: c_int) -> c_int; 
        pub fn attr_get (at1: *t::attr_t, s1: *c_short, v2: *c_void) -> c_int; 
        pub fn attr_off (at0: t::attr_t, v1: *c_void) -> c_int; 
        pub fn attr_on (at0: t::attr_t, v1: *c_void) -> c_int; 
        pub fn attr_set (at0: t::attr_t, s1: c_short, v2: *c_void) -> c_int; 
        pub fn baudrate () -> c_int; 
        pub fn beep  () -> c_int; 
        pub fn bkgd (ch0: t::ctype) -> c_int; 
        pub fn bkgdset (ch0: t::ctype) -> c_void; 
        pub fn border (c1: t::chtype, c2: t::chtype, c3: t::chtype, c4: t::chtype, 
                       c5: t::chtype, c6: t::chtype, c7: t::chtype, c8: t::chtype) -> c_int; 
        pub fn box (win: *t::WINDOW, c1: t::chtype, c2: t::chtype) -> c_int; 
        pub fn can_change_color () -> bool; 
        pub fn cbreak () -> c_int; 
        pub fn chgat (n: c_int, at2: t::attr_t, s2: c_short, v3: *c_void) -> c_int; 
        pub fn clear () -> c_int; 
        pub fn clearok (win: *t::WINDOW, b: bool) -> c_int; 
        pub fn clrtobot () -> c_int; 
        pub fn clrtoeol () -> c_int; 
        pub fn color_content (s0: c_short, s1: *c_short, s2: *c_short, s3: *c_short) -> c_int; 
        pub fn color_set (s0: c_short, v1: *c_void) -> c_int; 
        pub fn COLOR_PAIR (n0: c_int) -> c_int; 
        pub fn copywin (win0: *t::WINDOW, win1: *t::WINDOW, c2: c_int, c3: c_int, c4: c_int,
                        c5: c_int, c6: c_int, c7: c_int, c8: c_int) -> c_int; 
        pub fn curs_set (n0: c_int) -> c_int; 
        pub fn def_prog_mode () -> c_int; 
        pub fn def_shell_mode () -> c_int; 
        pub fn delay_output (n0: c_int) -> c_int; 
        pub fn delch () -> c_int; 
        pub fn delscreen (scr: *t::SCREEN) -> c_void; 
        pub fn delwin (win: *t::WINDOW) -> c_int; 
        pub fn deleteln () -> c_int; 
        pub fn derwin (win: *t::WINDOW, n1: c_int, n2: c_int, n3: c_int, n4: c_int) -> *t::WINDOW; 
        pub fn doupdate () -> c_int; 
        pub fn dupwin (win: *t::WINDOW) ->*t::WINDOW; 
        pub fn echo () -> c_int; 
        pub fn echochar (ch0: t::ctype) -> c_int; 
        pub fn erase () -> c_int; 
        pub fn endwin () -> c_int; 
        pub fn erasechar () -> char; 
        pub fn filter () -> c_void; 
        pub fn flash () -> c_int; 
        pub fn flushinp () -> c_int; 
        pub fn getbkgd (win: *t::WINDOW) -> t::chtype; 
        pub fn getch () -> c_int; 
        pub fn getnstr (c: *char, n1: c_int) -> c_int; 
        pub fn getstr (c: *char) -> c_int; 
        // -----------------------------------------------------------------------------
        pub fn getwin (f: *FILE) ->*t::WINDOW; 
        pub fn halfdelay (n0: c_int) -> c_int; 
        pub fn has_colors () -> bool; 
        pub fn has_ic () -> bool; 
        pub fn has_il () -> bool; 
        pub fn hline (ch0: t::ctype, n1: c_int) -> c_int; 
        pub fn idcok (win: *t::WINDOW, b1: bool) -> c_void; 
        pub fn idlok (win: *t::WINDOW, b1: bool) -> c_int; 
        pub fn immedok (win: *t::WINDOW, b1: bool) -> c_void; 
        pub fn inch () -> t::chtype; 
        pub fn inchnstr (ch0: *t::chtype, n1: c_int) -> c_int; 
        pub fn inchstr (ch0: *t::chtype) -> c_int; 
        pub fn initscr () -> *t::WINDOW; 
        pub fn init_color (s0: c_short, s1: c_short, s2: c_short, s3: c_short) -> c_int; 
        pub fn init_pair (s0: c_short, s1: c_short, s2: c_short) -> c_int; 
        pub fn innstr (c: *char, n1: c_int) -> c_int; 
        pub fn insch (ch0: t::ctype) -> c_int; 
        pub fn insdelln (n0: c_int) -> c_int; 
        pub fn insertln () -> c_int; 
        pub fn insnstr (c: *char, n1: c_int) -> c_int; 
        pub fn insstr (c: *char) -> c_int; 
        pub fn instr (c: *char) -> c_int; 
        pub fn intrflush (win: *t::WINDOW, b1: bool) -> c_int; 
        pub fn isendwin () -> bool; 
        pub fn is_linetouched (win: *t::WINDOW, n1: c_int) -> bool; 
        pub fn is_wintouched (win: *t::WINDOW) -> bool; 
        pub fn keyname (n0: c_int) -> *char; 
        pub fn keypad (win: *t::WINDOW, b1: bool) -> c_int; 
        pub fn killchar () -> char; 
        pub fn leaveok (win: *t::WINDOW, b1: bool) -> c_int; 
        pub fn longname () ->*char; 
        pub fn meta (win: *t::WINDOW, b1: bool) -> c_int; 
        // rust keyworkd "move" shadows this function name
        // pub fn move (n0: c_int, n1: c_int) -> c_int;  
        #[link_name = "move"]
        pub fn mv (n0: c_int, n1: c_int) -> c_int;  
        pub fn mvaddch (n0: c_int, n1: c_int, c2: t::chtype) -> c_int; 
        pub fn mvaddchnstr (n0: c_int, n1: c_int, ch2: *t::chtype, c3: c_int) -> c_int; 
        pub fn mvaddchstr (n0: c_int, n1: c_int, ch2: *t::chtype) -> c_int; 
        pub fn mvaddnstr (n0: c_int, n1: c_int, c2: *char, c3: c_int) -> c_int; 
        pub fn mvaddstr (n0: c_int, n1: c_int, c2: *char) -> c_int; 
        pub fn mvchgat (n0: c_int, n1: c_int, c2: c_int, at3: t::attr_t, s4: c_short, v: *c_void) -> c_int; 
        pub fn mvcur (n0: c_int, n1: c_int, c2: c_int, c3: c_int) -> c_int; 
        pub fn mvdelch (n0: c_int, n1: c_int) -> c_int; 
        pub fn mvderwin (win: *t::WINDOW, n1: c_int, c2: c_int) -> c_int; 
        pub fn mvgetch (n0: c_int, n1: c_int) -> c_int; 
        pub fn mvgetnstr (n0: c_int, n1: c_int, c2: *char, c3: c_int) -> c_int; 
        pub fn mvgetstr (n0: c_int, n1: c_int, c2: *char) -> c_int; 
        pub fn mvhline (n0: c_int, n1: c_int, c2: t::chtype, c3: c_int) -> c_int; 
        pub fn mvinch (n0: c_int, n1: c_int) -> t::chtype; 
        pub fn mvinchnstr (n0: c_int, n1: c_int, ch2: *t::chtype, c3: c_int) -> c_int; 
        pub fn mvinchstr (n0: c_int, n1: c_int, ch2: *t::chtype) -> c_int; 
        pub fn mvinnstr (n0: c_int, n1: c_int, c2: *char, c3: c_int) -> c_int; 
        pub fn mvinsch (n0: c_int, n1: c_int, c2: t::chtype) -> c_int; 
        pub fn mvinsnstr (n0: c_int, n1: c_int, c2: *char, c3: c_int) -> c_int; 
        pub fn mvinsstr (n0: c_int, n1: c_int, c2: *char) -> c_int; 
        pub fn mvinstr (n0: c_int, n1: c_int, c2: *char) -> c_int; 
        ////pub fn mvprc_intw (n0: c_int, n1: c_int, c2: *char三...); 
        //// extern NCURSES_EXPORT(n0: c_int) mvscanw (n0: c_int, n1: c_int二NCURSES_*char三...) 
        // extern NCURSES_EXPORT(n0: c_int) mvvline (n0: c_int, n1: c_int, c2: t::chtype, c3: c_int) -> c_int; 
        pub fn mvvline (n0: c_int, n1: c_int, c2: t::chtype, c3: c_int) -> c_int; 
        pub fn mvwaddch (win: *t::WINDOW, n1: c_int, c2: c_int, ch3: t::chtype) -> c_int; 
        //pub fn mvwaddchnstr (win: *t::WINDOW, n1: c_int, c2: c_int, 
        //                     ch3: *t::chtype, n4: c_int) -> c_int; 

        pub fn mvwaddchnstr (win: *t::WINDOW, n1: c_int, c2: c_int, ch3: *c_char, n4: c_int) -> c_int; 

        //pub fn mvwaddchstr (win: *t::WINDOW, n1: c_int, c2: c_int, ch3: *t::chtype) -> c_int;     
        pub fn mvwaddchstr (win: *t::WINDOW, n1: c_int, c2: c_int, ch3: *c_char) -> c_int;     

        pub fn mvwaddnstr (win: *t::WINDOW, n1: c_int, c2: c_int, c3: *c_char, n4: c_int) -> c_int; 
        pub fn mvwaddstr (win: *t::WINDOW, n1: c_int, c2: c_int, c3: *c_char) -> c_int; 
        pub fn mvwchgat (win: *t::WINDOW, n1: c_int, c2: c_int, c3: c_int, 
                         at4: t::attr_t, s: c_short, v: *c_void) -> c_int; 
        pub fn mvwdelch (win: *t::WINDOW, n1: c_int, c2: c_int) -> c_int; 
        pub fn mvwgetch (win: *t::WINDOW, n1: c_int, c2: c_int) -> c_int; 
        pub fn mvwgetnstr (win: *t::WINDOW, n1: c_int, c2: c_int, c3: *c_char, n4: c_int) -> c_int; 
        pub fn mvwgetstr (win: *t::WINDOW, n1: c_int, c2: c_int, c3: *c_char) -> c_int; 
        pub fn mvwhline (win: *t::WINDOW, n1: c_int, c2: c_int, ch3: t::chtype, n4: c_int) -> c_int; 
        pub fn mvwin (win: *t::WINDOW, n1: c_int, c2: c_int) -> c_int; 
        pub fn mvwinch (win: *t::WINDOW, n1: c_int, c2: c_int) -> t::chtype; 
        pub fn mvwinchnstr (win: *t::WINDOW, n1: c_int, c2: c_int, ch3: *t::chtype, n4: c_int) -> c_int; 
        pub fn mvwinchstr (win: *t::WINDOW, n1: c_int, c2: c_int, ch3: *t::chtype) -> c_int; 
        pub fn mvwinnstr (win: *t::WINDOW, n1: c_int, c2: c_int, c3: *c_char, n4: c_int) -> c_int; 
        pub fn mvwinsch (win: *t::WINDOW, n1: c_int, c2: c_int, ch3: t::chtype) -> c_int; 
        pub fn mvwinsnstr (win: *t::WINDOW, n1: c_int, c2: c_int, c3: *c_char, n4: c_int) -> c_int; 
        pub fn mvwinsstr (win: *t::WINDOW, n1: c_int, c2: c_int, c3: *c_char) -> c_int; 
        pub fn mvwinstr (win: *t::WINDOW, n1: c_int, c2: c_int, c3: *c_char) -> c_int; 
        //pub fn mvwprc_intw (win0: *t::WINDOW, n1: c_int, c2: c_int, c3: *c_char四...); 
        //// extern NCURSES_EXPORT(n0: c_int) mvwscanw (win: *t::WINDOW, n1: c_int, c2: c_int三NCURSES_*c_char四...) 
        // // pub fn extern NCURSES_EXPORT(n0: c_int) mvwvline (win: *t::WINDOW, n1: c_int, c2: c_int, ch3: t::chtype, n4: c_int) -> c_int; 
        pub fn napms (n0: c_int) -> c_int; 
        pub fn newpad (n0: c_int, n1: c_int) -> *t::WINDOW; 
        pub fn newterm (c: *c_char, f1: *FILE, f2: *FILE) -> *t::SCREEN; 
        pub fn newwin (n0: c_int, n1: c_int, c2: c_int, c3: c_int) -> *t::WINDOW; 
        pub fn nl () -> c_int; 
        pub fn nocbreak () -> c_int; 
        pub fn nodelay (win: *t::WINDOW, b1: bool) -> c_int; 
        pub fn noecho () -> c_int; 
        pub fn nonl () -> c_int; 
        pub fn noqiflush () -> c_void; 
        pub fn noraw () -> c_int; 
        pub fn notimeout (win: *t::WINDOW, b1: bool) -> c_int; 
        pub fn overlay (win0: *t::WINDOW, win: *t::WINDOW) -> c_int; 
        pub fn overwrite (win0: *t::WINDOW, win: *t::WINDOW) -> c_int; 
        pub fn pair_content (s0: c_short, s1: *c_short, s2: *c_short) -> c_int; 
        pub fn PAIR_NUMBER (n0: c_int) -> c_int; 
        pub fn pechochar (win: *t::WINDOW, c1: t::chtype) -> c_int; 
        pub fn pnoutrefresh (win0: *t::WINDOW, n1: c_int, c2: c_int, c3: c_int, 
                             c4: c_int, c5: c_int, c6: c_int) -> c_int; 
        pub fn prefresh (win: *t::WINDOW, n1: c_int, c2: c_int, c3: c_int, 
                         c4: c_int, c5: c_int, c6: c_int) -> c_int; 
        pub fn printw (c: *c_char) -> c_int;
        //pub fn prc_intw (c: *c_char一...) 
        // pub fn extern NCURSES_EXPORT(n0: c_int) putwin (win: *t::WINDOW一*FILE) -> c_int; 
        pub fn qiflush () -> c_void; 
        pub fn raw () -> c_int; 
        pub fn redrawwin (win: *t::WINDOW) -> c_int; 
        pub fn refresh () -> c_int; 
        pub fn resetty () -> c_int; 
        pub fn reset_prog_mode () -> c_int; 
        pub fn reset_shell_mode () -> c_int; 
        //pub fn ripoffline (n0: c_int, n1: c_int (*)(win: *t::WINDOW, c2: c_int)) -> c_int;  todo function ptr fun fun fun
        pub fn savetty () -> c_int; 
        //pub fn scanw (NCURSES_*c_char一...) 
        // pub fn extern NCURSES_EXPORT(n0: c_int) scr_dump (c: *c_char) -> c_int; 
        pub fn scr_dump (c: *i8) -> c_int; 
        pub fn scr_init (c: *c_char) -> c_int; 
        pub fn scrl (n0: c_int) -> c_int; 
        pub fn scroll (win: *t::WINDOW) -> c_int; 
        pub fn scrollok (win: *t::WINDOW, b1: bool) -> c_int; 
        pub fn scr_restore (c: *c_char) -> c_int; 
        pub fn scr_set (c: *c_char) -> c_int; 
        pub fn setscrreg (n0: c_int, n1: c_int) -> c_int; 
        pub fn set_term (scr0: *t::SCREEN) -> *t::SCREEN; 
        pub fn slk_attroff (ch0: t::ctype) -> c_int; 
        //pub fn slk_attr_off (at0: t::attr_t, v1: *c_void) -> c_int; 
        pub fn slk_attron (ch0: t::ctype) -> c_int; 
        //pub fn slk_attr_on (at0: t::attr_t, v1: *c_void) -> c_int; 
        pub fn slk_attrset (ch0: t::ctype) -> c_int; 
        pub fn slk_attr () -> t::attr_t; 
        pub fn slk_attr_set (at0: t::attr_t, s1: c_short, v2: *c_void) -> c_int; 
        pub fn slk_clear () -> c_int; 
        pub fn slk_color (s0: c_short) -> c_int; 
        pub fn slk_init (n0: c_int) -> c_int; 
        pub fn slk_label (n0: c_int) -> *c_char; 
        pub fn slk_noutrefresh () -> c_int; 
        pub fn slk_refresh () -> c_int; 
        pub fn slk_restore () -> c_int; 
        pub fn slk_set (n0: c_int, c1: *c_char, c2: c_int) -> c_int; 
        pub fn slk_touch () -> c_int; 
        pub fn standout () -> c_int; 
        pub fn standend () -> c_int; 
        pub fn start_color () -> c_int; 
        pub fn subpad (win: *t::WINDOW, n1: c_int, c2: c_int, c3: c_int, n4: c_int) -> *t::WINDOW; 
        pub fn subwin (win: *t::WINDOW, n1: c_int, c2: c_int, c3: c_int, n4: c_int) -> *t::WINDOW; 
        pub fn syncok (win: *t::WINDOW, b1: bool) -> c_int; 
        pub fn termattrs () -> t::chtype; 
        pub fn termname () -> *c_char; 
        pub fn timeout (n0: c_int) -> c_void; 
        pub fn touchline (win: *t::WINDOW, n1: c_int, c2: c_int) -> c_int; 
        pub fn touchwin (win: *t::WINDOW) -> c_int; 
        pub fn typeahead (n0: c_int) -> c_int; 
        pub fn ungetch (n0: c_int) -> c_int; 
        pub fn untouchwin (win: *t::WINDOW) -> c_int; 
        pub fn use_env (b0: bool) -> c_void; 
        pub fn vidattr (ch0: t::ctype) -> c_int; 
        pub fn vidputs (ch0: t::ctype) -> c_int; 
        pub fn vline (ch0: t::ctype, n1: c_int) -> c_int; 
        pub fn vwprintw (win: *t::WINDOW, c1: *c_char, va2: t::va_list) -> c_int; 
        pub fn vw_printw (win: *t::WINDOW, c1: *c_char, va2: t::va_list) -> c_int; 
        pub fn vwscanw (win: *t::WINDOW, c1: *c_char, va2: t::va_list) -> c_int; 
        pub fn vw_scanw (win: *t::WINDOW, c1: *c_char, va2: t::va_list) -> c_int; 
        pub fn waddch (win: *t::WINDOW, c1: t::chtype) -> c_int; 
        pub fn waddchnstr (win: *t::WINDOW, ch1: *t::chtype, c2: c_int) -> c_int; 
        pub fn waddchstr (win: *t::WINDOW, ch1: *t::chtype) -> c_int; 
        pub fn waddnstr (win: *t::WINDOW, c1: *c_char, c2: c_int) -> c_int; 
        pub fn waddstr (win: *t::WINDOW, c1: *c_char) -> c_int; 
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
                        ch5: t::chtype, ch6: t::chtype, ch7: t::chtype, ch8: t::chtype) -> c_int; 
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
        pub fn wgetnstr (win: *t::WINDOW, c1: *c_char, c2: c_int) -> c_int; 
        pub fn wgetstr (win: *t::WINDOW, c1: *c_char) -> c_int; 
        pub fn whline (win: *t::WINDOW, c1: t::chtype, c2: c_int) -> c_int; 
        pub fn winch (win: *t::WINDOW) -> t::chtype; 
        pub fn winchnstr (win: *t::WINDOW, ch1: *t::chtype, c2: c_int) -> c_int; 
        pub fn winchstr (win: *t::WINDOW, ch1: *t::chtype) -> c_int; 
        pub fn winnstr (win: *t::WINDOW, c1: *c_char, c2: c_int) -> c_int; 
        pub fn winsch (win: *t::WINDOW, c1: t::chtype) -> c_int; 
        pub fn winsdelln (win: *t::WINDOW, n1: c_int) -> c_int; 
        pub fn winsertln (win: *t::WINDOW) -> c_int; 
        pub fn winsnstr (win: *t::WINDOW, c1: *c_char, c2: c_int) -> c_int; 
        pub fn winsstr (win: *t::WINDOW, c1: *c_char) -> c_int; 
        pub fn winstr (win: *t::WINDOW, c1: *c_char) -> c_int; 
        pub fn wmove (win: *t::WINDOW, n1: c_int, c2: c_int) -> c_int; 
        pub fn wnoutrefresh (win: *t::WINDOW) -> c_int; 
        //pub fn wprc_intw (win: *t::WINDOW, c1: *c_char二...); 
        pub fn wredrawln (win: *t::WINDOW, n1: c_int, c2: c_int) -> c_int; 
        pub fn wrefresh (win: *t::WINDOW) -> c_int; 
        //pub fn wscanw (win: *t::WINDOW, c1: *c_char二...); 
        pub fn wscrl (win: *t::WINDOW, n1: c_int) -> c_int; 
        pub fn wsetscrreg (win: *t::WINDOW, n1: c_int, c2: c_int) -> c_int; 
        pub fn wstandout (win: *t::WINDOW) -> c_int; 
        pub fn wstandend (win: *t::WINDOW) -> c_int; 
        pub fn wsyncdown (win: *t::WINDOW) -> c_void; 
        pub fn wsyncup (win: *t::WINDOW) -> c_void; 
        pub fn wtimeout (win: *t::WINDOW, n1: c_int) -> c_void; 
        pub fn wtouchln (win: *t::WINDOW, n1: c_int, c2: c_int, c3: c_int) -> c_int; 
        pub fn wvline (win: *t::WINDOW, c1: t::chtype, c2: c_int) -> c_int; 
        pub fn tigetflag (c0: *c_char) -> c_int; 
        pub fn tigetnum (c0: *c_char) -> c_int; 
        pub fn tigetstr (c0: *c_char) -> *c_char; 
        pub fn putp (c: *c_char) -> c_int; 
        //pub fn tparm (c0: *c_char一...) -> *c_char; 
        pub fn tparm (c0: *c_char, l1: c_long, l2: c_long, l3: c_long, l4: c_long, 
                      l5: c_long, l6: c_long, l7: c_long, l8: c_long, l9: c_long) -> *c_char; 
        //pub fn tparm_varargs (c0: *c_char一...) -> *c_char; 
        //pub fn tiparm (c: *c_char一...) -> *c_char; 
        pub fn getattrs (win: *t::WINDOW) -> c_int; 
        pub fn getcurx (win: *t::WINDOW) -> c_int; 
        pub fn getcury (win: *t::WINDOW) -> c_int; 
        pub fn getbegx (win: *t::WINDOW) -> c_int; 
        pub fn getbegy (win: *t::WINDOW) -> c_int; 
        pub fn getmaxx (win: *t::WINDOW) -> c_int; 
        pub fn getmaxy (win: *t::WINDOW) -> c_int; 
        pub fn getparx (win: *t::WINDOW) -> c_int; 
        pub fn getpary (win: *t::WINDOW) -> c_int; 
        pub fn is_term_resized (n0: c_int, n1: c_int) -> bool; 
        // dup pub fn keybound (n0: c_int, n1: c_int) -> *c_char; 
        pub fn curses_version () -> *c_char; 
        pub fn assume_default_colors (n0: c_int, n1: c_int) -> c_int; 
        pub fn define_key (c: *c_char, n1: c_int) -> c_int; 
        pub fn get_escdelay () -> c_int; 
        pub fn key_defined (c: *c_char) -> c_int; 
        pub fn keyok (n0: c_int, b1: bool) -> c_int; 
        pub fn resize_term (n0: c_int, n1: c_int) -> c_int; 
        pub fn resizeterm (n0: c_int, n1: c_int) -> c_int; 
        pub fn set_escdelay (n0: c_int) -> c_int; 
        pub fn set_tabsize (n0: c_int) -> c_int; 
        pub fn use_default_colors () -> c_int; 
        pub fn use_extended_names (b0: bool) -> c_int; 
        pub fn use_legacy_coding (n0: c_int) -> c_int; 
        pub fn use_screen (scr0: *t::SCREEN, c1: t::SCREEN_CB, v2: *c_void) -> c_int; 
        pub fn use_window (win: *t::WINDOW, c1: t::WINDOW_CB, v2: *c_void) -> c_int; 
        pub fn wresize (win: *t::WINDOW, n1: c_int, c2: c_int) -> c_int; 
        pub fn nofilter() -> c_void; 
        pub fn wgetparent (win: *t::WINDOW) -> *t::WINDOW; 
        pub fn is_cleared (win: *t::WINDOW) -> bool; 
        pub fn is_idcok (win: *t::WINDOW) -> bool; 
        pub fn is_idlok (win: *t::WINDOW) -> bool; 
        pub fn is_immedok (win: *t::WINDOW) -> bool; 
        pub fn is_keypad (win: *t::WINDOW) -> bool; 
        pub fn is_leaveok (win: *t::WINDOW) -> bool; 
        pub fn is_nodelay (win: *t::WINDOW) -> bool; 
        pub fn is_notimeout (win: *t::WINDOW) -> bool; 
        // undef: pub fn is_pad (win: *t::WINDOW) -> bool; 
        pub fn is_scrollok (win: *t::WINDOW) -> bool; 
        // undef: pub fn is_subwin (win: *t::WINDOW) -> bool; 
        pub fn is_syncok (win: *t::WINDOW) -> bool; 
        pub fn wgetscrreg (win: *t::WINDOW, n1: *c_int, n2: *c_int) -> c_int; 
    }
}

#[fixed_stack_segment]
pub fn refresh() -> int {
    unsafe {
        return c::refresh() as int
    }
}


#[fixed_stack_segment]
pub fn printw(s: ~str) {
    unsafe {
        let cs = s.to_c_str().unwrap();
        c::printw(cs);
    }
}

#[fixed_stack_segment]
pub fn getch() -> int {
    unsafe {
        return c::getch() as int
    }
}

#[fixed_stack_segment]
pub fn scr_dump (filename: ~str) -> i32 {
    unsafe {        
        //pub fn scr_dump (c: *char) -> c_int; 
        let bs = filename.to_c_str().unwrap();
        c::scr_dump(bs)
    }
}


    // /// "Set the 'background' set of attributes to attr. This set is
    // /// initially 0 (no attributes)."]
    // fn attrset(self, attr: u32) {
    //     unsafe {
    //         ncurses::attrset(attr as c_int);
    //     }
    // }



static A_NORMAL: u32 =     0;
static A_ATTRIBUTES: u32 = 4294967040;
static A_CHARTEXT: u32 =   255 ;
static A_COLOR: u32 =      65280;
static A_STANDOUT: u32 =   65536 ;
static A_UNDERLINE: u32 =  131072 ;
static A_REVERSE: u32 =    262144 ;
static A_BLINK: u32 =      524288 ;
static A_DIM: u32 =        1048576;
static A_BOLD: u32 =       2097152;
static A_ALTCHARSET: u32 = 4194304;
static A_INVIS: u32 =      8388608;
static A_PROTECT: u32 =    16777216;
static A_HORIZONTAL: u32 = 33554432;
static A_LEFT: u32 =       67108864;
static A_LOW: u32 =        134217728;
static A_RIGHT: u32 =      268435456;
static A_TOP: u32 =        536870912;
static A_VERTICAL: u32 =   1073741824;

/// Set a global? attribute
#[fixed_stack_segment]
pub fn attrset(at: t::NCURSES_ATTR_T) -> int {
    unsafe {
        //let NCURSES_ATTR_SHIFT = 8;
        //#macro[[#Bits[mask, shift], (mask << shift) + NCURSES_ATTR_SHIFT]];  
        //let ct = #Bits[1,0] - 1;
        // NORMAL       { 1 - 1 }
        // ATTRIBUTES   { #Bits[int::compl(1 - 1),0] }
        // CHARTEXT	   { ct }
        // COLOR		   { #Bits[(1 << 8 - 1),0] }
        // STANDOUT     { #Bits[1, 8] }
        // UNDERLINE    { #Bits[1, 9] }
        // REVERSE      { #Bits[1, 10] }
        // BLINK        { #Bits[1, 11] }
        // DIM          { #Bits[1, 12] }
        // BOLD         { #Bits[1, 13] }
        // ALTCHARSET   { #Bits[1, 14] }
        // INVIS        { #Bits[1, 15] }
        // PROTECT      { #Bits[1, 16] }
        // HORIZONTAL   { #Bits[1, 17] }
        // LEFT         { #Bits[1, 18] }
        // LOW          { #Bits[1, 19] }
        // RIGHT        { #Bits[1, 20] }
        // TOP          { #Bits[1, 21] }
        // VERTICAL     { #Bits[1, 22] }

        // sanity check. ./util contains file manually-discover-attributes.c
        // this explicitly calculuates these values. 
        // ones marked 'good' are known to work
        // others are not known to work 

        // let n = match at {
        //     t::NORMAL     => 0,
        //     t::ATTRIBUTES => 4294967040,
        //     t::CHARTEXT   => 255, 
        //     t::COLOR      => 65280,
        //     t::STANDOUT   => 65536, // good
        //     t::UNDERLINE  => 131072, // good
        //     t::REVERSE    => 262144, // good
        //     t::BLINK      => 524288, 
        //     t::DIM        => 1048576,
        //     t::BOLD       => 2097152, // good
        //     t::ALTCHARSET => 4194304,
        //     t::INVIS      => 8388608, // good
        //     t::PROTECT    => 16777216,
        //     t::HORIZONTAL => 33554432,
        //     t::LEFT       => 67108864,
        //     t::LOW        => 134217728,
        //     t::RIGHT      => 268435456,
        //     t::TOP        => 536870912,
        //     t::VERTICAL   => 1073741824,
        // };

        return c::attrset(at) as int;
    }
}
#[test]
fn voidsuite() {
    //unsafe {
        // ncurses::initscr();
        // attrset(NORMAL);
        // printw("voidsuite: ");
        // ncurses::addch(('H' as u32 | A_BOLD) as chtype ); 
        // printw(#fmt["ncurses::baudrate(): %?", ncurses::baudrate()]);
        // #fmt["ncurses::beep : %?", ncurses::beep ()];
        // #fmt["ncurses::can_change_color: %?", ncurses::can_change_color()];
        // #fmt["ncurses::cbreak: %?", ncurses::cbreak()];
        // printw(#fmt["ncurses::clear: %?", ncurses::clear()]);
        // #fmt["ncurses::clrtobot: %?", ncurses::clrtobot()];
        // #fmt["ncurses::clrtoeol: %?", ncurses::clrtoeol()];
        // #fmt["ncurses::def_prog_mode: %?", ncurses::def_prog_mode()];
        // #fmt["ncurses::def_shell_mode: %?", ncurses::def_shell_mode()];
        // #fmt["ncurses::delch: %?", ncurses::delch()];
        // #fmt["ncurses::deleteln: %?", ncurses::deleteln()];
        // #fmt["ncurses::doupdate: %?", ncurses::doupdate()];
        // #fmt["ncurses::echo: %?", ncurses::echo()];
        // #fmt["ncurses::erase: %?", ncurses::erase()];
        // #fmt["ncurses::endwin: %?", ncurses::endwin()];
        // #fmt["ncurses::erasechar: %?", ncurses::erasechar()];
        // #fmt["ncurses::filter: %?", ncurses::filter()];
        // #fmt["ncurses::flash: %?", ncurses::flash()];
        // #fmt["ncurses::flushinp: %?", ncurses::flushinp()];
        // #fmt["ncurses::getch: %?", ncurses::getch()];
        // #fmt["ncurses::has_colors: %?", ncurses::has_colors()];
        // #fmt["ncurses::has_ic: %?", ncurses::has_ic()];
        // #fmt["ncurses::has_il: %?", ncurses::has_il()];
        // #fmt["ncurses::inch: %?", ncurses::inch()];
        // #fmt["ncurses::initscr: %?", ncurses::initscr()];
        // #fmt["ncurses::insertln: %?", ncurses::insertln()];
        // #fmt["ncurses::isendwin: %?", ncurses::isendwin()];
        // #fmt["ncurses::killchar: %?", ncurses::killchar()];
        // #fmt["ncurses::longname: %?", ncurses::longname()];
        // #fmt["ncurses::nl: %?", ncurses::nl()];
        // #fmt["ncurses::nocbreak: %?", ncurses::nocbreak()];
        // #fmt["ncurses::noecho: %?", ncurses::noecho()];
        // #fmt["ncurses::nonl: %?", ncurses::nonl()];
        // #fmt["ncurses::noqiflush: %?", ncurses::noqiflush()];
        // #fmt["ncurses::noraw: %?", ncurses::noraw()];
        // #fmt["ncurses::qiflush: %?", ncurses::qiflush()];
        // #fmt["ncurses::raw: %?", ncurses::raw()];
        // #fmt["ncurses::refresh: %?", ncurses::refresh()];
        // #fmt["ncurses::resetty: %?", ncurses::resetty()];
        // #fmt["ncurses::reset_prog_mode: %?", ncurses::reset_prog_mode()];
        // #fmt["ncurses::reset_shell_mode: %?", ncurses::reset_shell_mode()];
        // #fmt["ncurses::savetty: %?", ncurses::savetty()];
        // #fmt["ncurses::slk_attr: %?", ncurses::slk_attr()];
        // #fmt["ncurses::slk_clear: %?", ncurses::slk_clear()];
        // #fmt["ncurses::slk_noutrefresh: %?", ncurses::slk_noutrefresh()];
        // #fmt["ncurses::slk_refresh: %?", ncurses::slk_refresh()];
        // #fmt["ncurses::slk_restore: %?", ncurses::slk_restore()];
        // #fmt["ncurses::slk_touch: %?", ncurses::slk_touch()];
        // #fmt["ncurses::standout: %?", ncurses::standout()];
        // #fmt["ncurses::standend: %?", ncurses::standend()];
        // #fmt["ncurses::start_color: %?", ncurses::start_color()];
        // #fmt["ncurses::termattrs: %?", ncurses::termattrs()];
        // #fmt["ncurses::termname: %?", ncurses::termname()];
        // #fmt["ncurses::curses_version: %?", ncurses::curses_version()];
        // #fmt["ncurses::get_escdelay: %?", ncurses::get_escdelay()];
        // #fmt["ncurses::use_default_colors: %?", ncurses::use_default_colors()];


        // ncurses::refresh();
        // ncurses::getch();
        // ncurses::endwin();

        

        //ncurses::nofilte();
        //ncurses::new_prescr();
        //ncurses::has_mouse();
        //ncurses::_nc_tracebits();
    //}
}



// there're repeats here, consider modularizing these by first arg.
//     fn new_prescr () -> *SCREEN; 
//     fn baudrate (scr0: *SCREEN) -> c_int; 
//     fn beep (scr0: *SCREEN) -> c_int; 
//     fn can_change_color (scr0: *SCREEN) -> bool; 
//     fn cbreak (scr0: *SCREEN) -> c_int; 
//     fn curs_set (scr0: *SCREEN, n1: c_int) -> c_int; 
//     fn color_content (scr0: *SCREEN, s1: c_short, s2: *c_short, s3: *c_short, s4: *c_short) -> c_int; 
//     fn def_prog_mode (scr0: *SCREEN) -> c_int; 
//     fn def_shell_mode (scr0: *SCREEN) -> c_int; 
//     fn delay_output (scr0: *SCREEN, n1: c_int) -> c_int; 
//     fn doupdate (scr0: *SCREEN) -> c_int; 
//     fn echo (scr0: *SCREEN) -> c_int; 
//     fn endwin (scr0: *SCREEN) -> c_int; 
//     fn erasechar (scr0: *SCREEN) -> char; 
//     fn filter (scr0: *SCREEN) -> c_void; 
//     fn flash (scr0: *SCREEN) -> c_int; 
//     fn flushinp (scr0: *SCREEN) -> c_int; 
//     fn getwin (scr0: *SCREEN, f1: *FILE) -> *WINDOW; 
//     fn halfdelay (scr0: *SCREEN, n1: c_int) -> c_int; 
//     fn has_colors (scr0: *SCREEN) -> bool; 
//     fn has_ic (scr0: *SCREEN) -> bool; 
//     fn has_il (scr0: *SCREEN) -> bool; 
//     fn init_color (scr0: *SCREEN, s1: c_short, s2: c_short, s3: c_short, s4: c_short) -> c_int; 
//     fn init_pair (scr0: *SCREEN, s1: c_short, s2: c_short, s3: c_short) -> c_int; 
//     fn c_intrflush (scr0: *SCREEN, win1: *WINDOW, b: bool) -> c_int; 
//     // dup fn isendwin (scr0: *SCREEN) -> bool; 
//     fn keyname (scr0: *SCREEN, n1: c_int) -> *char; 
//     fn killchar (scr0: *SCREEN) -> char; 
//     fn c_longname (scr0: *SCREEN) -> *char; 
//     fn mvcur (scr0: *SCREEN, n1: c_int, c2: c_int, c3: c_int, n4: c_int) -> c_int; 
//     fn napms (scr0: *SCREEN, n1: c_int) -> c_int; 
//     fn newpad (scr0: *SCREEN, n1: c_int, c2: c_int) -> *WINDOW; 
//     fn newterm (scr0: *SCREEN, c1: *char, f1: *FILE, f2: *FILE) -> *SCREEN; 
//     fn newwin (scr0: *SCREEN, n1: c_int, c2: c_int, c3: c_int, n4: c_int) -> *WINDOW; 
//     fn nl (scr0: *SCREEN) -> c_int; 
//     fn nocbreak (scr0: *SCREEN) -> c_int; 
//     fn noecho (scr0: *SCREEN) -> c_int; 
//     fn nonl (scr0: *SCREEN) -> c_int; 
//     fn noqiflush (scr0: *SCREEN) -> c_void; 
//     fn noraw (scr0: *SCREEN) -> c_int; 
//     fn pair_content (scr0: *SCREEN, s1: c_short, s2: *c_short, s3: *c_short) -> c_int; 
//     fn qiflush (scr0: *SCREEN) -> c_void; 
//     fn raw (scr0: *SCREEN) -> c_int; 
//     fn reset_prog_mode (scr0: *SCREEN) -> c_int; 
//     fn reset_shell_mode (scr0: *SCREEN) -> c_int; 
//     fn resetty (scr0: *SCREEN) -> c_int; 
//     //fn ripoffline (scr0: *SCREEN, n1: c_int, c2: c_int (*)(win: *WINDOW, c3: c_int)) -> c_int;  todo func pointer
//     fn savetty (scr0: *SCREEN) -> c_int; 
//     fn scr_init (scr0: *SCREEN, c1: *char) -> c_int; 
//     fn scr_restore (scr0: *SCREEN, c1: *char) -> c_int; 
//     fn scr_set (scr0: *SCREEN, c1: *char) -> c_int; 
//     fn slk_attroff (scr0: *SCREEN, c1: chtype) -> c_int; 
//     fn slk_attron (scr0: *SCREEN, c1: chtype) -> c_int; 
//     fn slk_attrset (scr0: *SCREEN, c1: chtype) -> c_int; 
//     fn slk_attr (scr0: *SCREEN) -> attr_t; 
//     fn slk_attr_set (scr0: *SCREEN, at2: attr_t, s2: c_short, v3: *c_void) -> c_int; 
//     fn slk_clear (scr0: *SCREEN) -> c_int; 
//     fn slk_color (scr0: *SCREEN, s1: c_short) -> c_int; 
//     fn slk_init (scr0: *SCREEN, n1: c_int) -> c_int; 
//     fn slk_label (scr0: *SCREEN, n1: c_int) -> *char; 
//     fn slk_noutrefresh (scr0: *SCREEN) -> c_int; 
//     fn slk_refresh (scr0: *SCREEN) -> c_int; 
//     fn slk_restore (scr0: *SCREEN) -> c_int; 
//     fn slk_set (scr0: *SCREEN, n1: c_int, c2: *char, c3: c_int) -> c_int; 
//     fn slk_touch (scr0: *SCREEN) -> c_int; 
//     fn start_color (scr0: *SCREEN) -> c_int; 
//     fn termattrs (scr0: *SCREEN) -> chtype; 
//     fn termname (scr0: *SCREEN) -> *char; 
//     fn typeahead (scr0: *SCREEN, n1: c_int) -> c_int; 
//     fn ungetch (scr0: *SCREEN, n1: c_int) -> c_int; 
//     fn use_env (scr0: *SCREEN, b1: bool) -> c_void; 
//     fn vidattr (scr0: *SCREEN, c1: chtype) -> c_int; 
//     fn vidputs (scr0: *SCREEN, c1: chtype) -> c_int; 
//     fn keybound (scr0: *SCREEN, n1: c_int, c2: c_int) -> *char; 
//     fn assume_default_colors (scr0: *SCREEN, n1: c_int, c2: c_int) -> c_int; 
//     fn define_key (scr0: *SCREEN, c1: *char, c2: c_int) -> c_int; 
//     fn get_escdelay (scr0: *SCREEN) -> c_int; 
//     fn is_term_resized (scr0: *SCREEN, n1: c_int, c2: c_int) -> bool; 
//     fn key_defined (scr0: *SCREEN, c1: *char) -> c_int; 
//     fn keyok (scr0: *SCREEN, n1: c_int, b: bool) -> c_int; 
//     fn nofilter (scr0: *SCREEN) -> c_void; 
//     fn resize_term (scr0: *SCREEN, n1: c_int, c2: c_int) -> c_int; 
//     fn resizeterm (scr0: *SCREEN, n1: c_int, c2: c_int) -> c_int; 
//     fn set_escdelay (scr0: *SCREEN, n1: c_int) -> c_int; 
//     fn set_tabsize (scr0: *SCREEN, n1: c_int) -> c_int; 
//     fn use_default_colors (scr0: *SCREEN) -> c_int; 
//     fn use_legacy_coding (scr0: *SCREEN, n1: c_int) -> c_int; 
//     //extern NCURSES_EXPORT_VAR(win: *WINDOW) curscr; 
//     //extern NCURSES_EXPORT_VAR(win: *WINDOW) newscr; 
//     // fn extern NCURSES_EXPORT_VAR(win: *WINDOW) stdscr; 
//     //fn extern NCURSES_EXPORT_VAR(char) ttytype[]; 
//     // fn extern NCURSES_EXPORT_VAR(n0: c_int) COLORS; 
//     //fn extern NCURSES_EXPORT_VAR(n0: c_int) COLOR_PAIRS; 
//     // fn extern NCURSES_EXPORT_VAR(n0: c_int) COLS; 
//     //fn extern NCURSES_EXPORT_VAR(n0: c_int) ESCDELAY; 
//     // fn extern NCURSES_EXPORT_VAR(n0: c_int) LINES; 
//     //fn extern NCURSES_EXPORT_VAR(n0: c_int) TABSIZE; 
//     fn has_mouse(v: c_void) -> bool; 
//     fn has_mouse() -> bool; 
//     fn getmouse (e: *MEVENT) -> c_int; 
//     fn ungetmouse (e: *MEVENT) -> c_int; 
//     fn mousemask (m1: mmask_t, m2: *mmask_t) -> mmask_t; 
//     fn wenclose (win: *WINDOW, n1: c_int, c2: c_int) -> bool; 
//     fn mousec_interval (n0: c_int) -> c_int; 
//     fn wmouse_trafo (win0: *WINDOW, n1: *c_int, c2: *c_int, b: bool) -> bool; 
//     fn mouse_trafo (n0: *c_int, n1: *c_int, b:bool) -> bool; 
//     // fn NCURSES_SP_NAME(has_mouse) (scr0: *SCREEN) -> bool; 
//     // fn NCURSES_SP_NAME(getmouse) (scr0: *SCREEN一*MEVENT) -> c_int; 
//     // fn NCURSES_SP_NAME(ungetmouse) (scr0: *SCREEN一*MEVENT) -> c_int; 
//     // fn NCURSES_SP_NAME(mousemask) (scr0: *SCREEN一mmask_t二mmask_t *) -> mmask_t; 
//     // fn NCURSES_SP_NAME(mousec_interval) (scr0: *SCREEN, n1: c_int) -> c_int; 
//     fn mcprc_int (c: *char, n1: c_int) -> c_int; 
//     fn has_key (n0: c_int) -> c_int; 
//     // fn NCURSES_SP_NAME(has_key) (scr0: *SCREEN, n1: c_int) -> c_int; 
//     // fn NCURSES_SP_NAME(mcprc_int) (scr0: *SCREEN, c1: *char, c2: c_int) -> c_int; 
//     //fn _tracef (c: *char一...) GCC_PRC_INTFLIKE(1二2) -> c_void; 
//     fn _tracedump (c: *char, win: *WINDOW) -> c_void; 
//     fn _traceattr (at0: attr_t) -> *char; 
//     fn _traceattr2 (n0: c_int, c1: chtype) -> *char; 
//     fn _nc_tracebits (v: c_void) -> *char; 
//     fn _tracechar (n0: c_int) -> *char; 
//     fn _tracechtype (ch0: ctype) -> *char; 
//     fn _tracechtype2 (n0: c_int, c1: chtype) -> *char; 
//     fn _tracecchar_t (cc: *cchar_t) -> *char; 
//     fn _tracecchar_t2 (n0: c_int, cc: *cchar_t) -> *char; 
//     fn _tracemouse (e: *MEVENT) -> *char; 
//     fn trace (n: c_uint) -> c_void; 
//     // fn extern NCURSES_EXPORT_VAR(n0: c_int) _nc_optimize_enable; 
//     fn _nc_visbuf (c: *char) -> *char; 
// } 



