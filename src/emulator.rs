mod memory;
mod input;
mod display;
mod runner;

use minifb::{Window, WindowOptions};
use display::Display;

pub struct Emulator{
    pub memory : [u8; 4096],
    pub reg : [u8; 16],
    pub i : u16,
    pub pc : u16,
    pub stack : [Option<u16>; 16],
    pub sp : u8,
    pub delay : u8,
    pub sound : u8,
    pub window : Display,
    pub tick_us: u16,
}