use std::time::Instant;
use rand::Rng;

use super::{Emulator, display};

impl Emulator{
    // Main Emulator Loop
    pub fn main_loop(&mut self){
        let mut tick = Instant::now();
        let mut display_tick = Instant::now();
        
        loop{
            // 60fps
            if display_tick.elapsed().as_micros() >= 16666{
                self.display.refresh_display();

                self.sound = self.sound.wrapping_sub(1);
                self.delay = self.delay.wrapping_sub(1);

                display_tick = Instant::now();
            }

            // tps (Typically 700 tps)
            if tick.elapsed().as_micros() >= self.tick_rate{
                // wait till duration elapses
                continue;
            }

            tick = Instant::now();

            let byte1 = self.memory[self.pc as usize];
            let byte2 = self.memory[self.pc as usize + 1];

            self.pc += 2;

            let n1 = (byte1 & 0xF0) >> 4;
            let n2 = byte1 & 0x0F;
            let n3 = (byte2 & 0xF0) >> 4;
            let n4 = byte2 & 0x0F;

            match (n1, n2, n3, n4) {
                (0x0, 0x0, 0xE, 0x0) => {
                    // 00E0
                    // CLS
                    self.display.clear()
                },

                (0x0, 0x0, 0xE, 0xE) => {
                    // 00E0
                    // RET
                    self.sp -= 1;
                    self.pc = self.stack[self.sp].unwrap();
                    self.stack[self.sp] = None;
                },

                (0x1, ..) => {
                    // 1nnn
                    // JP addr
                    self.pc = (n2 << 8 + byte2) as u16
                },

                (0x2, ..) => {
                    // 2nnn
                    // CALL addr
                    self.stack[self.sp] = Some(self.pc);
                    self.sp += 1;
                    self.pc = (n2 << 8 + byte2) as u16
                },

                (0x3, ..) => {
                    // 3xnn
                    if self.reg[n2 as usize] == byte2 {
                        self.pc += 2;
                    }
                },

                (0x4, ..) => {
                    // 4XNN
                    if self.reg[n2 as usize] != byte2 {
                        self.pc += 2;
                    }
                },

                (0x5, _, _, 0x0) => {
                    // 5XY0
                    if self.reg[n2 as usize] == self.reg[n3 as usize] {
                        self.pc += 2;
                    }
                },

                (0x6, ..) => {
                    // 6XNN
                    self.reg[n2 as usize] = byte2;
                },

                (0x7, ..) => {
                    // 7XNN
                    self.reg[n2 as usize] = self.reg[n2 as usize].wrapping_add(byte2);
                },

                (0x8, _, _, 0x0) => {
                    // 8XY0
                    self.reg[n2 as usize] = self.reg[n3 as usize];
                },

                (0x8, _, _, 0x1) => {
                    // 8XY1
                    self.reg[n2 as usize] |= self.reg[n3 as usize];
                }

                (0x8, _, _, 0x2) => {
                    // 8XY2
                    self.reg[n2 as usize] &= self.reg[n3 as usize];
                },

                (0x8, _, _, 0x3) => {
                    // 8XY3
                    self.reg[n2 as usize] ^= self.reg[n3 as usize];
                },

                (0x8, _, _, 0x4) => {
                    // 8XY4
                    self.reg[n2 as usize] = self.reg[n2 as usize].wrapping_add(self.reg[n3 as usize])
                },

                (0x8, _, _, 0x5) => {
                    // 8XY5
                    self.reg[15] = if self.reg[n3 as usize] > self.reg[n2 as usize] { 1 } else { 0 };
                    self.reg[n2 as usize] = self.reg[n2 as usize].wrapping_sub(self.reg[n3 as usize])
                },

                (0x8, _, _, 0x6) => {
                    // 8XY6
                    self.reg[15] = self.reg[n2 as usize] & 0b1;
                    self.reg[n2 as usize] >>= 1;
                },

                (0x8, _, _, 0x7) => {
                    // 8XY7
                    self.reg[n2 as usize] = self.reg[n3 as usize].wrapping_sub(self.reg[n2 as usize]);
                    self.reg[15] = if self.reg[n2 as usize] > self.reg[n3 as usize] { 1 } else { 0 };
                },

                (0x8, _, _, 0xE) => {
                    // 8XYE
                    self.reg[15] = self.reg[n2 as usize] & 0b1000_0000;
                    self.reg[n2 as usize] <<= 1;
                },

                (0x9, _, _, 0x0) => {
                    // 9XY0
                    if self.reg[n2 as usize] != self.reg[n3 as usize]{
                        self.pc += 1;
                    }
                },

                (0xB, ..) => {
                    // BNNN
                    self.pc = self.reg[0] as u16 + ((n2 as u16) << 8 + byte2 as u16)
                },

                (0xA, ..) => {
                    // ANNN
                    self.i = (n2 as u16) << 8 + byte2 as u16
                },

                (0xC, ..) => {
                    // CXNN
                    let mut rng = rand::thread_rng();
                    self.reg[n2 as usize] = byte2 & rng.gen::<u8>();
                },

                (0xD, ..) => {
                    // DXYN
                    let coords = (self.reg[n2 as usize], self.reg[n3 as usize]);
                    let sprite = &self.memory[self.i as usize .. (self.i + n4 as u16) as usize];

                    self.reg[15] = self.display.draw(coords.0, coords.1, sprite) as u8;
                },

                (0xE, _, 0x9, 0xE) => {
                    // EX9E
                    if self.scan_key(self.reg[n2 as usize]) {
                        self.pc += 2;
                    }
                },

                (0xE, _, 0xA, 0x1) => {
                    // EXA1
                    if !self.scan_key(self.reg[n2 as usize]) {
                        self.pc += 2;
                    }
                },

                (0xF, _, 0x0, 0x7) => {
                    // FX07
                    self.reg[n2 as usize] = self.delay
                },

                (0xF, _, 0x1, 0x5) => {
                    // FX15
                    self.delay = self.reg[n2 as usize]
                },

                (0xF, _, 0x2, 0x9) => {
                    // FX29
                    let spr = self.reg[n2 as usize];
                    self.i = 5 * spr as u16;
                },

                (0xF, _, 0x3, 0x3) => {
                    // FX33
                    let val = self.reg[n2 as usize];

                    let hundreds = val / 100;
                    let tens = (val % 100) / 10;
                    let ones = val % 10;

                    self.memory[self.i as usize] = hundreds;
                    self.memory[self.i as usize + 1] = tens;
                    self.memory[self.i as usize + 2] = ones;
                },

                (0xF, _, 0x5, 0x5) => {
                    // FX55
                    for i in 0..=n2 as usize{
                        self.memory[self.i as usize + i] = self.reg[i as usize]
                    }
                },

                (0xF, _, 0x6, 0x5) => {
                    //FX65
                    for i in 0..=n2 as usize{
                        self.reg[i as usize] = self.memory[self.i as usize + i]
                    }
                },

                _ => continue
            }
        }
    }
}