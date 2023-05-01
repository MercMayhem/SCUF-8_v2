use minifb::{Window, WindowOptions};

const WIDTH: usize = 64;
const HEIGHT: usize = 32;

pub struct Display{
    buf : [[u32; WIDTH]; HEIGHT],
    window : Window
}

impl Display{
    fn refresh_display(&mut self){       
        let mut flattened_buffer = [0_u32; 2048];
        for row in 0..32_u16{
            let start_index = row * 64;
            let end_index = start_index + 64;

            flattened_buffer[start_index as usize..end_index as usize].copy_from_slice(&self.buf[row as usize][..]);
        }

        self.window.update_with_buffer(&flattened_buffer, 64, 32).unwrap()
    }

    fn draw(&mut self, X:u8, Y:u8, sprite:&[u8]) -> bool{
        let mut collision = false;
        
        for row in 0..sprite.len() as u8{    
            let mut rel_y = Y + row ;
            for pixel in 0..8_u8{
                let bit = sprite[row as usize] & (0x1 << 7 - pixel);
                let mut rel_x = X + pixel;

                if rel_x > 63{
                    rel_x = rel_x - 63;
                }
                if rel_y > 31{
                    rel_y = rel_y - 31;
                }
                if self.buf[rel_y as usize][rel_x as usize] == 0xFFFFFF && bit == 1{
                    collision = true
                }

                self.buf[rel_y as usize][rel_x as usize] = (bit as u32 * 0xFFFFFF) ^ self.buf[rel_y as usize][rel_x as usize];
            }
        }

        return collision;
    }
}