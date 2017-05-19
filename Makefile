SRC=$(wildcard src/*.rs)
ARGS=-- -Z unstable-options --pretty=expanded

.PHONE: test pretty

pretty: $(SRC)
	cargo rustc $(ARGS)
	cd examples/client && cargo rustc $(ARGS)
	cd examples/ocaml && make pretty
	examples/ocaml/test

test: $(SRC)
	cargo test
	cd examples/client && cargo build
	cd examples/ocaml && make
	examples/ocaml/test
