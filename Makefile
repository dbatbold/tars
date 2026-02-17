.PHONY: check build run release format

check:
	cargo check

build:
	cargo build

run:
	cargo run

release:
	cargo build --release

format:
	rustfmt src/main.rs
