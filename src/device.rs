use std::fs::File;

use crate::stack;

pub const DISPLAY_HEIGHT: usize = 32;
pub const DISPLAY_WIDTH: usize = 64;

pub struct Device {
    ram: crate::ram::RAM,
    stack: stack::Stack,
    vram_changed: bool,                          // special flag bc i'm an idiot
    vram: [[u8; DISPLAY_WIDTH]; DISPLAY_HEIGHT], // access at vram[y][x]
    cpu: crate::cpu::CPU,
}

impl Device {
    pub fn new_device() -> Device {
        Device {
            ram: crate::ram::RAM::new(),
            stack: crate::stack::Stack::new(),
            vram_changed: false,
            vram: [[0; DISPLAY_WIDTH]; DISPLAY_HEIGHT],
            cpu: crate::cpu::CPU::new(),
        }
    }

    pub fn load_rom(&mut self, f: File) {
        self.ram.load_rom(f);
    }

    pub fn emulate_cycle(&mut self) {
        // Each instruction is 2 bytes
        // Fetch the next instruction from memory at the PC and increment it
        let opcode = self.ram.get_instruction(self.cpu.program_counter);
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
            (0x6, x, _, _) => self.cpu.general_registers[x as usize] = nn,
            (0x7, x, _, _) => self.add_to_register(x, nn),
            (0xA, _, _, _) => self.set_index(nnn),
            (0xD, x, y, n) => self.update_vram(x, y, n as u8),
            _ => panic!(
                "Unknown opcode ({:#01x} {:#01x} {:#01x} {:#01x})",
                nibbles.0, nibbles.1, nibbles.2, nibbles.3
            ),
        }

        // println!(
        //    "EXECUTED OPCODE: ({:#01x} {:#01x} {:#01x} {:#01x})",
        //    nibbles.0, nibbles.1, nibbles.2, nibbles.3
        //);

        // Sleep to slow execution to a reasonable rate
        std::thread::sleep(std::time::Duration::new(0, 10000000));
    }

    // Get the device's VRAM
    pub fn get_vram(&mut self) -> &[[u8; DISPLAY_WIDTH]; DISPLAY_HEIGHT] {
        self.vram_changed = false;
        &self.vram
    }

    // Get status of VRAM
    pub fn get_vram_changed(&self) -> bool {
        self.vram_changed
    }

    ///////////////////////////////////////////////////////////////////////////////////////////////
    // OPCODES ////////////////////////////////////////////////////////////////////////////////////
    ///////////////////////////////////////////////////////////////////////////////////////////////

    // CLS - 00E0
    // Clear the screen of the display (set all the pixels to 'off')
    fn clear_screen(&mut self) {
        for x in self.vram {
            for mut y in x {
                y = 0;
            }
        }
    }

    // 00EE: Subroutine return
    // Returning from a subroutine is done with 00EE, and it does this by removing (“popping”)
    // the last address from the stack and setting the PC to it.
    fn return_from_subroutine(&mut self) {
        self.cpu.program_counter = self.stack.pop();
    }

    // 1NNN: Jump
    // This instruction should simply set PC to NNN, causing the program to jump to that memory
    // location. Do not increment the PC afterwards, it jumps directly there.
    fn jump_to(&mut self, nnn: u16) {
        self.cpu.program_counter = nnn
    }

    // 2NNN: Subroutine
    // Calls the subroutine at memory location NNN. In other words, just like 1NNN,
    // you should set PC to NNN. However, the difference between a jump and a call is that this
    // instruction should first should push the current PC to the stack, so the subroutine can return 
    // later.
    fn subroutine(&mut self, nnn: u16) {
        self.stack.push(self.cpu.program_counter);
        self.cpu.program_counter = nnn;
    }

    // 7XNN: Add
    // Add the value NN to VX.
    fn add_to_register(&mut self, x: u16, nn: u8) {
        let vx = self.cpu.general_registers[x as usize];
        self.cpu.general_registers[x as usize] = vx + nn;
    }

    // ANNN: Set Index
    // This sets the index register I to the value NNN.
    fn set_index(&mut self, nnn: u16) {
        self.cpu.index_register = nnn;
    }

    // DXYN: Display
    // The interpreter reads n bytes from memory, starting at the address
    // stored in I. These bytes are then displayed as sprites on screen at
    // coordinates (Vx, Vy). Sprites are XORed onto the existing screen.
    // If this causes any pixels to be erased, VF is set to 1, otherwise
    // it is set to 0. If the sprite is positioned so part of it is outside
    // the coordinates of the display, it wraps around to the opposite side
    // of the screen.
    fn update_vram(&mut self, mut x: u16, mut y: u16, n: u8) {
        self.cpu.general_registers[0xF] = 0;
        for byte in 0..n {
            let y = (self.cpu.general_registers[y as usize] + byte) as usize % DISPLAY_HEIGHT;
            for bit in 0..8 {
                let x = (self.cpu.general_registers[x as usize] + bit) as usize % DISPLAY_WIDTH;
                let fill = (self
                    .ram
                    .read_memory((self.cpu.index_register + byte as u16).into())
                    >> (7 - bit))
                    & 1;
                self.cpu.general_registers[0xF] |= fill & self.vram[y][x];
                self.vram[y][x] ^= fill;
            }
        }
        self.vram_changed = true;
    }
}
