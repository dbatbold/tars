.PHONY: check build test run clean release format

check:
	cargo check

build:
	cargo build

test:
	cargo test

run:
	cargo run

clean:
	cargo clean

release:
	cargo build --release

format:
	rustfmt src/main.rs
