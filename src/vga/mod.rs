mod color;
mod writer;

use color::{Color, ColorCode};
use core::fmt::Write;
use writer::WRITER;

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[macro_export]
macro_rules! eprint {
    ($($arg:tt)*) => ($crate::vga::_eprint(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! eprintln {
    () => ($crate::eprint!("\n"));
    ($($arg:tt)*) => ($crate::eprint!("{}\n", format_args!($($arg)*)));
}

use core::fmt;
#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    let mut writer = WRITER.lock();
    writer.write_fmt(args).unwrap();
    writer.reset_color_code()
}

#[doc(hidden)]
pub fn _eprint(args: fmt::Arguments) {
    set_error_color_code();
    _print(args)
}

fn set_error_color_code() {
    let color_code = ColorCode::new(Color::Red, Color::Black);
    WRITER.lock().set_color_code(color_code)
}
