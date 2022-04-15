use crate::{ram::RAM, stack::Stack, DISPLAY_HEIGHT, DISPLAY_WIDTH};

const NUM_REGISTERS: usize = 16;

pub struct CPU {
    pub general_registers: [u8; NUM_REGISTERS],
    pub delay_timer: u8,
    pub sound_timer: u8,
    pub program_counter: u16,
    pub index_register: u16,
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            program_counter: crate::ram::PROG_MEM_START as u16,
            general_registers: [0; NUM_REGISTERS],
            delay_timer: 0,
            sound_timer: 0,
            index_register: 0,
        }
    }

    pub fn increment_pc(&mut self) {
        self.program_counter += 2;
    }

    pub fn emulate_cycle(
        &mut self,
        stack: &mut Stack,
        vram: &mut [[u8; DISPLAY_WIDTH]; DISPLAY_HEIGHT],
        ram: &mut RAM,
    ) {
        // Each instruction is 2 bytes
        // Fetch the next instruction from memory at the PC and increment it
        let opcode = ram.get_instruction(self.program_counter);
        self.increment_pc();

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
            (0x0, 0x0, 0xE, 0x0) => self.op_clear_screen(vram),
            (0x0, 0x0, 0xE, 0xE) => self.op_return_from_subroutine(stack),
            (0x1, _, _, _) => self.op_jump(nnn),
            (0x2, _, _, _) => self.op_subroutine(stack, nnn),
            (0x3, x, _, _) => self.op_skip_if_eq(x, nn.into()),
            (0x4, x, _, _) => self.op_skip_if_not_eq(x, nn.into()),
            (0x6, x, _, _) => self.general_registers[x as usize] = nn,
            (0x5, x, y, _) => self.op_skip_if_eq_reg(x, y),
            (0x7, x, _, _) => self.op_add(x, nn),
            (0x9, x, y, _) => self.op_skip_if_not_eq_reg(x, y),
            (0xA, _, _, _) => self.op_set_index(nnn),
            (0xD, x, y, n) => self.op_display_vram(vram, ram, x, y, n as u8),
            _ => panic!(
                "Unknown opcode ({:#01x} {:#01x} {:#01x} {:#01x})",
                nibbles.0, nibbles.1, nibbles.2, nibbles.3
            ),
        }

        // Sleep to slow execution to a reasonable rate
        std::thread::sleep(std::time::Duration::new(0, 1000));
    }

    ///////////////////////////////////////////////////////////////////////////////////////////////
    // OPCODES ////////////////////////////////////////////////////////////////////////////////////
    ///////////////////////////////////////////////////////////////////////////////////////////////

    // CLS - 00E0
    // Clear the screen of the display (set all the pixels to 'off')
    fn op_clear_screen(&mut self, vram: &mut [[u8; DISPLAY_WIDTH]; DISPLAY_HEIGHT]) {
        for x in vram {
            for y in x {
                *y = 0;
            }
        }
    }

    // 00EE: Subroutine return
    // Returning from a subroutine is done with 00EE, and it does this by removing (“popping”)
    // the last address from the stack and setting the PC to it.
    fn op_return_from_subroutine(&mut self, stack: &mut Stack) {
        self.program_counter = stack.pop();
    }

    // 1NNN: Jump
    // This instruction should simply set PC to NNN, causing the program to jump to that memory
    // location. Do not increment the PC afterwards, it jumps directly there.
    fn op_jump(&mut self, nnn: u16) {
        self.program_counter = nnn
    }

    // 2NNN: Subroutine
    // Calls the subroutine at memory location NNN. In other words, just like 1NNN,
    // you should set PC to NNN. However, the difference between a jump and a call is that this
    // instruction should first should push the current PC to the stack, so the subroutine can return
    // later.
    fn op_subroutine(&mut self, stack: &mut Stack, nnn: u16) {
        stack.push(self.program_counter);
        self.program_counter = nnn;
    }

    // Skip group. These instructions do the same thing: They either do nothing, or they skip one
    // two-byte instruction (increment PC by 2). If you didn’t increment PC in the “fetch” stage
    // above, they will obviously increment PC by either 4 or 2.
    //
    // 3XNN will skip one instruction if the value in VX is equal to NN
    fn op_skip_if_eq(&mut self, x: u16, nn: u16) {
        let vx: u16 = self.general_registers[x as usize].into();
        if vx == nn {
            self.increment_pc();
        }
    }
    // 4XNN will skip if they are not equal.
    fn op_skip_if_not_eq(&mut self, x: u16, nn: u16) {
        let vx: u16 = self.general_registers[x as usize].into();
        if vx != nn {
            self.increment_pc();
        }
    }

    // 5XY0 skips if the values in VX and VY are equal
    fn op_skip_if_eq_reg(&mut self, x: u16, y: u16) {
        if self.general_registers[x as usize] == self.general_registers[y as usize] {
            self.increment_pc();
        }
    }
    // ... while 9XY0 skips if they are not equal.
    fn op_skip_if_not_eq_reg(&mut self, x: u16, y: u16) {
            if self.general_registers[x as usize] != self.general_registers[y as usize] {
            self.increment_pc();
        }
    }

    // 7XNN: Add
    // Add the value NN to VX.
    fn op_add(&mut self, x: u16, nn: u8) {
        let vx = self.general_registers[x as usize];
        self.general_registers[x as usize] = vx + nn;
    }

    // 8XY0: Set
    // Set VX to the value of VY
    fn op_set_vx_to_vy(&mut self, x: u16, y: u16) {
        self.general_registers[x as usize] = self.general_registers[y as usize];
    }

    // 8XY1: Binary OR
    // VX is set to the bitwise/binary logical disjunction (OR) of VX and VY. VY is not affected.
    fn op_binary_or(&mut self, x: u16, y: u16) {
        let vx = self.general_registers[x as usize];
        let vy = self.general_registers[y as usize];

        self.general_registers[x as usize] = vx | vy;
    }

    // 8XY2: Binary AND
    // VX is set to AND of VX and VY
    fn op_binary_and(&mut self, x: u16, y: u16) {
        let vx = self.general_registers[x as usize];
        let vy = self.general_registers[y as usize];

        self.general_registers[x as usize] = vx & vy;
    }

    // 8XY3: Logical XOR
    // VX is set to the bitwise/binary exclusive OR (XOR) of VX and VY. 
    fn op_logical_xor(&mut self, x: u16, y: u16) {
        let vx = self.general_registers[x as usize];
        let vy = self.general_registers[y as usize];

        self.general_registers[x as usize] = vx ^ vy;
    }

    // 8XY4: Add
    // VX is set to the value of VX + VY
    // Unlike 7XNN, the carry flag is affected. If the result is > 255, the flag register VF is set
    // to 1. Otherwise, it is set to 0. 
    fn op_add_with_carry(&mut self, x: u16, y: u16) {
        let vx = self.general_registers[x as usize];
        let vy = self.general_registers[y as usize];

        let (result, flag) = vx.overflowing_add(vy);
        self.general_registers[x as usize] = result;
        self.general_registers[0xF] = u8::from(flag);
    }
   
    // ANNN: Set Index
    // This sets the index register I to the value NNN.
    fn op_set_index(&mut self, nnn: u16) {
        self.index_register = nnn;
    }

    // DXYN: Display
    // The interpreter reads n bytes from memory, starting at the address
    // stored in I. These bytes are then displayed as sprites on screen at
    // coordinates (Vx, Vy). Sprites are XORed onto the existing screen.
    // If this causes any pixels to be erased, VF is set to 1, otherwise
    // it is set to 0. If the sprite is positioned so part of it is outside
    // the coordinates of the display, it wraps around to the opposite side
    // of the screen.
    fn op_display_vram(
        &mut self,
        vram: &mut [[u8; DISPLAY_WIDTH]; DISPLAY_HEIGHT],
        ram: &RAM,
        x: u16,
        y: u16,
        n: u8,
    ) {
        self.general_registers[0xF] = 0;
        for byte in 0..n {
            let y = (self.general_registers[y as usize] + byte) as usize % DISPLAY_HEIGHT;
            for bit in 0..8 {
                let x = (self.general_registers[x as usize] + bit) as usize % DISPLAY_WIDTH;
                let fill =
                    (ram.read_memory((self.index_register + byte as u16).into()) >> (7 - bit)) & 1;
                self.general_registers[0xF] |= fill & vram[y][x];
                vram[y][x] ^= fill;
            }
        }
    }
}
