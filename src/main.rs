#![no_std]
#![no_main]

use core::panic::PanicInfo;

extern crate rlibc;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

static BOOT_STR: &[u8] = b"FerociOS booted!";

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in BOOT_STR.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0x0a;
        }
    }

    loop {}
}
