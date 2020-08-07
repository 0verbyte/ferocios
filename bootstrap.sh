#!/bin/sh

echo "== Bootstrapping FerociOS =="
printf "Nightly Rust and QEMU are required for a proper dev environment.\n\n"

printf "Checking for rustup and cargo... "
if ! hash rustup 2> /dev/null || ! hash cargo 2> /dev/null; then
  printf "\nRust isn't installed correctly.\n"
  echo "Follow instructions at https://www.rust-lang.org/tools/install"
  exit 1
else
  echo "found"
fi

# Takes package to install.
installProgram() {
  printf "Install %s? (y/N) " "$1"
  read -r reply
  if [ "$reply" = "${reply#[Yy]}" ]; then
    echo "Aborting.."
    exit 1
  fi

  echo "Installing $1..."
  if hash brew 2> /dev/null; then
    brew install "$1" || exit 1
  elif hash port 2> /dev/null; then
    sudo port install "$1" || exit 1
  elif hash apt-get 2> /dev/null; then
    sudo apt-get install "$1" || exit 1
  elif hash yum 2> /dev/null; then
    sudo yum install "$1" || exit 1
  elif hash pacman 2> /dev/null; then
    sudo pacman -S "$1" || exit 1
  else
    echo "Don't know how to install it.. Aborting."
    exit 1
  fi
}

# Takes two arguments: program to check for existence and package to install if not found.
checkProgram() {
  printf "Checking for %s... " "$1"
  if hash "$1" 2> /dev/null; then
    echo "found"
  else
    echo "not found"
    installProgram "$2"
  fi
}

checkProgram qemu-system-x86_64 qemu

echo # newline

set -x
rustup +nightly component add rust-src llvm-tools-preview
cargo install bootimage
cargo +nightly check
