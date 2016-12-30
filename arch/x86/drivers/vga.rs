
use core::option::*;
use core::mem::transmute;
use core::{str, slice};

pub static ADDRESS: usize = 0xb8000;
pub static WIDTH: u16 = 80;
pub static HEIGHT: u16 = 25;


static mut display: Display = Display { x: 0, y: 0 };

pub fn putchar(c: Char) {
    unsafe { display.putchar(c); }
}

pub fn newline() {
    unsafe {
        display.x = 0;
        display.y += 1;
    }
}


#[derive(Clone)]
pub enum Color {
    Black       = 0,
    Blue        = 1,
    Green       = 2,
    Cyan        = 3,
    Red         = 4,
    Pink        = 5,
    Brown       = 6,
    LightGray   = 7,
    DarkGray    = 8,
    LightBlue   = 9,
    LightGreen  = 10,
    LightCyan   = 11,
    LightRed    = 12,
    LightPink   = 13,
    Yellow      = 14,
    White       = 15,
}

// The screen resolution is 80x25, the root address is 0xb8000
type VGA = [Char; 2000];

struct Display {
    pub y: u16,
    pub x: u16
}

impl Display {
    fn putchar(&mut self, c: Char) {
        if self.x >= WIDTH || self.y >= HEIGHT {
            return;
        }
        let idx : usize =  (self.y * WIDTH * 2 + self.x * 2) as usize;
        unsafe {
            *((ADDRESS + idx) as *mut u16) = c.as_vga_entry();
        }
        self.x += 1;
        if self.x > WIDTH {
            self.x -= WIDTH;
            self.y += 1;
        }
    }
}

// One char in the screen is composed by 2 bytes, 1 byte for the character itself
// and another for styling(foreground and background).
pub struct Char {
  pub char: u8,
  style: u8, // 4 bits for foreground and 4 bits for background
}

impl Char {
  pub fn new(c: char, fg: Color, bg: Color) -> Char {
    Char { char: c as u8, style: fg as u8 | (bg as u8) << 4 }
  }

  pub fn new_char(c: char) -> Char {
    let DEFAULT_FG: Color = Color::Green;
    let DEFAULT_BG: Color = Color::Black;
    Char { char: c as u8, style: DEFAULT_FG as u8 | (DEFAULT_BG as u8) << 4 }
  }

  pub fn as_vga_entry(&self) -> u16 {
    self.char as u16 | (self.style as u16) << 8
  }
}

