use core::convert::TryFrom;
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
}

impl TryFrom<u8> for Color {
    type Error = ();

    fn try_from(number: u8) -> Result<Self, Self::Error> {
        Color::into_enum_iter()
            .find(|&value| value.number() == number)
            .ok_or(())
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
        Color::try_from(self.0 & 0xF).ok()
    }

    #[allow(dead_code)]
    pub fn background(&self) -> Option<Color> {
        Color::try_from(self.0 >> 4).ok()
    }
}

#[test_case]
fn Color_entries_amount() {
    serial_print_fn!(".. ");

    assert_eq!(16, Color::VARIANT_COUNT);

    serial_println!("[ok]");
}

#[test_case]
fn Color_number() {
    serial_print_fn!(".. ");

    let mut pos = 0;
    for value in Color::into_enum_iter() {
        assert_eq!(value.number(), pos);
        pos += 1;
    }

    serial_println!("[ok]");
}

#[test_case]
fn Color_from() {
    serial_print_fn!(".. ");

    for value in Color::into_enum_iter() {
        assert_eq!(Color::try_from(value.number()), Ok(value));
    }

    // Outside the range.
    assert!(Color::try_from((Color::VARIANT_COUNT + 1) as u8).is_err());

    serial_println!("[ok]");
}

#[test_case]
fn ColorCode_foreground() {
    serial_print_fn!(".. ");

    let fg = Color::Blue;
    let color_code = ColorCode::new(fg, Color::Brown);
    assert_eq!(color_code.foreground(), Some(fg));

    serial_println!("[ok]");
}

#[test_case]
fn ColorCode_background() {
    serial_print_fn!(".. ");

    let bg = Color::LightRed;
    let color_code = ColorCode::new(Color::Magenta, bg);
    assert_eq!(color_code.background(), Some(bg));

    serial_println!("[ok]");
}
