use core::fmt;

use super::color::ColorCode;
use super::writer::Writer;

pub struct ColorScopedWriter<'a> {
    writer: &'a mut Writer,
}

impl<'a> ColorScopedWriter<'a> {
    pub fn new(writer: &'a mut Writer) -> Self {
        ColorScopedWriter { writer }
    }

    // Ignore dead code: function is only used in tests right now.
    #[allow(dead_code)]
    pub fn color_code(&self) -> ColorCode {
        self.writer.color_code()
    }
}

impl<'a> Drop for ColorScopedWriter<'a> {
    fn drop(&mut self) {
        self.writer.reset_color_code()
    }
}

impl<'a> fmt::Write for ColorScopedWriter<'a> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.writer.write_str(s)
    }
}

#[test_case]
fn color_code() {
    serial_print_func!(".. ");

    let mut writer = Writer::new();
    let color_code = writer.color_code();
    let scope = ColorScopedWriter::new(&mut writer);
    assert_eq!(scope.color_code(), color_code);

    serial_println!("[ok]");
}

#[test_case]
fn reset_on_drop() {
    use super::color::Color;

    serial_print_func!(".. ");

    let mut writer = Writer::new();
    let previous = writer.color_code();
    let new_color = ColorCode::new(Color::Red, Color::Blue);
    assert_ne!(previous, new_color);
    writer.set_color_code(new_color);

    {
        let _scope = ColorScopedWriter::new(&mut writer);
    } // reset color code

    assert_eq!(writer.color_code(), previous);

    serial_println!("[ok]");
}
