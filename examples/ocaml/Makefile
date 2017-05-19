## example makefile for building ocaml binary with rust lib
## NB: assumes you use rustup with standard install

RUSTSRC=$(wildcard ../client/src/*.rs)
CLIENT=../client/target/debug/libclient.a
ARGS=-- -Z unstable-options --pretty=expanded

build: test $(CLIENT)

$(CLIENT): $(RUSTSRC)
	cd ../client && cargo build

test: test.ml $(CLIENT)
	ocamlopt.opt -verbose -cclib -lclient -ccopt -L../client/target/debug -ccopt '-L ~/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/' -cclib -ldl -cclib -lpthread   test.ml -o test

pretty: test
	cd ../client && cargo rustc $(ARGS)

clean:
	rm test *.o *.cm*
	rm -rf _build
	cd ../client && cargo clean

.PHONY: build clean pretty
