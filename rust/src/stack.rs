use log::{info, warn};

const STACK_SIZE: usize = 16;

pub struct Stack {
    stack_pointer: u16,
    s: Vec<u16>,
}

impl Stack {
    pub fn new() -> Stack {
        Stack {
            stack_pointer: 0,
            s: Vec::with_capacity(STACK_SIZE),
        }
    }

    pub fn push(&mut self, v: u16) {
        self.s.push(v);
        self.stack_pointer += 1;
    }

    pub fn pop(&mut self) -> u16 {
        let v = match self.s.pop() {
            Some(x) => x,
            None => {
                warn!("The stack just tried to pop an empty list");
                0
            }
        };
        self.stack_pointer -= 1;
        v
    }
}
