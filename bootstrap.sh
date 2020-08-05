#!/bin/sh
rustup +nightly component add rust-src llvm-tools-preview
cargo install bootimage
cargo +nightly check

# Takes program as argument.
checkProgram() {
  if hash $1 2> /dev/null; then
    echo "$1 is installed!"
  else
    echo "$1 is not installed!"
    exit 1
  fi
}

# Takes title as argument.
writeBanner() {
  echo "== $1 =="
}

writeBanner "QEMU"
checkProgram qemu-system-x86_64
