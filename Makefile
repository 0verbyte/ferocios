all: build

build:
	cargo build

build-release:
	cargo build --release

build-all: build build-release

test:
	cargo test

test-release:
	cargo test --release

test-all: test test-release

clippy:
	cargo clippy --all-targets --all-features
