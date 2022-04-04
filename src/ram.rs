use std::io::Read;
use std::vec;

const MEM_BYTES: usize = 4096;
pub const PROG_MEM_START: usize = 0x200;
const FONT_COUNT: usize = 80;

const FONT_SET: [u8; FONT_COUNT] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

pub struct RAM {
    memory: Vec<u8>,
}

impl RAM {
    pub fn set_memory(&mut self, val: u8, index: usize) -> Result<(), String> {
        if index > MEM_BYTES {
            Err("Memory index out of bounds".to_string())
        } else {
            self.memory.insert(index, val);
            Ok(())
        }
    }

    pub fn read_memory(&self, addr: usize) -> u8 {
        *self.memory.get(addr).unwrap()
    }

    pub fn get_instruction(&self, addr: u16) -> u16 {
        (u16::from(self.read_memory(addr as usize)) << 8) | u16::from(self.read_memory((addr as usize) + 1))
    }

    pub fn new_ram() -> RAM {
        let mut r = RAM {
            memory: vec![0; MEM_BYTES],
        };

        r.load_font();
        r
    }

    pub fn load_rom(&mut self, mut f: std::fs::File) {
        let mut buffer = [0; MEM_BYTES];
        f.read(&mut buffer).unwrap();

        let mut i: usize = 0;
        for pos in PROG_MEM_START..MEM_BYTES {
            let mem_val = *buffer.get(i).unwrap();
            match self.set_memory(mem_val, pos) {
                Ok(_) => println!("Set memory address {:?} to value {:?}", pos, mem_val),
                Err(e) => println!(
                    "Could not set ROM into memory. Data may be corrupted. Error: {:?}",
                    e
                ),
            };
            i += 1;
        }
    }

    fn load_font(&mut self) {
        let mut i = 0;
        for c in FONT_SET {
            match self.set_memory(c, i) {
                Ok(_) => continue,
                Err(e) => println!("Encountered error {:?} while loading font.", e),
            };
            i += 1;
        }
    }
}
