use core::fmt;
use lazy_static::lazy_static;
use spin::Mutex;
use volatile::Volatile;

use super::color::{Color, ColorCode};
use super::color_scoped_writer::ColorScopedWriter;

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
    pub fn new() -> Self {
        Writer {
            column_position: 0,
            color_code: ColorCode::new(Color::Yellow, Color::Black),
            previous_color_code: None,
            buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
        }
    }

    pub fn color_code(&self) -> ColorCode {
        self.color_code
    }

    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                // Invalid ASCII range
                _ => self.write_byte(0xfe),
            }
        }
    }

    pub fn write_byte(&mut self, byte: u8) {
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

    pub fn new_line(&mut self) {
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                let character = self.buffer.chars[row][col].read();
                self.buffer.chars[row - 1][col].write(character)
            }
        }
        self.clear_row(BUFFER_HEIGHT - 1);
        self.column_position = 0;
    }

    pub fn clear_row(&mut self, row: usize) {
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
            self.color_code = previous_color_code;
            self.previous_color_code = None
        }
    }

    /// Sets color code and returns scoped instance that will reset color code on drop.
    pub fn color_scope(&mut self, color_code: Option<ColorCode>) -> ColorScopedWriter {
        if let Some(color_code) = color_code {
            self.set_color_code(color_code)
        }
        ColorScopedWriter::new(self)
    }
}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

#[test_case]
fn set_color_code() {
    let mut writer = Writer::new();

    let new_color = ColorCode::new(Color::Red, Color::Blue);
    assert_ne!(new_color, writer.color_code);

    writer.set_color_code(new_color);
    assert_eq!(writer.color_code, new_color);
}

#[test_case]
fn retain_previous_color() {
    let mut writer = Writer::new();
    let previous = writer.color_code;

    let new_color = ColorCode::new(Color::Red, Color::Blue);
    assert_ne!(previous, new_color);
    writer.set_color_code(new_color);
    assert_eq!(writer.previous_color_code, Some(previous));
    assert_eq!(writer.color_code, new_color);

    let newer_color = ColorCode::new(Color::Cyan, Color::Pink);
    writer.set_color_code(newer_color);
    assert_eq!(writer.previous_color_code, Some(new_color));
    assert_eq!(writer.color_code, newer_color);
}

#[test_case]
fn reset_color_code() {
    let mut writer = Writer::new();
    let previous = writer.color_code;

    let new_color = ColorCode::new(Color::Red, Color::Blue);
    assert_ne!(previous, new_color);
    writer.set_color_code(new_color);
    assert_eq!(writer.previous_color_code, Some(previous));
    assert_eq!(writer.color_code, new_color);

    writer.reset_color_code();
    assert_eq!(writer.previous_color_code, None);
    assert_eq!(writer.color_code, previous);
}

#[test_case]
fn color_scope() {
    let mut writer = Writer::new();
    let previous = writer.color_code;

    {
        let new_color = ColorCode::new(Color::Red, Color::Blue);
        assert_ne!(previous, new_color);

        let scope = writer.color_scope(Some(new_color));

        // We cannot do `writer.color_code`, which does an immutable borrow, because `writer` is
        // mutable borrowed on previous line.
        assert_eq!(scope.color_code(), new_color);
    } // reset color code

    assert_eq!(writer.color_code, previous);
}
