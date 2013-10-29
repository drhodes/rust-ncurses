use std::libc::types::os::arch::c95::{ c_char, c_int, c_short, c_long};
use std::libc::types::common::c95::{ c_void, FILE};

pub enum PANEL{}
pub enum WINDOW{}
pub enum SCREEN{}
pub enum _nc_eventlist{}
pub enum attr_t{}
pub enum ctype{}
pub enum mmask_t{}
pub enum cchar_t{}
pub enum SCREEN_CB{}
pub enum WINDOW_CB{}
pub enum MEVENT{}
pub enum va_list{}

/// chtype: An integral type that can contain at least an unsigned char
/// and attributes. Values of type chtype are formed by OR-ing together an
/// unsigned char value and zero or more of the base attribute flags
/// defined in that have the A_ prefix. The application can extract these
/// components of a chtype value using the base masks defined in
/// <curses.h> for this purpose. The chtype data type also contains a
/// colour-pair. Values of type chtype are formed by OR-ing together an
/// unsigned char value, a colour pair, and zero or more of the attributes
/// defined in <curses.h> that begin with the prefix A_. The application
/// can extract these components of a chtype value using the masks defined
/// in <curses.h> for this purpose.
pub type chtype = u32;

pub static TRUE: c_int = 1;
pub static FALSE: c_int = 0;
pub static ERR: c_int = -1;
pub static OK: c_int = 0;


pub enum NCURSES_ATTR_T {
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

