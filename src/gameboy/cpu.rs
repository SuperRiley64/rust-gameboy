#[derive(Debug, Clone, Copy)]
pub enum Register16{
    BC,
    DE,
    HL,
    SP,
}

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
    pub memory: [u8; 0x10000]
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

    // Runs the fetch decode execute cycle
    pub fn cycle(&mut self) {
        let opcode = self.memory[self.pc as usize];

        match opcode {
            0x00 => { self.pc += 1} // NOP
             
            // 8-bit load immediate instructions
            0x3E => { self.a = self.memory[(self.pc as usize) + 1]; self.pc += 2; } // LD A, n
            0x06 => { self.b = self.memory[(self.pc as usize) + 1]; self.pc += 2; } // LD B, n
            0x0E => { self.c = self.memory[(self.pc as usize) + 1]; self.pc += 2; } // LD C, n
            0x16 => { self.d = self.memory[(self.pc as usize) + 1]; self.pc += 2; } // LD D, n
            0x1E => { self.e = self.memory[(self.pc as usize) + 1]; self.pc += 2; } // LD E, n
            0x26 => { self.h = self.memory[(self.pc as usize) + 1]; self.pc += 2; } // LD H, n
            0x2E => { self.l = self.memory[(self.pc as usize) + 1]; self.pc += 2; } // LD L, n

            // 16-bit load immediate instructions
            0x01 => { let val = self.get_16_bit_operand(self.sp); self.assign_16bit_register(Register16::BC, val); self.pc += 3; } // LD BC, nn
            0x11 => { let val = self.get_16_bit_operand(self.sp); self.assign_16bit_register(Register16::DE, val); self.pc += 3; } // LD DE, nn
            0x21 => { let val = self.get_16_bit_operand(self.sp); self.assign_16bit_register(Register16::HL, val); self.pc += 3; } // LD HL, nn
            0x31 => { let val = self.get_16_bit_operand(self.sp); self.assign_16bit_register(Register16::SP, val); self.pc += 3; } // LD SP, nn

            // 8-bit load register instructions
            0x78 => { self.a = self.b; self.pc += 1; } // LD A, B
            0x79 => { self.a = self.c; self.pc += 1; } // LD A, C
            0x7A => { self.a = self.d; self.pc += 1; } // LD A, D
            0x7B => { self.a = self.e; self.pc += 1; } // LD A, E
            0x7C => { self.a = self.h; self.pc += 1; } // LD A, H
            0x7D => { self.a = self.l; self.pc += 1; } // LD A, L

            // 8-bit arithmetic instructions
            0x87 => { self.a = self.add_8_bit(self.a, self.a); self.pc += 1; } // ADD A, A

            // Jump instructions
            0xC3 => {self.pc = self.get_16_bit_operand(self.sp)} // JP nn

            // 8-bit increment instructions
            0x3C => { self.a = self.increment_8bit_register(self.a); self.pc += 1; } // INC A
            0x04 => { self.b = self.increment_8bit_register(self.b); self.pc += 1; } // INC B
            0x0C => { self.c = self.increment_8bit_register(self.c); self.pc += 1; } // INC C
            0x14 => { self.d = self.increment_8bit_register(self.d); self.pc += 1; } // INC D
            0x1C => { self.e = self.increment_8bit_register(self.e); self.pc += 1; } // INC E
            0x24 => { self.h = self.increment_8bit_register(self.h); self.pc += 1; } // INC H
            0x2C => { self.l = self.increment_8bit_register(self.l); self.pc += 1; } // INC L

            // 8-bit decrement instructions
            0x3D => { self.a = self.decrement_8bit_register(self.a); self.pc += 1; } // DEC A
            0x05 => { self.b = self.decrement_8bit_register(self.b); self.pc += 1; } // DEC B
            0x0D => { self.c = self.decrement_8bit_register(self.c); self.pc += 1; } // DEC C
            0x15 => { self.d = self.decrement_8bit_register(self.d); self.pc += 1; } // DEC D
            0x1D => { self.e = self.decrement_8bit_register(self.e); self.pc += 1; } // DEC E
            0x25 => { self.h = self.decrement_8bit_register(self.h); self.pc += 1; } // DEC H
            0x2D => { self.l = self.decrement_8bit_register(self.l); self.pc += 1; } // DEC L

            // 16-bit increment instructions
            0x03 => { self.increment_16_bit_register(self.b, self.c, Register16::BC); self.pc += 2; } // INC BC
            0x13 => { self.increment_16_bit_register(self.d, self.e, Register16::DE); self.pc += 2; } // INC DE
            0x23 => { self.increment_16_bit_register(self.h, self.l, Register16::HL); self.pc += 2; } // INC HL
            0x33 => { let high = (self.sp >> 8) as u8; let low = (self.sp & 0xFF) as u8; self.increment_16_bit_register(high, low, Register16::SP); self.pc += 2; } // INC SP

            // 16-bit decrement instructions
            0x0B => { self.decrement_16_bit_register(self.b, self.c, Register16::BC); self.pc += 2; } // DEC BC
            0x1B => { self.decrement_16_bit_register(self.d, self.e, Register16::DE); self.pc += 2; } // DEC DE
            0x2B => { self.decrement_16_bit_register(self.h, self.l, Register16::HL); self.pc += 2; } // DEC HL
            0x3B => { let high = (self.sp >> 8) as u8; let low = (self.sp & 0xFF) as u8; self.decrement_16_bit_register(high, low, Register16::SP); self.pc += 2; }
            _ => { self.pc += 1; } // Unimplemented
        }
    }

    // ============= HELPER FUNCTIONS ==========================

    // Helper method to increment a register and update flags
    fn increment_8bit_register(&mut self, reg: u8) -> u8 {
        let result = reg.wrapping_add(1);

        // Preserve carry flag
        self.f &= 0x10;

        if result == 0 {
            self.f |= 0x80; // Zero flag
        }

        if (reg & 0x0F) == 0x0F {
            self.f |= 0x20; // Half-carry flag
        }

        // Clear subtract flag
        self.f &= !0x40;

        result
    }

    // Helper method to decrement a register and update flags
    fn decrement_8bit_register(&mut self, reg: u8) -> u8 {
        let result = reg.wrapping_sub(1);

        // Preserve carry flag
        self.f &= 0x10;

        if result == 0 {
            self.f |= 0x80; // Zero flag
        } else {
            self.f &= !0x80;
        }

        if (reg & 0x0F) == 0 {
            self.f |= 0x20; // Half-carry flag
        } else {
            self.f &= !0x20;
        }

        // Set subtract flag
        self.f |= 0x40;

        result
    }

    // Helper method to assign 16-bit register from a u16
    fn assign_16bit_register(&mut self, reg: Register16, value: u16) {
        match reg {
            Register16::BC => {
                self.b = (value >> 8) as u8;   // High byte goes to `b`
                self.c = (value & 0xFF) as u8; // Low byte goes to `c`
            },
            Register16::DE => {
                self.d = (value >> 8) as u8;   // High byte goes to `d`
                self.e = (value & 0xFF) as u8; // Low byte goes to `e`
            },
            Register16::HL => {
                self.h = (value >> 8) as u8;   // High byte goes to `h`
                self.l = (value & 0xFF) as u8; // Low byte goes to `l`
            },
            Register16::SP => self.sp = value, // sp is 16 bit so can be assigned directly
        }
    }

    // Helper method for 8-bit add instructions
    //TODO: fix flag updates
    fn add_8_bit(&mut self, reg1: u8, reg2: u8) -> u8 {
        let (result, overflow) = reg1.overflowing_add(reg2);

        // Set Zero Flag (bit 7): 1 if result is 0
        // Set Half Carry Flag (bit 5): 1 if lower 4 bits caused carry
        // Set Carry Flag (bit 4): 1 if overflow occurred
        // Clear Subtract Flag (bit 6)
        self.f = 0;

        if result == 0 {
            self.f |= 0x80;
        }

        if (reg1 & 0x0F) >= 0x08 {
            self.f |= 0x20;
        }

        if overflow {
            self.f |= 0x10;
        }

        result
    }

    // Helper method to increment a 16-bit register and update flags
    //TODO: Fix flag updates
    fn increment_16_bit_register(
        &mut self,
        high_byte: u8,
        low_byte: u8,
        reg: Register16,
    ) {
        let value = ((high_byte as u16) << 8) | low_byte as u16;
        let new_value = value.wrapping_add(1);

        // Assign the updated value to the register.
        self.assign_16bit_register(reg, new_value);

        // 16-bit inc and dec on the SM83 do not modify flags.
    }

    // Helper method to increment a 16-bit register
    fn decrement_16_bit_register(
        &mut self,
        high_byte: u8,
        low_byte: u8,
        reg: Register16,
    ) {
        let value = ((high_byte as u16) << 8) | low_byte as u16;
        let new_value = value.wrapping_sub(1);

        // Assign the updated value to the register.
        self.assign_16bit_register(reg, new_value);

        // 16-bit inc and dec on the SM83 do not modify flags.
    }

    fn get_16_bit_operand(&mut self, index: u16) -> u16 {
        self.memory[(index + 1) as usize] as u16 | (self.memory[(index + 2) as usize] as u16) << 8
    }

    
}