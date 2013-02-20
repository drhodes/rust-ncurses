// Copyright 2012 Derek A. Rhodes.  All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.
// 

use libc::*;
use io;

enum NCURSES_ATTR_T {
      NORMAL,
      ATTRIBUTES,
      CHARTEXT,
      COLOR,
      STANDOUT,
      UNDERLINE,
      REVERSE,
      BLINK,
      DIM,
      BOLD,
      ALTCHARSET,
      INVIS,
      PROTECT,
      HORIZONTAL,
      LEFT,
      LOW,
      RIGHT,
      TOP,
      VERTICAL,
}

pub enum PANEL{}
pub enum WINDOW{}
enum SCREEN{}
//enum chtype{}
enum _nc_eventlist{}
enum attr_t{}
enum ctype{}
enum mmask_t{}
enum cchar_t{}
enum SCREEN_CB{}
enum WINDOW_CB{}
enum MEVENT{}
enum va_list{}
type chtype = u32;

extern mod ncurses {
    //extern NCURSES_EXPORT_VAR(chtype) acs_map[];  
    //fn wgetch_events (win: *WINDOW, nc: *_nc_eventlist) -> c_int; 
    //fn wgetnstr_events (win: *WINDOW, c: *char, n: c_int, nc: *_nc_eventlist); 

    fn addch (ch: chtype) -> c_int; 
    fn addchnstr (c: *chtype, n1: c_int) -> c_int; 
    fn addchstr (c: *chtype) -> c_int; 
    fn addnstr (c: *char, n1: c_int) -> c_int; 
    fn addstr (c: *c_char) -> c_int; 
    fn attroff (nc: NCURSES_ATTR_T) -> c_int; 
    fn attron (nc: NCURSES_ATTR_T) -> c_int; 
//    fn attrset (nc: NCURSES_ATTR_T) -> c_int; 
    fn attrset (nc: c_int) -> c_int; 
    fn attr_get (at1: *attr_t, s1: *c_short, v2: *c_void) -> c_int; 
    fn attr_off (at0: attr_t, v1: *c_void) -> c_int; 
    fn attr_on (at0: attr_t, v1: *c_void) -> c_int; 
    fn attr_set (at0: attr_t, s1: c_short, v2: *c_void) -> c_int; 
    fn baudrate () -> c_int; 
    fn beep  () -> c_int; 
    fn bkgd (ch0: ctype) -> c_int; 
    fn bkgdset (ch0: ctype) -> c_void; 
    fn border (c1: chtype, c2: chtype, c3: chtype, c4: chtype, 
               c5: chtype, c6: chtype, c7: chtype, c8: chtype) -> c_int; 
    fn box (win: *WINDOW, c1: chtype, c2: chtype) -> c_int; 
    fn can_change_color () -> bool; 
    fn cbreak () -> c_int; 
    fn chgat (n: c_int, at2: attr_t, s2: c_short, v3: *c_void) -> c_int; 
    fn clear () -> c_int; 
    fn clearok (win: *WINDOW, b: bool) -> c_int; 
    fn clrtobot () -> c_int; 
    fn clrtoeol () -> c_int; 
    fn color_content (s0: c_short, s1: *c_short, s2: *c_short, s3: *c_short) -> c_int; 
    fn color_set (s0: c_short, v1: *c_void) -> c_int; 
    fn COLOR_PAIR (n0: c_int) -> c_int; 
    fn copywin (win0: *WINDOW, win1: *WINDOW, c2: c_int, c3: c_int, c4: c_int,
                c5: c_int, c6: c_int, c7: c_int, c8: c_int) -> c_int; 
    fn curs_set (n0: c_int) -> c_int; 
    fn def_prog_mode () -> c_int; 
    fn def_shell_mode () -> c_int; 
    fn delay_output (n0: c_int) -> c_int; 
    fn delch () -> c_int; 
    fn delscreen (scr: *SCREEN) -> c_void; 
    fn delwin (win: *WINDOW) -> c_int; 
    fn deleteln () -> c_int; 
    fn derwin (win: *WINDOW, n1: c_int, n2: c_int, n3: c_int, n4: c_int) -> *WINDOW; 
    fn doupdate () -> c_int; 
    fn dupwin (win: *WINDOW) ->*WINDOW; 
    fn echo () -> c_int; 
    fn echochar (ch0: ctype) -> c_int; 
    fn erase () -> c_int; 
    fn endwin () -> c_int; 
    fn erasechar () -> char; 
    fn filter () -> c_void; 
    fn flash () -> c_int; 
    fn flushinp () -> c_int; 
    fn getbkgd (win: *WINDOW) -> chtype; 
    fn getch () -> c_int; 
    fn getnstr (c: *char, n1: c_int) -> c_int; 
    fn getstr (c: *char) -> c_int; 
    // -----------------------------------------------------------------------------
    fn getwin (f: *FILE) ->*WINDOW; 
    fn halfdelay (n0: c_int) -> c_int; 
    fn has_colors () -> bool; 
    fn has_ic () -> bool; 
    fn has_il () -> bool; 
    fn hline (ch0: ctype, n1: c_int) -> c_int; 
    fn idcok (win: *WINDOW, b1: bool) -> c_void; 
    fn idlok (win: *WINDOW, b1: bool) -> c_int; 
    fn immedok (win: *WINDOW, b1: bool) -> c_void; 
    fn inch () -> chtype; 
    fn inchnstr (ch0: *chtype, n1: c_int) -> c_int; 
    fn inchstr (ch0: *chtype) -> c_int; 
    fn initscr () -> *WINDOW; 

    fn init_color (s0: c_short, s1: c_short, s2: c_short, s3: c_short) -> c_int; 
    fn init_pair (s0: c_short, s1: c_short, s2: c_short) -> c_int; 
    fn innstr (c: *char, n1: c_int) -> c_int; 
    fn insch (ch0: ctype) -> c_int; 
    fn insdelln (n0: c_int) -> c_int; 
    fn insertln () -> c_int; 
    fn insnstr (c: *char, n1: c_int) -> c_int; 
    fn insstr (c: *char) -> c_int; 
    fn instr (c: *char) -> c_int; 
    fn intrflush (win: *WINDOW, b1: bool) -> c_int; 
    fn isendwin () -> bool; 
    fn is_linetouched (win: *WINDOW, n1: c_int) -> bool; 
    fn is_wintouched (win: *WINDOW) -> bool; 
    fn keyname (n0: c_int) -> *char; 
    fn keypad (win: *WINDOW, b1: bool) -> c_int; 
    fn killchar () -> char; 
    fn leaveok (win: *WINDOW, b1: bool) -> c_int; 
    fn longname () ->*char; 
    fn meta (win: *WINDOW, b1: bool) -> c_int; 

    // rust keyworkd "move" shadows this function name
    // fn move (n0: c_int, n1: c_int) -> c_int;  

    #[link_name = "move"]
    fn mv (n0: c_int, n1: c_int) -> c_int;  

    fn mvaddch (n0: c_int, n1: c_int, c2: chtype) -> c_int; 
    fn mvaddchnstr (n0: c_int, n1: c_int, ch2: *chtype, c3: c_int) -> c_int; 
    fn mvaddchstr (n0: c_int, n1: c_int, ch2: *chtype) -> c_int; 
    fn mvaddnstr (n0: c_int, n1: c_int, c2: *char, c3: c_int) -> c_int; 
    fn mvaddstr (n0: c_int, n1: c_int, c2: *char) -> c_int; 
    fn mvchgat (n0: c_int, n1: c_int, c2: c_int, at3: attr_t, s4: c_short, v: *c_void) -> c_int; 
    fn mvcur (n0: c_int, n1: c_int, c2: c_int, c3: c_int) -> c_int; 
    fn mvdelch (n0: c_int, n1: c_int) -> c_int; 
    fn mvderwin (win: *WINDOW, n1: c_int, c2: c_int) -> c_int; 
    fn mvgetch (n0: c_int, n1: c_int) -> c_int; 
    fn mvgetnstr (n0: c_int, n1: c_int, c2: *char, c3: c_int) -> c_int; 
    fn mvgetstr (n0: c_int, n1: c_int, c2: *char) -> c_int; 
    fn mvhline (n0: c_int, n1: c_int, c2: chtype, c3: c_int) -> c_int; 
    fn mvinch (n0: c_int, n1: c_int) -> chtype; 
    fn mvinchnstr (n0: c_int, n1: c_int, ch2: *chtype, c3: c_int) -> c_int; 
    fn mvinchstr (n0: c_int, n1: c_int, ch2: *chtype) -> c_int; 
    fn mvinnstr (n0: c_int, n1: c_int, c2: *char, c3: c_int) -> c_int; 
    fn mvinsch (n0: c_int, n1: c_int, c2: chtype) -> c_int; 
    fn mvinsnstr (n0: c_int, n1: c_int, c2: *char, c3: c_int) -> c_int; 
    fn mvinsstr (n0: c_int, n1: c_int, c2: *char) -> c_int; 
    fn mvinstr (n0: c_int, n1: c_int, c2: *char) -> c_int; 
    ////fn mvprc_intw (n0: c_int, n1: c_int, c2: *char三...); 
    //// extern NCURSES_EXPORT(n0: c_int) mvscanw (n0: c_int, n1: c_int二NCURSES_*char三...) 

    // extern NCURSES_EXPORT(n0: c_int) mvvline (n0: c_int, n1: c_int, c2: chtype, c3: c_int) -> c_int; 
    fn mvvline (n0: c_int, n1: c_int, c2: chtype, c3: c_int) -> c_int; 
    fn mvwaddch (win: *WINDOW, n1: c_int, c2: c_int, ch3: chtype) -> c_int; 
    fn mvwaddchnstr (win: *WINDOW, n1: c_int, c2: c_int, ch3: *chtype, n4: c_int) -> c_int; 
    fn mvwaddchstr (win: *WINDOW, n1: c_int, c2: c_int, ch3: *chtype) -> c_int;     
    fn mvwaddnstr (win: *WINDOW, n1: c_int, c2: c_int, c3: *char, n4: c_int) -> c_int; 
    fn mvwaddstr (win: *WINDOW, n1: c_int, c2: c_int, c3: *char) -> c_int; 
    fn mvwchgat (win: *WINDOW, n1: c_int, c2: c_int, c3: c_int, at4: attr_t, s: c_short, v: *c_void) -> c_int; 
    fn mvwdelch (win: *WINDOW, n1: c_int, c2: c_int) -> c_int; 
    fn mvwgetch (win: *WINDOW, n1: c_int, c2: c_int) -> c_int; 
    fn mvwgetnstr (win: *WINDOW, n1: c_int, c2: c_int, c3: *char, n4: c_int) -> c_int; 
    fn mvwgetstr (win: *WINDOW, n1: c_int, c2: c_int, c3: *char) -> c_int; 
    fn mvwhline (win: *WINDOW, n1: c_int, c2: c_int, ch3: chtype, n4: c_int) -> c_int; 
    fn mvwin (win: *WINDOW, n1: c_int, c2: c_int) -> c_int; 
    fn mvwinch (win: *WINDOW, n1: c_int, c2: c_int) -> chtype; 
    fn mvwinchnstr (win: *WINDOW, n1: c_int, c2: c_int, ch3: *chtype, n4: c_int) -> c_int; 
    fn mvwinchstr (win: *WINDOW, n1: c_int, c2: c_int, ch3: *chtype) -> c_int; 
    fn mvwinnstr (win: *WINDOW, n1: c_int, c2: c_int, c3: *char, n4: c_int) -> c_int; 
    fn mvwinsch (win: *WINDOW, n1: c_int, c2: c_int, ch3: chtype) -> c_int; 
    fn mvwinsnstr (win: *WINDOW, n1: c_int, c2: c_int, c3: *char, n4: c_int) -> c_int; 
    fn mvwinsstr (win: *WINDOW, n1: c_int, c2: c_int, c3: *char) -> c_int; 
    fn mvwinstr (win: *WINDOW, n1: c_int, c2: c_int, c3: *char) -> c_int; 
    //fn mvwprc_intw (win0: *WINDOW, n1: c_int, c2: c_int, c3: *char四...); 
    //// extern NCURSES_EXPORT(n0: c_int) mvwscanw (win: *WINDOW, n1: c_int, c2: c_int三NCURSES_*char四...) 
    // // fn extern NCURSES_EXPORT(n0: c_int) mvwvline (win: *WINDOW, n1: c_int, c2: c_int, ch3: chtype, n4: c_int) -> c_int; 
    fn napms (n0: c_int) -> c_int; 
    fn newpad (n0: c_int, n1: c_int) -> *WINDOW; 
    fn newterm (c: *char, f1: *FILE, f2: *FILE) -> *SCREEN; 
    fn newwin (n0: c_int, n1: c_int, c2: c_int, c3: c_int) -> *WINDOW; 
    fn nl () -> c_int; 
    fn nocbreak () -> c_int; 
    fn nodelay (win: *WINDOW, b1: bool) -> c_int; 
    fn noecho () -> c_int; 
    fn nonl () -> c_int; 
    fn noqiflush () -> c_void; 
    fn noraw () -> c_int; 
    fn notimeout (win: *WINDOW, b1: bool) -> c_int; 
    fn overlay (win0: *WINDOW, win: *WINDOW) -> c_int; 
    fn overwrite (win0: *WINDOW, win: *WINDOW) -> c_int; 
    fn pair_content (s0: c_short, s1: *c_short, s2: *c_short) -> c_int; 
    fn PAIR_NUMBER (n0: c_int) -> c_int; 
    fn pechochar (win: *WINDOW, c1: chtype) -> c_int; 
    fn pnoutrefresh (win0: *WINDOW, n1: c_int, c2: c_int, c3: c_int, 
                     c4: c_int, c5: c_int, c6: c_int) -> c_int; 
    fn prefresh (win: *WINDOW, n1: c_int, c2: c_int, c3: c_int, 
                 c4: c_int, c5: c_int, c6: c_int) -> c_int; 
    fn printw (c: *c_char) -> c_int;

    //fn prc_intw (c: *char一...) 
    // fn extern NCURSES_EXPORT(n0: c_int) putwin (win: *WINDOW一*FILE) -> c_int; 
    fn qiflush () -> c_void; 
    fn raw () -> c_int; 
    fn redrawwin (win: *WINDOW) -> c_int; 
    fn refresh () -> c_int; 
    fn resetty () -> c_int; 
    fn reset_prog_mode () -> c_int; 
    fn reset_shell_mode () -> c_int; 
    //fn ripoffline (n0: c_int, n1: c_int (*)(win: *WINDOW, c2: c_int)) -> c_int;  todo function ptr fun fun fun
    fn savetty () -> c_int; 
    //fn scanw (NCURSES_*char一...) 
    // fn extern NCURSES_EXPORT(n0: c_int) scr_dump (c: *char) -> c_int; 
    fn scr_init (c: *char) -> c_int; 
    fn scrl (n0: c_int) -> c_int; 
    fn scroll (win: *WINDOW) -> c_int; 
    fn scrollok (win: *WINDOW, b1: bool) -> c_int; 
    fn scr_restore (c: *char) -> c_int; 
    fn scr_set (c: *char) -> c_int; 
    fn setscrreg (n0: c_int, n1: c_int) -> c_int; 
    fn set_term (scr0: *SCREEN) -> *SCREEN; 
    fn slk_attroff (ch0: ctype) -> c_int; 
    //fn slk_attr_off (at0: attr_t, v1: *c_void) -> c_int; 
    fn slk_attron (ch0: ctype) -> c_int; 
    //fn slk_attr_on (at0: attr_t, v1: *c_void) -> c_int; 
    fn slk_attrset (ch0: ctype) -> c_int; 
    fn slk_attr () -> attr_t; 
    fn slk_attr_set (at0: attr_t, s1: c_short, v2: *c_void) -> c_int; 
    fn slk_clear () -> c_int; 
    fn slk_color (s0: c_short) -> c_int; 
    fn slk_init (n0: c_int) -> c_int; 
    fn slk_label (n0: c_int) -> *char; 
    fn slk_noutrefresh () -> c_int; 
    fn slk_refresh () -> c_int; 
    fn slk_restore () -> c_int; 
    fn slk_set (n0: c_int, c1: *char, c2: c_int) -> c_int; 
    fn slk_touch () -> c_int; 
    fn standout () -> c_int; 
    fn standend () -> c_int; 
    fn start_color () -> c_int; 
    fn subpad (win: *WINDOW, n1: c_int, c2: c_int, c3: c_int, n4: c_int) -> *WINDOW; 
    fn subwin (win: *WINDOW, n1: c_int, c2: c_int, c3: c_int, n4: c_int) -> *WINDOW; 
    fn syncok (win: *WINDOW, b1: bool) -> c_int; 
    fn termattrs () -> chtype; 
    fn termname () -> *char; 
    fn timeout (n0: c_int) -> c_void; 
    fn touchline (win: *WINDOW, n1: c_int, c2: c_int) -> c_int; 
    fn touchwin (win: *WINDOW) -> c_int; 
    fn typeahead (n0: c_int) -> c_int; 
    fn ungetch (n0: c_int) -> c_int; 
    fn untouchwin (win: *WINDOW) -> c_int; 
    fn use_env (b0: bool) -> c_void; 
    fn vidattr (ch0: ctype) -> c_int; 
    fn vidputs (ch0: ctype) -> c_int; 
    fn vline (ch0: ctype, n1: c_int) -> c_int; 
    fn vwprintw (win: *WINDOW, c1: *char, va2: va_list) -> c_int; 
    fn vw_printw (win: *WINDOW, c1: *char, va2: va_list) -> c_int; 
    fn vwscanw (win: *WINDOW, c1: *char, va2: va_list) -> c_int; 
    fn vw_scanw (win: *WINDOW, c1: *char, va2: va_list) -> c_int; 
    fn waddch (win: *WINDOW, c1: chtype) -> c_int; 
    fn waddchnstr (win: *WINDOW, ch1: *chtype, c2: c_int) -> c_int; 
    fn waddchstr (win: *WINDOW, ch1: *chtype) -> c_int; 
    fn waddnstr (win: *WINDOW, c1: *char, c2: c_int) -> c_int; 
    fn waddstr (win: *WINDOW, c1: *char) -> c_int; 
    fn wattron (win: *WINDOW, n1: c_int) -> c_int; 
    fn wattroff (win: *WINDOW, n1: c_int) -> c_int; 
    fn wattrset (win: *WINDOW, n1: c_int) -> c_int; 
    fn wattr_get (win: *WINDOW, at1: *attr_t, s2: *c_short, v3: *c_void) -> c_int; 
    fn wattr_on (win: *WINDOW, at2: attr_t, v2: *c_void) -> c_int; 
    fn wattr_off (win: *WINDOW, at2: attr_t, v2: *c_void) -> c_int; 
    fn wattr_set (win: *WINDOW, at2: attr_t, s2: c_short, v3: *c_void) -> c_int; 
    fn wbkgd (win: *WINDOW, c1: chtype) -> c_int; 
    fn wbkgdset (win: *WINDOW, c1: chtype) -> c_void; 
    fn wborder (win: *WINDOW, c1: chtype, c2: chtype, ch3: chtype, ch4: chtype,
                ch5: chtype, ch6: chtype, ch7: chtype, ch8: chtype) -> c_int; 
    fn wchgat (win: *WINDOW, n1: c_int, at: attr_t, s3: c_short, v: *c_void) -> c_int; 
    fn wclear (win: *WINDOW) -> c_int; 
    fn wclrtobot (win: *WINDOW) -> c_int; 
    fn wclrtoeol (win: *WINDOW) -> c_int; 
    fn wcolor_set (win0: *WINDOW, s1: c_short, v2: *c_void) -> c_int; 
    fn wcursyncup (win: *WINDOW) -> c_void; 
    fn wdelch (win: *WINDOW) -> c_int; 
    fn wdeleteln (win: *WINDOW) -> c_int; 
    fn wechochar (win: *WINDOW, c1: chtype) -> c_int; 
    fn werase (win: *WINDOW) -> c_int; 
    fn wgetch (win: *WINDOW) -> c_int; 
    fn wgetnstr (win: *WINDOW, c1: *char, c2: c_int) -> c_int; 
    fn wgetstr (win: *WINDOW, c1: *char) -> c_int; 
    fn whline (win: *WINDOW, c1: chtype, c2: c_int) -> c_int; 
    fn winch (win: *WINDOW) -> chtype; 
    fn winchnstr (win: *WINDOW, ch1: *chtype, c2: c_int) -> c_int; 
    fn winchstr (win: *WINDOW, ch1: *chtype) -> c_int; 
    fn winnstr (win: *WINDOW, c1: *char, c2: c_int) -> c_int; 
    fn winsch (win: *WINDOW, c1: chtype) -> c_int; 
    fn winsdelln (win: *WINDOW, n1: c_int) -> c_int; 
    fn winsertln (win: *WINDOW) -> c_int; 
    fn winsnstr (win: *WINDOW, c1: *char, c2: c_int) -> c_int; 
    fn winsstr (win: *WINDOW, c1: *char) -> c_int; 
    fn winstr (win: *WINDOW, c1: *char) -> c_int; 
    fn wmove (win: *WINDOW, n1: c_int, c2: c_int) -> c_int; 
    fn wnoutrefresh (win: *WINDOW) -> c_int; 
    //fn wprc_intw (win: *WINDOW, c1: *char二...); 
    fn wredrawln (win: *WINDOW, n1: c_int, c2: c_int) -> c_int; 
    fn wrefresh (win: *WINDOW) -> c_int; 
    //fn wscanw (win: *WINDOW, c1: *char二...); 
    fn wscrl (win: *WINDOW, n1: c_int) -> c_int; 
    fn wsetscrreg (win: *WINDOW, n1: c_int, c2: c_int) -> c_int; 
    fn wstandout (win: *WINDOW) -> c_int; 
    fn wstandend (win: *WINDOW) -> c_int; 
    fn wsyncdown (win: *WINDOW) -> c_void; 
    fn wsyncup (win: *WINDOW) -> c_void; 
    fn wtimeout (win: *WINDOW, n1: c_int) -> c_void; 
    fn wtouchln (win: *WINDOW, n1: c_int, c2: c_int, c3: c_int) -> c_int; 
    fn wvline (win: *WINDOW, c1: chtype, c2: c_int) -> c_int; 

    fn tigetflag (c0: *char) -> c_int; 
    fn tigetnum (c0: *char) -> c_int; 
    fn tigetstr (c0: *char) -> *char; 
    fn putp (c: *char) -> c_int; 
    //fn tparm (c0: *char一...) -> *char; 
    fn tparm (c0: *char, l1: c_long, l2: c_long, l3: c_long, l4: c_long, 
              l5: c_long, l6: c_long, l7: c_long, l8: c_long, l9: c_long) -> *char; 
    //fn tparm_varargs (c0: *char一...) -> *char; 
    //fn tiparm (c: *char一...) -> *char; 
    fn getattrs (win: *WINDOW) -> c_int; 
    fn getcurx (win: *WINDOW) -> c_int; 
    fn getcury (win: *WINDOW) -> c_int; 
    fn getbegx (win: *WINDOW) -> c_int; 
    fn getbegy (win: *WINDOW) -> c_int; 
    fn getmaxx (win: *WINDOW) -> c_int; 
    fn getmaxy (win: *WINDOW) -> c_int; 
    fn getparx (win: *WINDOW) -> c_int; 
    fn getpary (win: *WINDOW) -> c_int; 
    fn is_term_resized (n0: c_int, n1: c_int) -> bool; 
    // dup fn keybound (n0: c_int, n1: c_int) -> *char; 
    fn curses_version () -> *char; 
    fn assume_default_colors (n0: c_int, n1: c_int) -> c_int; 
    fn define_key (c: *char, n1: c_int) -> c_int; 
    fn get_escdelay () -> c_int; 
    fn key_defined (c: *char) -> c_int; 
    fn keyok (n0: c_int, b1: bool) -> c_int; 
    fn resize_term (n0: c_int, n1: c_int) -> c_int; 
    fn resizeterm (n0: c_int, n1: c_int) -> c_int; 
    fn set_escdelay (n0: c_int) -> c_int; 
    fn set_tabsize (n0: c_int) -> c_int; 
    fn use_default_colors () -> c_int; 
    fn use_extended_names (b0: bool) -> c_int; 
    fn use_legacy_coding (n0: c_int) -> c_int; 
    fn use_screen (scr0: *SCREEN, c1: SCREEN_CB, v2: *c_void) -> c_int; 
    fn use_window (win: *WINDOW, c1: WINDOW_CB, v2: *c_void) -> c_int; 
    fn wresize (win: *WINDOW, n1: c_int, c2: c_int) -> c_int; 
    fn nofilter() -> c_void; 
    fn wgetparent (win: *WINDOW) -> *WINDOW; 
    fn is_cleared (win: *WINDOW) -> bool; 
    fn is_idcok (win: *WINDOW) -> bool; 
    fn is_idlok (win: *WINDOW) -> bool; 
    fn is_immedok (win: *WINDOW) -> bool; 
    fn is_keypad (win: *WINDOW) -> bool; 
    fn is_leaveok (win: *WINDOW) -> bool; 
    fn is_nodelay (win: *WINDOW) -> bool; 
    fn is_notimeout (win: *WINDOW) -> bool; 
    // undef: fn is_pad (win: *WINDOW) -> bool; 
    fn is_scrollok (win: *WINDOW) -> bool; 
    // undef: fn is_subwin (win: *WINDOW) -> bool; 
    fn is_syncok (win: *WINDOW) -> bool; 
    fn wgetscrreg (win: *WINDOW, n1: *c_int, n2: *c_int) -> c_int; 

}

fn printw(s: ~str) {
    unsafe {
        str::as_c_str(s, ncurses::printw);
    }
}



const      A_NORMAL: u32 =     0;
const      A_ATTRIBUTES: u32 = 4294967040;
const      A_CHARTEXT: u32 =   255 ;
const      A_COLOR: u32 =      65280;
const      A_STANDOUT: u32 =   65536 ;
const      A_UNDERLINE: u32 =  131072 ;
const      A_REVERSE: u32 =    262144 ;
const      A_BLINK: u32 =      524288 ;
const      A_DIM: u32 =        1048576;
const      A_BOLD: u32 =       2097152;
const      A_ALTCHARSET: u32 = 4194304;
const      A_INVIS: u32 =      8388608;
const      A_PROTECT: u32 =    16777216;
const      A_HORIZONTAL: u32 = 33554432;
const      A_LEFT: u32 =       67108864;
const      A_LOW: u32 =        134217728;
const      A_RIGHT: u32 =      268435456;
const      A_TOP: u32 =        536870912;
const      A_VERTICAL: u32 =   1073741824;


unsafe fn attrset(at: NCURSES_ATTR_T) -> int {
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

    let val = match at {
      NORMAL     => 0,
      ATTRIBUTES => 4294967040,
      CHARTEXT   => 255, 
      COLOR      => 65280,
      STANDOUT   => 65536, // good
      UNDERLINE  => 131072, // good
      REVERSE    => 262144, // good
      BLINK      => 524288, 
      DIM        => 1048576,
      BOLD       => 2097152, // good
      ALTCHARSET => 4194304,
      INVIS      => 8388608, // good
      PROTECT    => 16777216,
      HORIZONTAL => 33554432,
      LEFT       => 67108864,
      LOW        => 134217728,
      RIGHT      => 268435456,
      TOP        => 536870912,
      VERTICAL   => 1073741824,
    };
    //ret ncurses::attrset(val as c_int) as int
    return ncurses::attrset(val as c_int) as int
}

// #[test]
// fn hello() {
//     ncurses::initscr();/* Start curses mode   */

//     attrset(BOLD);
//     printw("Hello");
        
//     attrset(REVERSE);
//     printw(" World !!!");

//     ncurses::refresh();/* Print it on to the real screen */
//     ncurses::getch();/* Wait for user input */
//     ncurses::endwin();/* End curses mode  */
// }

#[test]
fn voidsuite() {
    unsafe {
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


        ncurses::refresh();
        ncurses::getch();
        ncurses::endwin();

        

        //ncurses::nofilte();
        //ncurses::new_prescr();
        //ncurses::has_mouse();
        //ncurses::_nc_tracebits();
    }
}





// there repeats here, consider modularizing these by first arg.
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



