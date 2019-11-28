// Make sure the compiler does not optimize 
// the buffer can change anytime
use volatile::Volatile;

#[allow(dead_code)] // Don't warn me about unused functions
#[derive(Debug, Clone, Copy, PartialEq, Eq)] // Enable some basic semantics
#[repr(u8)] // Allows me to assign values to the enum
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
#[repr(transparent)] // Same data layout as an u8
struct ColorCode(u8);

impl ColorCode{
    fn new(foreground: Color, background: Color) -> ColorCode {
        // "<< 4" shifts for bits to the left 
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)] // field ordering matters
struct ScreenChar {
    ascii_code: u8,
    color_code: ColorCode,
}

// usize: how many bytes it takes to reference any location in memory. 
const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

#[repr(transparent)]
struct Buffer {
    // 2D array for screen
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

pub struct Writer {
    column_position: usize,
    color_code: ColorCode,
    buffer: &'static mut Buffer,
}

impl Writer {
    pub fn write_byte(&mut self, byte: u8){
        match byte{
            b'\n' => self.new_line(),
            byte => {
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }

                let row = BUFFER_HEIGHT -1;
                let col = self.column_position;

                let color_code = self.color_code;
                // The compiler will never optimize this write
                self.buffer.chars[row][col].write(ScreenChar{
                    ascii_code: byte,
                    color_code,
                });
                self.column_position +=1;
            }
        }
    }

    fn new_line(&mut self){
        for row in 1..BUFFER_HEIGHT{
            for col in 0..BUFFER_WIDTH{
                let character = self.buffer.chars[row][col].read();
                self.buffer.chars[row -1][col].write(character);
            }
        }
        // Now lets delete the top line
        self.clear_row(BUFFER_HEIGHT -1);
        self.column_position = 0;
    }

    fn clear_row(&mut self, row: usize){
        let blank = ScreenChar{
            ascii_code: b' ',
            color_code: self.color_code,
        };
        for col in 0..BUFFER_WIDTH{
            self.buffer.chars[row][col].write(blank);
        }
    }

    fn write_string(&mut self, s: &str){
        for byte in s.bytes() {
            match byte {
                // A byte that's part of the ASCII table
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                // else
                _ => self.write_byte(0xfe),
            }
        }
    }
}

/*pub fn print_something(){
    // FIXME: If this dosen't work
    // let mut vga =  0xb8000 as *mut u8;
    let mut vga = unsafe{ &mut *(0xb8000 as *mut Buffer)};
    let mut writer = Writer{
        column_position: 0,
        color_code: ColorCode::new(Color::Yellow, Color::Black),
        buffer: vga,
    };
    // write!("Hello Test {}", 5);
    writer.write_string("Friedrich This works hahah");
}*/

use core::fmt;

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(()) // Return empty stuff
    }
}

// NOTE: Uses lazy statics
// a.k.a initilization is at runtime

use lazy_static::lazy_static;
use spin::Mutex;
//...
lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
        column_position: 0,
        color_code: ColorCode::new(Color::Yellow, Color::Black),
        buffer: unsafe{ &mut *(0xb8000 as *mut Buffer)},
    });
}


#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga_buffer::_print(format_args!($($arg)*)));
}

// use create::print! so we don't have to import both
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

#[cfg(test)]
use crate::{serial_print, serial_println};

#[test_case]
pub fn test_println_output(){
    serial_print!("test_println...");
    println!("THIS IS THE TEST");
    serial_println!("[Passed]");
}

#[test_case]
pub fn test_println_many(){
    serial_print!("test_println_many...");
    for _ in 0..200 {
        println!("Testing many lines");
    }
    serial_println!("[Passed]");
}

#[test_case]
pub fn test_println_output(){
    serial_print!("test_println_output");

    let s = "Some test string that fits on a single line";
    println!("{}",s);
    for (i,c) in s.chars().enumerate() {
        let screen_char = WRITER.lock().buffer.chars[BUFFER_HEIGHT -2][i].read();
        assert_eq!(char::from(screen_char.ascii_code), c);
    }
    serial_println!("[Passed]");
}
