use std::fs::File;

pub const DISPLAY_HEIGHT: usize = 32;
pub const DISPLAY_WIDTH: usize = 64;

pub struct Device {
    ram: crate::ram::RAM,
    vram: [[bool; DISPLAY_HEIGHT]; DISPLAY_WIDTH],
    cpu: crate::cpu::CPU,
}

impl Device {
    pub fn new_device() -> Device {
        Device {
            ram: crate::ram::RAM::new_ram(),
            vram: [[false; DISPLAY_HEIGHT]; DISPLAY_WIDTH],
            cpu: crate::cpu::CPU::new_cpu(),
        }
    }

    pub fn load_rom(&mut self, f: File) {
        self.ram.load_rom(f);
    }

    pub fn emulate_cycle(&mut self) {
        // Each instruction is 2 bytes
        // Fetch the next instruction from memory at the PC and increment it
        let opcode = self.ram.get_instruction(self.cpu.get_pc());
        self.cpu.increment_pc();

        // Decode
        let nnn = opcode & 0x0FFF;
        let nn = (opcode & 0x00FF) as u8;
        let nibbles = (
            (opcode & 0xF000) >> 12,
            (opcode & 0x0F00) >> 8,
            (opcode & 0x00F0) >> 4,
            (opcode & 0x000F),
        );

        // Execute
        match nibbles {
            (0x0, 0x0, 0xE, 0x0) => self.clear_screen(),
            (0x1, _, _, _) => self.jump_to(nnn),
            (0x6, x, _, _) => self.cpu.set_register(x as usize, nn),
            (0x7, x, _, _) => self.add_to_register(x, nn),
            (0xD, x, y, n) => self.update_vram(x, y, n as u8),
            _ => panic!("Unknown opcode {:?}", nibbles),
        }

        // Sleep to slow execution to a reasonable rate
        std::thread::sleep(std::time::Duration::new(0, 100000000));
    }

    // CLS - 00E0
    // Clear the screen of the display (set all the pixels to 'off')
    fn clear_screen(&mut self) {
        for row in self.vram {
            for mut pixel in row {
                pixel = false;
            }
        }
    }

    // 1NNN: Jump
    // This instruction should simply set PC to NNN, causing the program to jump to that memory
    // location. Do not increment the PC afterwards, it jumps directly there.
    fn jump_to(&mut self, nnn: u16) {
        self.cpu.set_pc(nnn)
    }

    // 7XNN: Add
    // Add the value NN to VX.
    fn add_to_register(&mut self, x: u16, nn: u8) {
        let vx = self.cpu.get_register(x as usize);
        self.cpu.set_register(x.into(), vx + nn);
    }

    // DXYN: Display
    fn update_vram(&mut self, mut x: u16, mut y: u16, n: u8) {
        // Set the X coordinate to the value in VX modulo 64
        let xc: usize = (self.cpu.get_register(x as usize) as usize % DISPLAY_WIDTH).try_into().unwrap();
        // Set the Y coordinate to the value in VY modulo 32
        let yc: usize = (self.cpu.get_register(y as usize) as usize % DISPLAY_HEIGHT).try_into().unwrap();

        // Set VF to 0
        self.cpu.set_register(0xF, 0);

        // For N rows...
        for i in 0..n {
            // Get the Nth byte of sprite data, counting from the memory address in the I register 
            // (I is not incremented)
            let nth_byte_spr_data = self.cpu.get_register((self.cpu.get_index_register() + i as u16).into());

            // For each of the 8 pixels/bits in this sprite row:
            for j in 0..8 {
                let spr_row_pixel = (nth_byte_spr_data >> i) & 1;
                // If the current pixel in the sprite row is on and the pixel at coordinates X,Y 
                // on the screen is also on, turn off the pixel and set VF to 1
                if spr_row_pixel != 0 && self.vram[xc][yc] {
                    self.vram[xc][yc] = false;
                    self.cpu.set_register(0xF, 1);
                }
                // Or if the current pixel in the sprite row is on and the screen pixel is not, 
                // draw the pixel at the X and Y coordinates
                else if spr_row_pixel != 0 && !(self.vram[xc][yc]) {
                    self.vram[xc][yc] = true;
                }
                // If you reach the right edge of the screen, stop drawing this row
                // FIXME

                // Increment X (VX is not incremented)
                x += 1;
            }
            // Increment Y (VY is not incremented)
            y += 1;

            // Stop if you reach the bottom edge of the screen
            // FIXME
        }
    }
}
