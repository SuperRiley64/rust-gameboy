pub struct CPU {
    pub a: u8,
    pub pc: u16,
    pub memory: [u8; 0x10000],
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            a: 0,
            pc: 0,
            memory: [0; 0x10000],
        }
    }

    pub fn cycle(&mut self) {
        let opcode = self.memory[self.pc as usize];
        self.pc += 1;

        match opcode {
            0x3E => { // LD A, n
                self.a = self.memory[self.pc as usize];
                self.pc += 1;
            }
            0x87 => { // ADD A, A
                let (result, overflow) = self.a.overflowing_add(self.a);
                self.a = result;
                // Flags not implemented (simplified)
            }
            0xC3 => { // JP nn
                let addr = self.memory[self.pc as usize] as u16 | 
                           (self.memory[(self.pc + 1) as usize] as u16) << 8;
                self.pc = addr;
            }
            _ => {} // Unimplemented
        }
    }
}
