use enum_iterator::IntoEnumIterator;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, IntoEnumIterator)]
#[repr(u8)]
pub enum Color {
    Black,
    Blue,
    Green,
    Cyan,
    Red,
    Magenta,
    Brown,
    LightGray,
    DarkGray,
    LightBlue,
    LightGreen,
    LightCyan,
    LightRed,
    Pink,
    Yellow,
    White,
}

impl Color {
    pub fn number(&self) -> u8 {
        *self as u8
    }

    pub fn from_u8(number: u8) -> Option<Color> {
        Color::into_enum_iter().find(|&value| value.number() == number)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct ColorCode(u8);

impl ColorCode {
    pub fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }

    #[allow(dead_code)]
    pub fn foreground(&self) -> Option<Color> {
        Color::from_u8(self.0 & 0xF)
    }

    #[allow(dead_code)]
    pub fn background(&self) -> Option<Color> {
        Color::from_u8(self.0 >> 4)
    }
}

#[test_case]
fn Color_entries_amount() {
    serial_print_func!(".. ");

    assert_eq!(16, Color::VARIANT_COUNT);

    serial_println!("[ok]");
}

#[test_case]
fn Color_number() {
    serial_print_func!(".. ");

    let mut pos = 0;
    for value in Color::into_enum_iter() {
        assert_eq!(value.number(), pos);
        pos += 1;
    }

    serial_println!("[ok]");
}

#[test_case]
fn Color_from_u8() {
    serial_print_func!(".. ");

    for value in Color::into_enum_iter() {
        assert_eq!(Color::from_u8(value.number()), Some(value));
    }

    // Outside the range.
    assert_eq!(Color::from_u8((Color::VARIANT_COUNT + 1) as u8), None);

    serial_println!("[ok]");
}

#[test_case]
fn ColorCode_foreground() {
    serial_print_func!(".. ");

    let fg = Color::Blue;
    let color_code = ColorCode::new(fg, Color::Brown);
    assert_eq!(color_code.foreground(), Some(fg));

    serial_println!("[ok]");
}

#[test_case]
fn ColorCode_background() {
    serial_print_func!(".. ");

    let bg = Color::LightRed;
    let color_code = ColorCode::new(Color::Magenta, bg);
    assert_eq!(color_code.background(), Some(bg));

    serial_println!("[ok]");
}
