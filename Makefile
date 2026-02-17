.PHONY: check build test run release format

check:
	cargo check

build:
	cargo build

test:
	cargo test

run:
	cargo run

release:
	cargo build --release

format:
	rustfmt src/main.rs
