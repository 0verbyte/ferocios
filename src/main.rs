#![no_std]
#![no_main]

use core::panic::PanicInfo;

extern crate rlibc;

mod vga;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("FerociOS booting{}", "..");
    panic!("Not implemented");
}
