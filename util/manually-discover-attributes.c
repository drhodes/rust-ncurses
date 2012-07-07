#include <stdio.h>

#define NCURSES_ATTR_SHIFT       8
#define NCURSES_BITS(mask,shift) ((mask) << ((shift) + NCURSES_ATTR_SHIFT))

#define A_NORMAL    (1UL - 1UL)
#define A_ATTRIBUTES    NCURSES_BITS(~(1UL - 1UL),0)
#define A_CHARTEXT  (NCURSES_BITS(1UL,0) - 1UL)
#define A_COLOR     NCURSES_BITS(((1UL) << 8) - 1UL,0)
#define A_STANDOUT  NCURSES_BITS(1UL,8)
#define A_UNDERLINE NCURSES_BITS(1UL,9)
#define A_REVERSE   NCURSES_BITS(1UL,10)
#define A_BLINK     NCURSES_BITS(1UL,11)
#define A_DIM       NCURSES_BITS(1UL,12)
#define A_BOLD      NCURSES_BITS(1UL,13)
#define A_ALTCHARSET    NCURSES_BITS(1UL,14)
#define A_INVIS     NCURSES_BITS(1UL,15)
#define A_PROTECT   NCURSES_BITS(1UL,16)
#define A_HORIZONTAL    NCURSES_BITS(1UL,17)
#define A_LEFT      NCURSES_BITS(1UL,18)
#define A_LOW       NCURSES_BITS(1UL,19)
#define A_RIGHT     NCURSES_BITS(1UL,20)
#define A_TOP       NCURSES_BITS(1UL,21)
#define A_VERTICAL  NCURSES_BITS(1UL,22)

int main() {
  printf(" A_NORMAL: %lu\n", A_NORMAL);
  printf(" A_ATTRIBUTES: %lu\n", A_ATTRIBUTES);
  printf(" A_CHARTEXT: %lu\n", A_CHARTEXT);
  printf(" A_COLOR: %lu\n", A_COLOR);
  printf(" A_STANDOUT: %lu\n", A_STANDOUT);
  printf(" A_UNDERLINE: %lu\n", A_UNDERLINE);
  printf(" A_REVERSE: %lu\n", A_REVERSE);
  printf(" A_BLINK: %lu\n", A_BLINK);
  printf(" A_DIM: %lu\n", A_DIM);
  printf(" A_BOLD: %lu\n", A_BOLD);
  printf(" A_ALTCHARSET: %lu\n", A_ALTCHARSET);
  printf(" A_INVIS: %lu\n", A_INVIS);
  printf(" A_PROTECT: %lu\n", A_PROTECT);
  printf(" A_HORIZONTAL: %lu\n", A_HORIZONTAL);
  printf(" A_LEFT: %lu\n", A_LEFT);
  printf(" A_LOW: %lu\n", A_LOW);
  printf(" A_RIGHT: %lu\n", A_RIGHT);
  printf(" A_TOP: %lu\n", A_TOP);
  printf(" A_VERTICAL: %lu\n", A_VERTICAL);
  return 0;
}
