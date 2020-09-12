[![Build, Test, Clippy](https://github.com/ferocios/ferocios/workflows/Build%20Test%20Clippy/badge.svg)](https://github.com/ferocios/ferocios/actions)
[![Security Audit](https://github.com/ferocios/ferocios/workflows/Security%20Audit/badge.svg)](https://github.com/ferocios/ferocios/actions)

# FerociOS
Ferocious..

##  Development
To bootstrap development, run `./bootstrap.sh`.

It checks that nightly Rust and QEMU can be found and is correctly configured. It will also install Rust components `rust-src` and `llvm-tools-preview`, `cargo install bootimage` to easily boot kernel in QEMU, and `cargo check` to check things are working.
