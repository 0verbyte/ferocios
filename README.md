# FerociOS
Ferocious..

##  Development
To bootstrap development, run `./bootstrap.sh`.

It checks that nightly Rust and QEMU can be found and is correctly configured. It will also install Rust components `rust-src` and `llvm-tools-preview`, `cargo install bootimage` to easily boot kernel in QEMU, and `cargo check` to check things are working.
