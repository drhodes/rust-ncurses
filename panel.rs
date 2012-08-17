use std;
import libc::*;
import io;
import str::unsafe;
import result::{result, ok, err};

// typedef struct panel
// {
//     WINDOW *win;
//     struct panel *below;
//     struct panel *above;
//     NCURSES_CONST void *user;
// } PANEL;

enum PANEL{}
enum WINDOW{}
enum chtype{}
extern mod ncurses {
// -----------------------------------------------------------------------------
// ./panel.h:56:extern NCURSES_EXPORT(WINDOW*) panel_window (const PANEL *);
    // fn panel_window (p: *PANEL) -> *WINDOW;
// ./panel.h:57:extern NCURSES_EXPORT(void)    update_panels (void);
// ./panel.h:58:extern NCURSES_EXPORT(int)     hide_panel (PANEL *);
// ./panel.h:59:extern NCURSES_EXPORT(int)     show_panel (PANEL *);
// ./panel.h:60:extern NCURSES_EXPORT(int)     del_panel (PANEL *);
// ./panel.h:61:extern NCURSES_EXPORT(int)     top_panel (PANEL *);
// ./panel.h:62:extern NCURSES_EXPORT(int)     bottom_panel (PANEL *);
// ./panel.h:63:extern NCURSES_EXPORT(PANEL*)  new_panel (WINDOW *);
// ./panel.h:64:extern NCURSES_EXPORT(PANEL*)  panel_above (const PANEL *);
// ./panel.h:65:extern NCURSES_EXPORT(PANEL*)  panel_below (const PANEL *);
// ./panel.h:66:extern NCURSES_EXPORT(int)     set_panel_userptr (PANEL *, NCURSES_CONST void *);
// ./panel.h:67:extern NCURSES_EXPORT(NCURSES_CONST void*) panel_userptr (const PANEL *);
// ./panel.h:68:extern NCURSES_EXPORT(int)     move_panel (PANEL *, int, int);
// ./panel.h:69:extern NCURSES_EXPORT(int)     replace_panel (PANEL *,WINDOW *);
// ./panel.h:70:extern NCURSES_EXPORT(int)     panel_hidden (const PANEL *);
// ./panel.h:73:extern NCURSES_EXPORT(PANEL *) ground_panel(SCREEN *);
// ./panel.h:74:extern NCURSES_EXPORT(PANEL *) ceiling_panel(SCREEN *);
// ./panel.h:76:extern NCURSES_EXPORT(void)    NCURSES_SP_NAME(update_panels) (SCREEN*);

// ./ncurses.h:608:extern NCURSES_EXPORT(int) hline (chtype, int);				/* generated */
    //fn hline (c: chtype, n: c_int) -> c_int;				/* generated */


}
