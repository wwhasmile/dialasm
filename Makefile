build:
	cargo fmt
	cargo build

test:
	cargo test

run:
	cargo fmt
	cargo run $(args)

run-example:
	cargo fmt
	cargo run example

clean:
	cargo clean

format:
	cargo fmt

clippy:
	cargo clippy

check:
	cargo fmt
	cargo clippy
	cargo test