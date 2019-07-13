extern crate volatile;

extern crate lazy_static;
extern crate spin;

use super::io;

use self::lazy_static::lazy_static;
use self::spin::Mutex;

lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
        column_position: 0,
        color_code: ColorCode::new(Color::White, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    });
}

//dead bc some colors might be unused.
//we want to treat this value with this derived attr.
//internal rep must be u8!
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct ColorCode(u8);
//colorcode is just 4 bits of bg, then 4 bits of fg.
impl ColorCode {
    pub fn new(fg: Color, bg: Color) -> ColorCode {
        ColorCode((bg as u8) << 4 | (fg as u8))
    }
}

//a screen char is colorcode and char.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode,
}

//default vga buffer values.
const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

//volatile so that rustc doesnt optimize away.
use self::volatile::Volatile;
//the buffer is just a 2d array.
#[repr(transparent)]
pub struct Buffer {
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}
//main thing: this writes to screen.
pub struct Writer {
    pub column_position: usize,
    pub color_code: ColorCode,
    pub buffer: &'static mut Buffer,
}

impl Writer {
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            //treat newlines
            b'\n' => self.new_line(),
            b'\t' => self.tab(),
            byte => {
                //account for going over width
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }
                //always start from bbottom.
                let row = BUFFER_HEIGHT - 1;
                let col = self.column_position;
                //physically move the screenchar into memory
                let color_code = self.color_code;
                self.buffer.chars[row][col].write(ScreenChar {
                    ascii_character: byte,
                    color_code,
                });
                //update our position
                self.column_position += 1;
                move_cursor(self.column_position as u8, (BUFFER_HEIGHT - 1) as u8);
            }
        }
    }
    fn tab(&mut self) {
        for _ in 0..(4 - self.column_position % 4) {
            self.write_byte(0x20);
        }
    }
    fn new_line(&mut self) {
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                //just shift everything up a row.
                let character = self.buffer.chars[row][col].read();
                self.buffer.chars[row - 1][col].write(character);
            }
        }
        //and reset position.
        self.clear_row(BUFFER_HEIGHT - 1);
        self.column_position = 0;
        move_cursor(self.column_position as u8, (BUFFER_HEIGHT - 1) as u8);
    }
    fn clear_row(&mut self, row: usize) {
        let blank = ScreenChar {
            ascii_character: b' ',
            color_code: self.color_code,
        };
        for col in 0..BUFFER_WIDTH {
            self.buffer.chars[row][col].write(blank);
        }
    }
    pub fn clear(&mut self) {
        for row in 0..BUFFER_HEIGHT {
            self.clear_row(row);
        }
        self.column_position = 0;
        move_cursor(self.column_position as u8, (BUFFER_HEIGHT - 1) as u8);
    }
    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                0x20...0x7e | b'\n' | b'\t' =>
                //only handle printable ascii.
                {
                    self.write_byte(byte)
                }
                _ => self.write_byte(0xfe), //everything else put a block char.
            }
        }
    }
}

//for using regular write! and format! macros.
use core::fmt;
impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(()) //always succeed
    }
}

// Print Macros

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n",format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    WRITER.lock().write_fmt(args).unwrap();
}

pub fn clear() {
    WRITER.lock().clear();
}

pub fn move_cursor(x: u8, y: u8) {
    let pos: u16 = (y as u16) * (BUFFER_WIDTH as u16) + (x as u16);

    unsafe {
        io::outb(0x0F, 0x3D4);
        io::outb((pos & 0xFF) as u8, 0x3D5);
        io::outb(0x0E, 0x3D4);
        io::outb(((pos >> 8) & 0xFF) as u8, 0x3d5);
    }
}
