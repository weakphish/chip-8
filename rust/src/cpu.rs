use crate::{ram::RAM, stack::Stack, DISPLAY_HEIGHT, DISPLAY_WIDTH};
use rand::{prelude::ThreadRng, Rng};
use winit::event::DeviceEvent;
use winit::event::Event;
use winit::event::VirtualKeyCode;
use winit::event_loop::ControlFlow;
use winit_input_helper::WinitInputHelper;

const NUM_REGISTERS: usize = 16;

pub struct CPU {
    pub general_registers: [u8; NUM_REGISTERS],
    pub delay_timer: u8,
    pub sound_timer: u8,
    pub program_counter: u16,
    pub index_register: u16,
    pub rng: ThreadRng,
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            program_counter: crate::ram::PROG_MEM_START as u16,
            general_registers: [0; NUM_REGISTERS],
            delay_timer: 0,
            sound_timer: 0,
            index_register: 0,
            rng: rand::thread_rng(),
        }
    }

    pub fn increment_pc(&mut self) {
        self.program_counter += 2;
    }

    pub fn lookup_key_code(&self, value: u8) -> VirtualKeyCode {
        match value {
            0x0 => VirtualKeyCode::Key1,
            0x1 => VirtualKeyCode::Key2,
            0x2 => VirtualKeyCode::Key3,
            0x3 => VirtualKeyCode::C,
            0x4 => VirtualKeyCode::Key4,
            0x5 => VirtualKeyCode::Key5,
            0x6 => VirtualKeyCode::Key6,
            0x7 => VirtualKeyCode::D,
            0x8 => VirtualKeyCode::Key7,
            0x9 => VirtualKeyCode::Key8,
            0xA => VirtualKeyCode::Key9,
            0xB => VirtualKeyCode::E,
            0xC => VirtualKeyCode::A,
            0xD => VirtualKeyCode::Key0,
            0xE => VirtualKeyCode::B,
            0xF => VirtualKeyCode::F,
            _ => VirtualKeyCode::F1,
        }
    }

    pub fn emulate_cycle<T>(
        &mut self,
        event: &Event<T>,
        stack: &mut Stack,
        vram: &mut [[u8; DISPLAY_WIDTH]; DISPLAY_HEIGHT],
        ram: &mut RAM,
        control_flow: &mut ControlFlow
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
            (0x8, x, y, 0) => self.op_set_vx_to_vy(x, y),
            (0x8, x, y, 0x1) => self.op_binary_or(x, y),
            (0x8, x, y, 0x2) => self.op_binary_and(x, y),
            (0x8, x, y, 0x3) => self.op_logical_xor(x, y),
            (0x8, x, y, 0x4) => self.op_add_with_carry(x, y),
            (0x8, x, y, 0x5) => self.op_subtract_vy_from_vx(x, y),
            (0x8, x, y, 0x7) => self.op_subtract_vx_from_vy(x, y),
            (0x8, x, y, 6) => self.op_shift_right(x, y),
            (0x8, x, y, 0xE) => self.op_shift_left(x, y),
            (0x9, x, y, _) => self.op_skip_if_not_eq_reg(x, y),
            (0xA, _, _, _) => self.op_set_index(nnn),
            (0xB, _, _, _) => self.op_jump_location_plus_reg(nnn),
            (0xC, x, _, _) => self.op_rand_and(x, nn),
            (0xD, x, y, n) => self.op_display_vram(vram, ram, x, y, n as u8),
            (0xE, x, 0x9, 0xE) => self.op_skip_if_pressed(&event, x),
            (0xF, x, 0x0, 0xA) => {
                *control_flow = ControlFlow::Wait;
            }
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

    // 8XY5: Subtract (VX - VY into VX)
    // These both subtract the value in one register from the other, and put the result in VX.
    // In both cases, VY is not affected.
    //
    // This subtraction will also affect the carry flag, but note that it’s opposite from what you
    // might think. If the minuend (the first operand) is larger than the subtrahend
    // (second operand) VF will be set to 1. If the subtrahend is larger, and we “underflow” the
    // result, VF is set to 0. Another way of thinking of it is that VF is set to 1 before the
    // subtraction, and then the subtraction either borrows from VF (setting it to 0) or not.
    fn op_subtract_vy_from_vx(&mut self, x: u16, y: u16) {
        let vx = self.general_registers[x as usize];
        let vy = self.general_registers[y as usize];

        if vx > vy {
            self.general_registers[0xF] = 1;
        } else {
            self.general_registers[0xF] = 0;
        }

        self.general_registers[x as usize] = vx - vy;
    }
    // 8XY7: Subtract (VY - VX into VX)
    fn op_subtract_vx_from_vy(&mut self, x: u16, y: u16) {
        let vx = self.general_registers[x as usize];
        let vy = self.general_registers[y as usize];

        if vy > vx {
            self.general_registers[0xF] = 1;
        } else {
            self.general_registers[0xF] = 0;
        }

        self.general_registers[x as usize] = vy - vx;
    }

    // 8XY6 and 8XYE: Shift group
    // In the CHIP-8 interpreter for the original COSMAC VIP, this instruction did the following:
    // It put the value of VY into VX, and then shifted the value in VX 1 bit to the right (8XY6)
    // or left (8XYE). VY was not affected, but the flag register VF would be set to the bit that
    // was shifted out.
    //
    // However, starting with CHIP-48 and SUPER-CHIP in the early 1990s, these instructions were
    // changed so that they shifted VX in place, and ignored the Y completely.
    //
    // Step by step:
    // 1. (Optional, or configurable) Set VX to the value of VY
    // 2. Shift the value of VX one bit to the right (8XY6) or left (8XYE)
    // 3. Set VF to 1 if the bit that was shifted out was 1, or 0 if it was 0
    fn op_shift_right(&mut self, x: u16, _y: u16) {
        let vx = self.general_registers[x as usize];
        self.general_registers[x as usize] = vx >> 1;
    }

    fn op_shift_left(&mut self, x: u16, _y: u16) {
        let vx = self.general_registers[x as usize];
        self.general_registers[x as usize] = vx << 1;
    }

    // ANNN: Set Index
    // This sets the index register I to the value NNN.
    fn op_set_index(&mut self, nnn: u16) {
        self.index_register = nnn;
    }

    // BNNN - JP V0, addr
    // Jump to location nnn + V0.
    fn op_jump_location_plus_reg(&mut self, nnn: u16) {
        self.program_counter = (self.general_registers[0] as u16) + nnn;
    }

    // CXKK - RND Vx, byte
    // Set Vx = random byte AND kk.
    // The interpreter generates a random number from 0 to 255, which is then ANDed with the value
    // kk. The results are stored in Vx. See instruction 8xy2 for more information on AND.
    fn op_rand_and(&mut self, x: u16, nn: u8) {
        let rand: u8 = self.rng.gen();
        self.general_registers[x as usize] = rand & nn;
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

    // Skip if key group
    // Like the earlier skip instructions, these two also skip the following instruction based on
    // a condition. These skip based on whether the player is currently pressing a key or not.
    // These instructions (unlike the later FX0A) don’t wait for input, they just check if the key
    // is currently being held down.

    // EX9E: Skip if pressed
    // Will skip one instruction (increment PC by 2) if the key corresponding to the value in VX
    // is pressed.
    fn op_skip_if_pressed<T>(&mut self, e: &Event<T>, x: u16) {
        let queried_key = self.lookup_key_code(self.general_registers[x as usize]);
        //if event.key_pressed(queried_key) {
        //    self.increment_pc();
        //}

        if let Event::DeviceEvent { device_id, event } = e {
            if let DeviceEvent::Key(keyboard_stuff) = event {
                if let Some(virtual_key_code) = keyboard_stuff.virtual_keycode {
                    if virtual_key_code == queried_key {
                        self.increment_pc();
                    }
                }
            }
        }
        
    }

    /// EXA1: Skips if the key corresponding to the value in VX is not pressed.
    fn op_skip_if_not_pressed<T>(&mut self, e: &Event<T>, x: u16) {
        let queried_key = self.lookup_key_code(self.general_registers[x as usize]);
        //if !input.key_pressed(queried_key) {
        //    self.increment_pc();
        //}

        if let Event::DeviceEvent { device_id, event} = e {
            if let DeviceEvent::Key(keyboard_stuff) = event {
                if let Some(virtual_key_code) = keyboard_stuff.virtual_keycode {
                    if virtual_key_code == queried_key {
                        return;
                    }
                }
            }
        }
        self.increment_pc();
    }

    /// FX07: Sets VX to the current value of the delay timer
    fn op_set_to_delay(&mut self, x: u16) {
        self.general_registers[x as usize] = self.delay_timer;
    }

    /// FX15: Sets delay timer to VX
    fn op_set_delay_to(&mut self, x: u16) {
        self.delay_timer = self.general_registers[x as usize];
    }

    /// FX18: Sets sound timer to VX
    fn op_set_sound_to(&mut self, x: u16) {
        self.sound_timer = self.general_registers[x as usize];
    }

    /// FX1E: Add to index
    /// Add VX to the index register I
    fn op_add_to_index(&mut self, x: u16) {
        let tmp_i = self.index_register;
        let (result, flag) = tmp_i.overflowing_add(self.general_registers[x as usize].into());
        self.general_registers[0xF] = u8::from(flag);
        self.index_register = result;
    }

    /// FX0A: Get key
    /// "Blocks" and waits for key input.
    /// This instruction “blocks”; it stops executing instructions and waits for key input (or loops forever, unless a key is pressed).
    /// In other words, if you followed my advice earlier and increment PC after fetching each instruction, then it should be decremented again here unless a key is pressed. 
    /// Otherwise, PC should simply not be incremented.
    /// Although this instruction stops the program from executing further instructions, the timers (delay timer and sound timer) should still be decreased while it’s waiting.
    /// If a key is pressed while this instruction is waiting for input, its hexadecimal value will be put in VX and execution continues.
    /// On the original COSMAC VIP, the key was only registered when it was pressed and then released.
    fn op_get_key<T>(&self, e: &Event<T>) {
        if let Event::DeviceEvent { device_id, event } = e {
            if let DeviceEvent::Key(keyboard_stuff) = event {
                if let Some(virtual_key_code) = keyboard_stuff.virtual_keycode {

                }
            }
        }
    }
}
