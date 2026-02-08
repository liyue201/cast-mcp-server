.PHONY: build celan

build:
	cargo build --release

fmt:
	cargo +nightly fmt

clean:
	cargo clean

test:
	cargo test