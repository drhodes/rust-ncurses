SAFETY= -A unused_imports \
		-A unused_unsafe  \
		-A unused_variable \
		-A unused_mut

DEBUG= -Z debug-info

build: lib
	rustc ${SAFETY} ${DEBUG} --bin crate.rs --test -o testbin

test: 
	./testbin && cd tests 

lib:
	rustc ${SAFETY} ${DEBUG} --lib crate.rs -A unused_imports

clean:
	rm -f libncurses*so *~ testbin packed-docs.zip
	cd examples/helloworld && make clean
	cd tests && make clean

docs:
	rustdoc crate.rs -o ./doc --output-format html

