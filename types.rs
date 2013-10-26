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

