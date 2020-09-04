use core::fmt;
use lazy_static::lazy_static;
use spin::Mutex;
use volatile::Volatile;

use super::color::{Color, ColorCode};

lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer::new());
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode,
}

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

#[repr(transparent)]
struct Buffer {
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

pub struct Writer {
    column_position: usize,
    color_code: ColorCode,
    previous_color_code: Option<ColorCode>,
    buffer: &'static mut Buffer,
}

impl Writer {
    fn new() -> Self {
        Writer {
            column_position: 0,
            color_code: ColorCode::new(Color::Yellow, Color::Black),
            previous_color_code: None,
            buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
        }
    }

    fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                // Invalid ASCII range
                _ => self.write_byte(0xfe),
            }
        }
    }

    fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line()
                }

                let row = BUFFER_HEIGHT - 1;
                let col = self.column_position;

                let color_code = self.color_code;
                self.buffer.chars[row][col].write(ScreenChar {
                    ascii_character: byte,
                    color_code,
                });
                self.column_position += 1;
            }
        }
    }

    fn new_line(&mut self) {
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                let character = self.buffer.chars[row][col].read();
                self.buffer.chars[row - 1][col].write(character)
            }
        }
        self.clear_row(BUFFER_HEIGHT - 1);
        self.column_position = 0;
    }

    fn clear_row(&mut self, row: usize) {
        let blank = ScreenChar {
            ascii_character: b' ',
            color_code: self.color_code,
        };
        for col in 0..BUFFER_WIDTH {
            self.buffer.chars[row][col].write(blank)
        }
    }

    pub fn set_color_code(&mut self, color_code: ColorCode) {
        self.previous_color_code = Some(self.color_code);
        self.color_code = color_code
    }

    pub fn reset_color_code(&mut self) {
        if let Some(previous_color_code) = self.previous_color_code {
            self.color_code = previous_color_code
        }
    }

    /// Sets color code and returns scoped instance that will reset color code on drop.
    pub fn color_scope(&mut self, color_code: Option<ColorCode>) -> ColorScopedWriter {
        if let Some(color_code) = color_code {
            self.set_color_code(color_code)
        }
        ColorScopedWriter { writer: self }
    }
}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

pub struct ColorScopedWriter<'a> {
    writer: &'a mut Writer,
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
