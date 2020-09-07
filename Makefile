all: build

build:
	cargo build

build-verbose:
	cargo build --verbose

build-release:
	cargo build --release

build-all: build build-release

test:
	cargo test

test-verbose:
	cargo test --verbose

test-release:
	cargo test --release

test-all: test test-release

clippy:
	cargo clippy --all-targets --all-features
