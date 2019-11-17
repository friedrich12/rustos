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
    chars: [[ScreenChar; BUFFER_WIDTH]; BUFFER_HEIGHT],
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
                self.buffer.chars[row][col] = ScreenChar{
                    ascii_code: byte,
                    color_code,
                };
                self.column_position +=1;
            }
        }
    }

    //TODO: Finish this
    pub fn new_line(&mut self){}

    pub fn write_string(&mut self, s: &str){
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