SAFETY= -A unused_imports \
		-A unused_unsafe  \
		-A unused_variable \
		-A unused_mut

build: lib
	rustc ${SAFETY} --bin crate.rs --test -o testbin

test: 
	./testbin

lib:
	rustc ${SAFETY} --lib crate.rs -A unused_imports

clean:
	rm -f libncurses*so *~ testbin

docs:
	rustdoc crate.rs -o ./doc --output-format html
