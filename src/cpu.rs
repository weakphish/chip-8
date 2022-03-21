const NUM_REGISTERS: usize = 16;

pub struct CPU {
    general_registers: [u8; NUM_REGISTERS],
    delay_timer: u8,
    sound_timer: u8,
    program_counter: u16,
    index_register: u16,
}

impl CPU {
    pub fn new_cpu() -> CPU {
        CPU {
            program_counter: crate::ram::PROG_MEM_START as u16,
            general_registers: [0; NUM_REGISTERS],
            delay_timer: 0,
            sound_timer: 0,
            index_register: 0,
        }
    }

    pub fn get_pc(&self) -> u16 {
        self.program_counter
    }

    pub fn increment_pc(&mut self) {
        self.program_counter += 1;
    }

    pub fn set_pc(&mut self, val: u16) {
        self.program_counter = val;
    }
}
