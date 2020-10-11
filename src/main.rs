#![no_std]
#![no_main]
#![feature(asm)]
#![feature(abi_x86_interrupt)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test::test_runner)]
#![reexport_test_harness_main = "test_main"]

mod qemu;

#[cfg(test)]
use crate::qemu::{exit_qemu, QemuExitCode};

use core::panic::PanicInfo;

extern crate rlibc;

#[macro_use]
mod util;

#[macro_use]
mod serial;

#[cfg(test)]
mod test;

#[macro_use]
mod vga;

mod gdt;
mod interrupts;

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    eprintln!("{}", info);

    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);

    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    gdt::init();
    interrupts::init();
    println!("FerociOS booting..");

    #[cfg(test)]
    test_main();

    panic!("Not implemented")
}
