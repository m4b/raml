#LIB=libclient.a
#DEBUG=target/debug
#PATH=../client/${DEBUG}
#RAMLLIB=${PATH}/${LIB}
RUSTSRC=$(wildcard ../client/src/*.rs)
CLIENT=../client/target/debug/libclient.a
ARGS=-- -Z unstable-options --pretty=expanded

.PHONY: build clean pretty

build: test $(CLIENT)

$(CLIENT): $(RUSTSRC)
	cd ../client && cargo build

test: test.ml $(CLIENT)
	ocamlopt.opt -verbose -cclib -lclient -ccopt -L../client/target/debug -ccopt '-L ~/.multirust/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/' -cclib -ldl -cclib -lpthread -cclib -l':libstd-f4594d3e53dcb114.rlib'  test.ml -o test
#	ocamlopt.opt -verbose -cclib -lclient -ccopt -L$(PATH) -ccopt '-L ~/.multirust/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/' -cclib -ldl -cclib -lpthread -cclib -l':libstd-6eb85298.rlib' test.ml -o test

pretty: test
	cd ../client && cargo rustc $(ARGS)

clean:
	rm test *.o *.cm*
	cd ../client && cargo clean
