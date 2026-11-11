#[derive(Debug, Clone, Copy)]
pub enum Register8 {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
}

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
            0x00 => { self.pc += 1 } // NOP
            0x10 => { self.pc += 1 } // STOP //TODO: Handle RESET
            0x76 => { self.pc += 1 } // HALT //TODO: Interrupt/Reset
             
            // 8-bit transfer instructions
            0x3E => { self.a = self.memory[(self.pc as usize) + 1]; self.pc += 2; } // LD A, n
            0x06 => { self.b = self.memory[(self.pc as usize) + 1]; self.pc += 2; } // LD B, n
            0x0E => { self.c = self.memory[(self.pc as usize) + 1]; self.pc += 2; } // LD C, n
            0x16 => { self.d = self.memory[(self.pc as usize) + 1]; self.pc += 2; } // LD D, n
            0x1E => { self.e = self.memory[(self.pc as usize) + 1]; self.pc += 2; } // LD E, n
            0x26 => { self.h = self.memory[(self.pc as usize) + 1]; self.pc += 2; } // LD H, n
            0x2E => { self.l = self.memory[(self.pc as usize) + 1]; self.pc += 2; } // LD L, n

            0xFA => { self.ld_a_from_a16(); self.pc += 3; } // LD A, (a16)
            0xF0 => { self.ld_a_from_a8(); self.pc += 2; } // LD A, (a8)
            0x1A => { self.ld_a_from_de(); self.pc += 3; } // LD A, (DE)
            0x3A => { self.ld_a_from_hl_minus(); self.pc += 3; } // LD A, (HL-)
            0x2A => { self.ld_a_from_hl_plus(); self.pc += 3; } // LD A, (HL+)
            0x12 => { self.ld_memory_at_de_from_a(); self.pc += 1; } // LD (DE), A
            0x32 => { self.ld_memory_at_hl_minus_from_a(); self.pc += 3; } // LD (HL-), A
            0xEA => { self.ld_memory_at_a16_from_a(); self.pc += 3; } // LD (a16), A
            0xE2 => { self.ld_memory_at_c_from_a(); self.pc += 1; } // LD (C), A
            0xE0 => { self.ld_memory_at_a8_from_a(); self.pc += 2; } // LD (a8), A
            0x22 => { self.ld_memory_at_hl_plus_from_a(); self.pc += 3; } // LD (HL+), A
            0x36 => { self.ld_memory_at_hl_from_d8(); self.pc += 2; } // LD (HL), d8 
            
            0x70 => { self.memory[((self.h as u16) << 8 | self.l as u16) as usize] = self.b; self.pc += 1; } // LD (HL), B
            0x71 => { self.memory[((self.h as u16) << 8 | self.l as u16) as usize] = self.c; self.pc += 1; } // LD (HL), C
            0x72 => { self.memory[((self.h as u16) << 8 | self.l as u16) as usize] = self.d; self.pc += 1; } // LD (HL), D
            0x73 => { self.memory[((self.h as u16) << 8 | self.l as u16) as usize] = self.e; self.pc += 1; } // LD (HL), E
            0x74 => { self.memory[((self.h as u16) << 8 | self.l as u16) as usize] = self.h; self.pc += 1; } // LD (HL), H
            0x75 => { self.memory[((self.h as u16) << 8 | self.l as u16) as usize] = self.l; self.pc += 1; } // LD (HL), L
            0x7E => { self.ld_reg_from_hl(Register8::A); self.pc += 1; } // LD A, (HL)

            0x46 => { self.ld_reg_from_hl(Register8::B); self.pc += 1; } // LD B, (HL)
            0x56 => { self.ld_reg_from_hl(Register8::D); self.pc += 1; } // LD D, (HL)
            0x5E => { self.ld_reg_from_hl(Register8::E); self.pc += 1; } // LD E, (HL)
            0x66 => { self.ld_reg_from_hl(Register8::H); self.pc += 1; } // LD H, (HL)
            0x6E => { self.ld_reg_from_hl(Register8::L); self.pc += 1; } // LD L, (HL)

            // 16-bit load immediate instructions
            0x01 => { let val = self.get_16_bit_operand(self.sp); self.assign_16bit_register(Register16::BC, val); self.pc += 3; } // LD BC, nn
            0x11 => { let val = self.get_16_bit_operand(self.sp); self.assign_16bit_register(Register16::DE, val); self.pc += 3; } // LD DE, nn
            0x21 => { let val = self.get_16_bit_operand(self.sp); self.assign_16bit_register(Register16::HL, val); self.pc += 3; } // LD HL, nn
            0x31 => { let val = self.get_16_bit_operand(self.sp); self.assign_16bit_register(Register16::SP, val); self.pc += 3; } // LD SP, nn

            // 8-bit load register instructions
            0x7F => { self.ld_reg_to_reg(Register8::A, Register8::A); self.pc += 1; } // LD A, A
            0x78 => { self.ld_reg_to_reg(Register8::B, Register8::A); self.pc += 1; } // LD A, B
            0x79 => { self.ld_reg_to_reg(Register8::C, Register8::A); self.pc += 1; } // LD A, C
            0x7A => { self.ld_reg_to_reg(Register8::D, Register8::A); self.pc += 1; } // LD A, D
            0x7B => { self.ld_reg_to_reg(Register8::E, Register8::A); self.pc += 1; } // LD A, E
            0x7C => { self.ld_reg_to_reg(Register8::H, Register8::A); self.pc += 1; } // LD A, H
            0x7D => { self.ld_reg_to_reg(Register8::L, Register8::A); self.pc += 1; } // LD A, L

            0x40 => { self.ld_reg_to_reg(Register8::B, Register8::B); self.pc += 1; } // LD B, B
            0x47 => { self.ld_reg_to_reg(Register8::A, Register8::B); self.pc += 1; } // LD B, A
            0x41 => { self.ld_reg_to_reg(Register8::C, Register8::B); self.pc += 1; } // LD B, C
            0x42 => { self.ld_reg_to_reg(Register8::D, Register8::B); self.pc += 1; } // LD B, D
            0x43 => { self.ld_reg_to_reg(Register8::E, Register8::B); self.pc += 1; } // LD B, E
            0x44 => { self.ld_reg_to_reg(Register8::H, Register8::B); self.pc += 1; } // LD B, H
            0x45 => { self.ld_reg_to_reg(Register8::L, Register8::B); self.pc += 1; } // LD B, L

            0x4F => { self.ld_reg_to_reg(Register8::A, Register8::C); self.pc += 1; } // LD C, A
            0x48 => { self.ld_reg_to_reg(Register8::B, Register8::C); self.pc += 1; } // LD C, B
            0x49 => { self.ld_reg_to_reg(Register8::C, Register8::C); self.pc += 1; } // LD C, C
            0x4A => { self.ld_reg_to_reg(Register8::D, Register8::C); self.pc += 1; } // LD C, D
            0x4B => { self.ld_reg_to_reg(Register8::E, Register8::C); self.pc += 1; } // LD C, E
            0x4C => { self.ld_reg_to_reg(Register8::H, Register8::C); self.pc += 1; } // LD C, H
            0x4D => { self.ld_reg_to_reg(Register8::L, Register8::C); self.pc += 1; } // LD C, L

            0x85 => { self.ld_reg_to_reg(Register8::A, Register8::D); self.pc += 1; } // LD D, A
            0x50 => { self.ld_reg_to_reg(Register8::B, Register8::D); self.pc += 1; } // LD D, B
            0x51 => { self.ld_reg_to_reg(Register8::C, Register8::D); self.pc += 1; } // LD D, C
            0x52 => { self.ld_reg_to_reg(Register8::D, Register8::D); self.pc += 1; } // LD D, D
            0x53 => { self.ld_reg_to_reg(Register8::E, Register8::D); self.pc += 1; } // LD D, E
            0x54 => { self.ld_reg_to_reg(Register8::H, Register8::D); self.pc += 1; } // LD D, H
            0x55 => { self.ld_reg_to_reg(Register8::L, Register8::D); self.pc += 1; } // LD D, L

            0x5F => { self.ld_reg_to_reg(Register8::A, Register8::E); self.pc += 1; } // LD E, A
            0x58 => { self.ld_reg_to_reg(Register8::B, Register8::E); self.pc += 1; } // LD E, B
            0x59 => { self.ld_reg_to_reg(Register8::C, Register8::E); self.pc += 1; } // LD E, C
            0x5A => { self.ld_reg_to_reg(Register8::D, Register8::E); self.pc += 1; } // LD E, D
            0x5B => { self.ld_reg_to_reg(Register8::E, Register8::E); self.pc += 1; } // LD E, E
            0x5C => { self.ld_reg_to_reg(Register8::H, Register8::E); self.pc += 1; } // LD E, H
            0x5D => { self.ld_reg_to_reg(Register8::L, Register8::E); self.pc += 1; } // LD E, L

            0x67 => { self.ld_reg_to_reg(Register8::A, Register8::H); self.pc += 1; } // LD H, A
            0x60 => { self.ld_reg_to_reg(Register8::B, Register8::H); self.pc += 1; } // LD H, B
            0x61 => { self.ld_reg_to_reg(Register8::C, Register8::H); self.pc += 1; } // LD H, C
            0x62 => { self.ld_reg_to_reg(Register8::D, Register8::H); self.pc += 1; } // LD H, D
            0x63 => { self.ld_reg_to_reg(Register8::E, Register8::H); self.pc += 1; } // LD H, E
            0x64 => { self.ld_reg_to_reg(Register8::H, Register8::H); self.pc += 1; } // LD H, H
            0x65 => { self.ld_reg_to_reg(Register8::L, Register8::H); self.pc += 1; } // LD H, L

            0x6F => { self.ld_reg_to_reg(Register8::A, Register8::L); self.pc += 1; } // LD L, A
            0x68 => { self.ld_reg_to_reg(Register8::B, Register8::L); self.pc += 1; } // LD L, B
            0x69 => { self.ld_reg_to_reg(Register8::C, Register8::L); self.pc += 1; } // LD L, C
            0x6A => { self.ld_reg_to_reg(Register8::D, Register8::L); self.pc += 1; } // LD L, D
            0x6B => { self.ld_reg_to_reg(Register8::E, Register8::L); self.pc += 1; } // LD L, E
            0x6C => { self.ld_reg_to_reg(Register8::H, Register8::L); self.pc += 1; } // LD L, H
            0x6D => { self.ld_reg_to_reg(Register8::L, Register8::L); self.pc += 1; } // LD L, L

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
        self.f = 0; // clear all flags

        if result == 0 { // set the 0 flag if needed
            self.f |= 0x80;
        }

        if (reg1 & 0x0F) >= 0x08 { // set the half carry flag if needed
            self.f |= 0x20;
        }

        if overflow { // set the carry flag if needed
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

    fn ld_reg_to_reg(&mut self, src_reg: Register8, dest_reg: Register8) {
        let src_val = match src_reg {
            Register8::A => self.a,
            Register8::B => self.b,
            Register8::C => self.c,
            Register8::D => self.d,
            Register8::E => self.e,
            Register8::H => self.h,
            Register8::L => self.l,
        };

        match dest_reg {
            Register8::A => self.a = src_val,
            Register8::B => self.b = src_val,
            Register8::C => self.c = src_val,
            Register8::D => self.d = src_val,
            Register8::E => self.e = src_val,
            Register8::H => self.h = src_val,
            Register8::L => self.l = src_val,
        }
    }

    fn ld_a_from_a16(&mut self) {
        self.a = self.memory[self.get_16_bit_operand(self.pc) as usize];
    }

    fn ld_a_from_a8(&mut self) {
        self.a = self.memory[(self.pc as usize) + 1];
    }

    fn ld_a_from_bc(&mut self) {
        self.a = self.memory[((self.b as u16) << 8 | self.c as u16) as usize];
    }

    fn ld_a_from_c(&mut self) {
        self.a = self.memory[self.c as usize];
    }

    fn ld_a_from_de(&mut self) {
        self.a = self.memory[((self.d as u16) << 8 | self.e as u16) as usize];
    }

    fn ld_a_from_hl_minus(&mut self) {
        let addr = ((self.h as u16) << 8 | self.l as u16) - 1;
        self.a = self.memory[addr as usize];
        self.h = (addr >> 8) as u8;
        self.l = (addr & 0xFF) as u8;
    }

    fn ld_a_from_hl_plus(&mut self) {
        let addr = ((self.h as u16) << 8 | self.l as u16) + 1;
        self.a = self.memory[addr as usize];
        self.h = (addr >> 8) as u8;
        self.l = (addr & 0xFF) as u8;
    }

    fn ld_memory_at_a16_from_a(&mut self) {
        let addr = self.get_16_bit_operand(self.pc);
        self.memory[addr as usize] = self.a;
    }

    fn ld_memory_at_a8_from_a(&mut self) {
        self.memory[(self.pc as usize) + 1] = self.a;
    }

    fn ld_memory_at_bc_from_a(&mut self) {
        let addr = ((self.b as u16) << 8 | self.c as u16);
        self.memory[addr as usize] = self.a;
    }

    fn ld_memory_at_c_from_a(&mut self) {
        self.memory[self.c as usize] = self.a;
    }

    fn ld_memory_at_de_from_a(&mut self) {
        let addr = ((self.d as u16) << 8 | self.e as u16);
        self.memory[addr as usize] = self.a;
    }

    fn ld_memory_at_hl_minus_from_a(&mut self) {
        let addr = ((self.h as u16) << 8 | self.l as u16) - 1;
        self.memory[addr as usize] = self.a;
        self.h = (addr >> 8) as u8;
        self.l = (addr & 0xFF) as u8;
    }

    fn ld_memory_at_hl_plus_from_a(&mut self) {
        let addr = ((self.h as u16) << 8 | self.l as u16) + 1;
        self.memory[addr as usize] = self.a;
        self.h = (addr >> 8) as u8;
        self.l = (addr & 0xFF) as u8;
    }

    fn ld_memory_at_hl_from_d8(&mut self) {
        let val = self.memory[(self.pc as usize) + 1];
        self.h = (val >> 8) as u8;
        self.l = (val & 0xFF) as u8;
    }

    fn ld_reg_from_hl(&mut self, reg: Register8) {
        let addr = ((self.h as u16) << 8 | self.l as u16);
        let val = self.memory[addr as usize];
        match reg {
            Register8::A => self.a = val,
            Register8::B => self.b = val,
            Register8::C => self.c = val,
            Register8::D => self.d = val,
            Register8::E => self.e = val,
            Register8::H => self.h = val,
            Register8::L => self.l = val,
        }
    }

    fn ld_d_from_a(&mut self) {
        self.d = self.a;
    }
