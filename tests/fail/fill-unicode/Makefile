SAFETY= -A unused_imports \
		-A unused_unsafe  \
		-A unused_variable \
		-A unused_mut

DEBUG= -Z debug-info

build:
	rustc ${SAFETY} ${DEBUG} --bin main.rs -L../../../

test:
	./main	 
	cat output

clean:
	@rm -f ./main *~
	@rm -f output


debug: build
	gdb ./main
