use std::fs::File;

const DISPLAY_HEIGHT: usize = 32;
const DISPLAY_WIDTH: usize = 64;

pub struct Device {
    ram: crate::ram::RAM,
    display: [[bool; DISPLAY_HEIGHT]; DISPLAY_WIDTH],
    cpu: crate::cpu::CPU,
}

impl Device {
    pub fn new_device() -> Device {
        Device {
            ram: crate::ram::RAM::new_ram(),
            display: [[false; DISPLAY_HEIGHT]; DISPLAY_WIDTH],
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
        let nn = opcode & 0x00FF;
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
            _ => panic!("Unknown opcode {:?}", nibbles),
        }

        // Sleep to slow execution to a reasonable rate
        std::thread::sleep(std::time::Duration::new(0, 100000000));
    }

    // CLS - 00E0
    // Clear the screen of the display (set all the pixels to 'off')
    fn clear_screen(&mut self) {
        for row in self.display {
            for mut pixel in row {
                pixel = false;
            }
        }
    }

    //
    // 1NNN: Jump
    // This instruction should simply set PC to NNN, causing the program to jump to that memory
    // location. Do not increment the PC afterwards, it jumps directly there.
    fn jump_to(&mut self, nnn: u16) {
        self.cpu.set_pc(nnn)
    }

    // 6XNN: Set
    // Simply set the register VX to the value NN.
    fn set_vx(&mut self, nn: u16) {
    }
}
