const STACK_SIZE: usize = 16;

struct Stack {
    stack_pointer: u16,
    s: Vec<u16>,
}

impl Stack {
    fn new_stack() -> Stack {
        Stack {
            stack_pointer: 0,
            s: Vec::with_capacity(STACK_SIZE),
        }
    }

    fn push(&mut self, v: u16) {
        self.s.push(v);
        self.stack_pointer += 1;
    }

    fn pop(&mut self) -> u16 {
        let v = self.s.pop().unwrap();
        self.stack_pointer -= 1;
        v
    }
}