all:
	rustc --lib ncurses.rc

test:
	rustc --bin ncurses.rc  --test && ./ncurses

clean:
	rm -f libncurses*so
