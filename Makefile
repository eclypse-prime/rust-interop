all: cargo header compile

cargo:
	@(cargo build --release)

header:
	@(cbindgen --lang c --output target/release/rust_interop.h)

compile:
	gcc main.c -o main -Itarget/release -Ltarget/release -lrust_interop -Wl,-rpath=target/release
