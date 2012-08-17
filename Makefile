all:
	rustc --bin ncurses.rc --test  

test:
	rustc --bin ncurses.rc  --test && ./ncurses
#
#clean:
#	rm -rf datetime *.so
#