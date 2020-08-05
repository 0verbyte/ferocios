#!/bin/sh
rustup +nightly component add rust-src llvm-tools-preview
cargo install bootimage
cargo +nightly check
