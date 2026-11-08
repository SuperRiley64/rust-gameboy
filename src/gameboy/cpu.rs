pub struct CPU {
    pub a: u8,
    pub f: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,
    pub sp: u16,
    pub pc: u16,
    pub memory: [u8; 0x10000],
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            a: 0,
            f: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            h: 0,
            l: 0,
            sp: 0,
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

                self.f = 0;

                // Set Zero Flag (bit 7): 1 if result is 0
                if result == 0 {
                    self.f |= 0x80;
                }

                // Set Half Carry Flag (bit 5): 1 if lower 4 bits caused carry
                if (self.a & 0x0F) >= 0x08 {
                    self.f |= 0x20;
                }

                // Set Carry Flag (bit 4): 1 if overflow occurred
                if overflow {
                    self.f |= 0x10;
                }
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