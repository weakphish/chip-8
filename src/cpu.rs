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
}
