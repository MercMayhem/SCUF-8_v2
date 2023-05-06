mod memory;
mod input;
mod display;
mod runner;

use minifb::{Window, WindowOptions};
use display::Display;

use self::memory::SCRIPT_ADDR;

pub struct Emulator{
    pub memory : [u8; 4096],
    pub reg : [u8; 16],
    pub i : u16,
    pub pc : u16,
    pub stack : [Option<u16>; 16],
    pub sp : usize,
    pub delay : u8,
    pub sound : u8,
    pub display : Display,
    pub tick_rate: u128,
}

impl Emulator {
    pub fn initialize(tick_rate:u128) -> Emulator{
        let emu = Emulator{
            memory : [0; 4096],
            reg : [0; 16],
            i : 0,
            pc : SCRIPT_ADDR as u16,
            stack : [None; 16],
            sp : 0,
            delay : 0,
            sound : 0,
            tick_rate,
            display : Display {
                buf : [[0; 64]; 32],
                window : Window::new("SCUF-8", 640, 320, WindowOptions::default()).unwrap()
            }
        };

        return emu;
    }

    pub fn run_script(&mut self, script: &[u8]){
        self.load_font();
        self.load_script(script);
        self.main_loop();
    }
}