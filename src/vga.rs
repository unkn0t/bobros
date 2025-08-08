use core::{fmt, ptr};

use spin::{Lazy, Mutex};

use crate::memory::Volatile;

pub static WRITER: Lazy<Mutex<Writer>> = Lazy::new(|| Mutex::new(Writer::new()));

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    WRITER.lock().write_fmt(args).unwrap();
}

pub struct Writer {
    column: usize,
    color_code: ColorCode,
    buffer: &'static mut TextBuffer,
}

#[repr(transparent)]
struct TextBuffer {
    chars: [[Volatile<ScreenChar>; TextBuffer::WIDTH]; TextBuffer::HEIGHT],
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(C)]
struct ScreenChar {
    character: u8,
    color_code: ColorCode,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(transparent)]
pub struct ColorCode(u8);

impl fmt::Write for Writer {
    #[inline]
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_bytes(s.as_bytes());
        Ok(())
    }
}

impl Writer {
    const fn new() -> Self {
        Self {
            column: 0,
            color_code: ColorCode::new(Color::Yellow, Color::Black),
            buffer: unsafe { &mut *TextBuffer::ADDRESS },
        }
    }

    pub fn write_bytes(&mut self, bytes: &[u8]) {
        for &byte in bytes {
            match byte {
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                _ => self.write_byte(0xfe),
            }
        }
    }

    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            _ => {
                if self.column >= TextBuffer::WIDTH {
                    self.new_line();
                }

                let row = TextBuffer::HEIGHT - 1;
                let col = self.column;

                self.buffer.chars[row][col].write(ScreenChar::new(byte, self.color_code));
                self.column += 1;
            }
        }
    }

    pub fn set_color(&mut self, color_code: ColorCode) {
        self.color_code = color_code;
    }

    fn new_line(&mut self) {
        for row in 1..TextBuffer::HEIGHT {
            for col in 0..TextBuffer::WIDTH {
                let char = self.buffer.chars[row][col].read();
                self.buffer.chars[row - 1][col].write(char);
            }
        }
        self.clear_row(TextBuffer::HEIGHT - 1);
        self.column = 0;
    }

    fn clear_row(&mut self, row: usize) {
        let blank = ScreenChar::new(b' ', self.color_code);
        for col in 0..TextBuffer::WIDTH {
            self.buffer.chars[row][col].write(blank);
        }
    }
}

impl TextBuffer {
    const WIDTH: usize = 80;
    const HEIGHT: usize = 25;

    const ADDRESS: *mut Self = ptr::without_provenance_mut(0xb8000);
}

impl ScreenChar {
    const fn new(character: u8, color_code: ColorCode) -> Self {
        Self {
            character,
            color_code,
        }
    }
}

impl ColorCode {
    pub const fn new(foreground: Color, background: Color) -> Self {
        Self((background as u8) << 4 | (foreground as u8))
    }
}
