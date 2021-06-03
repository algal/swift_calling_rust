build:
	cargo build --release

clean:
	cargo clean

bindings:
	cbindgen --config cbindgen.toml --crate rust_factorial --lang C  --output CFactorial.h

