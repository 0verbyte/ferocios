mod color;
mod writer;

use color::{Color, ColorCode};
use core::fmt;
use core::fmt::Write;
use writer::WRITER;

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga::_print(format_args!($($arg)*), None));
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

#[doc(hidden)]
pub fn _print(args: fmt::Arguments, color_code: Option<ColorCode>) {
    WRITER
        .lock()
        .color_scope(color_code)
        .write_fmt(args)
        .unwrap()
}

#[doc(hidden)]
pub fn _eprint(args: fmt::Arguments) {
    _print(args, Some(ColorCode::new(Color::Red, Color::Black)))
}
