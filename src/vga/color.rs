#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
    Black = 0x0,
    Blue = 0x1,
    Green = 0x2,
    Cyan = 0x3,
    Red = 0x4,
    Magenta = 0x5,
    Brown = 0x6,
    LightGray = 0x7,
    DarkGray = 0x8,
    LightBlue = 0x9,
    LightGreen = 0xA,
    LightCyan = 0xB,
    LightRed = 0xC,
    Pink = 0xD,
    Yellow = 0xE,
    White = 0xF,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct ColorCode(u8);

impl ColorCode {
    pub fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

#[test_case]
fn construct_fg_bg() {
    serial_print_func!(".. ");

    let fg = Color::Blue;
    let bg = Color::White;
    let color_code = ColorCode::new(fg, bg);
    assert_eq!(color_code.0, (bg as u8) << 4 | (fg as u8));

    serial_println!("[ok]");
}

#[test_case]
fn deconstruct_fg_bg() {
    serial_print_func!(".. ");

    let fg = Color::Blue;
    let bg = Color::White;
    let color_code = ColorCode::new(fg, bg);
    assert_eq!(color_code.0 & 0xF, fg as u8);
    assert_eq!(color_code.0 >> 4, bg as u8);

    serial_println!("[ok]");
}
