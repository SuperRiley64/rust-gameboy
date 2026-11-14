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
    pub memory: [u8; 0x10000],
    pub cycles: u8
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
            cycles: 0,
        }
    }

    // Runs the fetch decode execute cycle
    pub fn cycle(&mut self) {
        println!("Cycle pc: {}", self.pc);
        if self.cycles == 0 {
            let opcode = self.memory[self.pc as usize]; 
                println!("    Running opcode: 0x{:02X}", opcode);
            // cycles is always the documented cycles minus 1.
            match opcode {
                0x00 => { self.pc += 1; self.cycles = 0; } // NOP
                0x10 => { self.pc += 2; self.cycles = 0; } // STOP //TODO: Handle RESET
                0x76 => { self.pc += 1; self.cycles = 0; } // HALT //TODO: Interrupt/Reset
                
                // 8-bit transfer instructions
                0x3E => { self.a = self.memory[(self.pc as usize) + 1]; self.pc += 2; self.cycles = 1; } // LD A, n
                0x06 => { self.b = self.memory[(self.pc as usize) + 1]; self.pc += 2; self.cycles = 1; } // LD B, n
                0x0E => { self.c = self.memory[(self.pc as usize) + 1]; self.pc += 2; self.cycles = 1; } // LD C, n
                0x16 => { self.d = self.memory[(self.pc as usize) + 1]; self.pc += 2; self.cycles = 1; } // LD D, n
                0x1E => { self.e = self.memory[(self.pc as usize) + 1]; self.pc += 2; self.cycles = 1; } // LD E, n
                0x26 => { self.h = self.memory[(self.pc as usize) + 1]; self.pc += 2; self.cycles = 1; } // LD H, n
                0x2E => { self.l = self.memory[(self.pc as usize) + 1]; self.pc += 2; self.cycles = 1; } // LD L, n
                
                0xFA => { self.ld_a_from_a16(); self.pc += 3; self.cycles = 3;} // LD A, (a16)
                0xF0 => { self.ld_a_from_a8(); self.pc += 2; self.cycles = 2;} // LD A, (a8)
                
                0xF2 => { self.ld_a_from_c_high(); self.pc += 1; self.cycles = 1;} // LD A, (C)
                0x0A => { self.ld_a_from_bc(); self.pc += 1; self.cycles = 1;} // LD A, (BC)
                0x1A => { self.ld_a_from_de(); self.pc += 1; self.cycles = 1;} // LD A, (DE)
                
                0x2A => { self.ld_a_to_hl_plus(); self.pc += 1; self.cycles = 1;} // LD A, (HL+)
                0x3A => { self.ld_a_to_hl_minus(); self.pc += 1; self.cycles = 1;} // LD A, (HL-)
                0x02 => { self.ld_memory_at_bc_from_a(); self.pc += 1; self.cycles = 1;} // LD (BC), A
                0x12 => { self.ld_memory_at_de_from_a(); self.pc += 1; self.cycles = 1;} // LD (DE), A
                
                0x32 => { self.ld_memory_at_hl_minus_from_a(); self.pc += 1; self.cycles = 1;} // LD (HL-), A
                0x22 => { self.ld_memory_at_hl_plus_from_a(); self.pc += 1; self.cycles = 1;} // LD (HL+), A
                0x08 => { self.ld_memory_at_a16_from_sp(); self.pc += 3; self.cycles = 4; } // LD (a16), SP
                0xEA => { self.ld_memory_at_a16_from_a(); self.pc += 3; self.cycles = 3;} // LD (a16), A
                0xE2 => { self.ld_memory_at_c_from_a(); self.pc += 1; self.cycles = 1;} // LD (C), A
                0xE0 => { self.ld_memory_at_a8_from_a(); self.pc += 2; self.cycles = 2;} // LD (a8), A
                
                0x36 => { self.ld_memory_at_hl_from_d8(); self.pc += 2; self.cycles = 3;} // LD (HL), d8 
                
                0x70 => { self.memory[((self.h as u16) << 8 | self.l as u16) as usize] = self.b; self.pc += 1; self.cycles = 1; } // LD (HL), B
                0x71 => { self.memory[((self.h as u16) << 8 | self.l as u16) as usize] = self.c; self.pc += 1; self.cycles = 1; } // LD (HL), C
                0x72 => { self.memory[((self.h as u16) << 8 | self.l as u16) as usize] = self.d; self.pc += 1; self.cycles = 1; } // LD (HL), D
                0x73 => { self.memory[((self.h as u16) << 8 | self.l as u16) as usize] = self.e; self.pc += 1; self.cycles = 1; } // LD (HL), E
                0x74 => { self.memory[((self.h as u16) << 8 | self.l as u16) as usize] = self.h; self.pc += 1; self.cycles = 1; } // LD (HL), H
                0x75 => { self.memory[((self.h as u16) << 8 | self.l as u16) as usize] = self.l; self.pc += 1; self.cycles = 1; } // LD (HL), L
                
                0x7E => { self.ld_reg_from_hl(Register8::A); self.pc += 1; self.cycles = 1; } // LD A, (HL)
                0x46 => { self.ld_reg_from_hl(Register8::B); self.pc += 1; self.cycles = 1; } // LD B, (HL)
                0x56 => { self.ld_reg_from_hl(Register8::D); self.pc += 1; self.cycles = 1; } // LD D, (HL)
                0x5E => { self.ld_reg_from_hl(Register8::E); self.pc += 1; self.cycles = 1; } // LD E, (HL)
                0x66 => { self.ld_reg_from_hl(Register8::H); self.pc += 1; self.cycles = 1; } // LD H, (HL)
                0x6E => { self.ld_reg_from_hl(Register8::L); self.pc += 1; self.cycles = 1; } // LD L, (HL)
                
                // 16-bit load immediate instructions
                0x01 => { let val = self.get_16_bit_operand(self.pc); self.assign_16bit_register(Register16::BC, val); self.pc += 3; self.cycles = 2; } // LD BC, nn
                0x11 => { let val = self.get_16_bit_operand(self.pc); self.assign_16bit_register(Register16::DE, val); self.pc += 3; self.cycles = 2; } // LD DE, nn
                0x21 => { let val = self.get_16_bit_operand(self.pc); self.assign_16bit_register(Register16::HL, val); self.pc += 3; self.cycles = 2; } // LD HL, nn
                0x31 => { let val = self.get_16_bit_operand(self.pc); self.assign_16bit_register(Register16::SP, val); self.pc += 3; self.cycles = 2; } // LD SP, nn
                
                // 8-bit load register instructions
                0x7F => { self.ld_reg_to_reg(Register8::A, Register8::A); self.pc += 1; self.cycles = 0; } // LD A, A
                0x78 => { self.ld_reg_to_reg(Register8::B, Register8::A); self.pc += 1; self.cycles = 0; } // LD A, B
                0x79 => { self.ld_reg_to_reg(Register8::C, Register8::A); self.pc += 1; self.cycles = 0; } // LD A, C
                0x7A => { self.ld_reg_to_reg(Register8::D, Register8::A); self.pc += 1; self.cycles = 0; } // LD A, D
                0x7B => { self.ld_reg_to_reg(Register8::E, Register8::A); self.pc += 1; self.cycles = 0; } // LD A, E
                0x7C => { self.ld_reg_to_reg(Register8::H, Register8::A); self.pc += 1; self.cycles = 0; } // LD A, H
                0x7D => { self.ld_reg_to_reg(Register8::L, Register8::A); self.pc += 1; self.cycles = 0; } // LD A, L
                
                0x40 => { self.ld_reg_to_reg(Register8::B, Register8::B); self.pc += 1; self.cycles = 0; } // LD B, B
                0x47 => { self.ld_reg_to_reg(Register8::A, Register8::B); self.pc += 1; self.cycles = 0; } // LD B, A
                0x41 => { self.ld_reg_to_reg(Register8::C, Register8::B); self.pc += 1; self.cycles = 0; } // LD B, C
                0x42 => { self.ld_reg_to_reg(Register8::D, Register8::B); self.pc += 1; self.cycles = 0; } // LD B, D
                0x43 => { self.ld_reg_to_reg(Register8::E, Register8::B); self.pc += 1; self.cycles = 0; } // LD B, E
                0x44 => { self.ld_reg_to_reg(Register8::H, Register8::B); self.pc += 1; self.cycles = 0; } // LD B, H
                0x45 => { self.ld_reg_to_reg(Register8::L, Register8::B); self.pc += 1; self.cycles = 0; } // LD B, L
                
                0x4F => { self.ld_reg_to_reg(Register8::A, Register8::C); self.pc += 1; self.cycles = 0; } // LD C, A
                0x48 => { self.ld_reg_to_reg(Register8::B, Register8::C); self.pc += 1; self.cycles = 0; } // LD C, B
                0x49 => { self.ld_reg_to_reg(Register8::C, Register8::C); self.pc += 1; self.cycles = 0; } // LD C, C
                0x4A => { self.ld_reg_to_reg(Register8::D, Register8::C); self.pc += 1; self.cycles = 0; } // LD C, D
                0x4B => { self.ld_reg_to_reg(Register8::E, Register8::C); self.pc += 1; self.cycles = 0; } // LD C, E
                0x4C => { self.ld_reg_to_reg(Register8::H, Register8::C); self.pc += 1; self.cycles = 0; } // LD C, H
                0x4D => { self.ld_reg_to_reg(Register8::L, Register8::C); self.pc += 1; self.cycles = 0; } // LD C, L
                
                0x57 => { self.ld_reg_to_reg(Register8::A, Register8::D); self.pc += 1; self.cycles = 0; } // LD D, A
                0x50 => { self.ld_reg_to_reg(Register8::B, Register8::D); self.pc += 1; self.cycles = 0; } // LD D, B
                0x51 => { self.ld_reg_to_reg(Register8::C, Register8::D); self.pc += 1; self.cycles = 0; } // LD D, C
                0x52 => { self.ld_reg_to_reg(Register8::D, Register8::D); self.pc += 1; self.cycles = 0; } // LD D, D
                0x53 => { self.ld_reg_to_reg(Register8::E, Register8::D); self.pc += 1; self.cycles = 0; } // LD D, E
                0x54 => { self.ld_reg_to_reg(Register8::H, Register8::D); self.pc += 1; self.cycles = 0; } // LD D, H
                0x55 => { self.ld_reg_to_reg(Register8::L, Register8::D); self.pc += 1; self.cycles = 0; } // LD D, L
                
                0x5F => { self.ld_reg_to_reg(Register8::A, Register8::E); self.pc += 1; self.cycles = 0; } // LD E, A
                0x58 => { self.ld_reg_to_reg(Register8::B, Register8::E); self.pc += 1; self.cycles = 0; } // LD E, B
                0x59 => { self.ld_reg_to_reg(Register8::C, Register8::E); self.pc += 1; self.cycles = 0; } // LD E, C
                0x5A => { self.ld_reg_to_reg(Register8::D, Register8::E); self.pc += 1; self.cycles = 0; } // LD E, D
                0x5B => { self.ld_reg_to_reg(Register8::E, Register8::E); self.pc += 1; self.cycles = 0; } // LD E, E
                0x5C => { self.ld_reg_to_reg(Register8::H, Register8::E); self.pc += 1; self.cycles = 0; } // LD E, H
                0x5D => { self.ld_reg_to_reg(Register8::L, Register8::E); self.pc += 1; self.cycles = 0; } // LD E, L
                
                0x67 => { self.ld_reg_to_reg(Register8::A, Register8::H); self.pc += 1; self.cycles = 0; } // LD H, A
                0x60 => { self.ld_reg_to_reg(Register8::B, Register8::H); self.pc += 1; self.cycles = 0; } // LD H, B
                0x61 => { self.ld_reg_to_reg(Register8::C, Register8::H); self.pc += 1; self.cycles = 0; } // LD H, C
                0x62 => { self.ld_reg_to_reg(Register8::D, Register8::H); self.pc += 1; self.cycles = 0; } // LD H, D
                0x63 => { self.ld_reg_to_reg(Register8::E, Register8::H); self.pc += 1; self.cycles = 0; } // LD H, E
                0x64 => { self.ld_reg_to_reg(Register8::H, Register8::H); self.pc += 1; self.cycles = 0; } // LD H, H
                0x65 => { self.ld_reg_to_reg(Register8::L, Register8::H); self.pc += 1; self.cycles = 0; } // LD H, L
                
                0x6F => { self.ld_reg_to_reg(Register8::A, Register8::L); self.pc += 1; self.cycles = 0; } // LD L, A
                0x68 => { self.ld_reg_to_reg(Register8::B, Register8::L); self.pc += 1; self.cycles = 0; } // LD L, B
                0x69 => { self.ld_reg_to_reg(Register8::C, Register8::L); self.pc += 1; self.cycles = 0; } // LD L, C
                0x6A => { self.ld_reg_to_reg(Register8::D, Register8::L); self.pc += 1; self.cycles = 0; } // LD L, D
                0x6B => { self.ld_reg_to_reg(Register8::E, Register8::L); self.pc += 1; self.cycles = 0; } // LD L, E
                0x6C => { self.ld_reg_to_reg(Register8::H, Register8::L); self.pc += 1; self.cycles = 0; } // LD L, H
                0x6D => { self.ld_reg_to_reg(Register8::L, Register8::L); self.pc += 1; self.cycles = 0; } // LD L, L
                
                // 8-bit arithmetic instructions
                0x87 => { self.a = self.add_8_bit(self.a, self.a); self.pc += 1; self.cycles = 0;} // ADD A, A
                
                // Jump instructions
                0x18 => { self.jump_relative_s8(self.memory[(self.pc as usize) + 1] as i8); self.cycles = 2; } // JR s8
                0x20 => {
                    if (self.f & 0x80) == 0 { // zero flag
                        self.jump_relative_s8(self.memory[(self.pc as usize) + 1] as i8);
                    } else {
                        self.pc += 2;
                    }
                    self.cycles = 2;
                } // JR NZ s8
                0x30 => {
                    if (self.f & 0x10) == 0 { // carry flag
                        self.jump_relative_s8(self.memory[(self.pc as usize) + 1] as i8);
                    } else {
                        self.pc += 2;
                    }
                    self.cycles = 2;
                } // JR NC s8
                0x28 => {
                    if (self.f & 0x80) == 0x80 { // zero flag
                        self.jump_relative_s8(self.memory[(self.pc as usize) + 1] as i8);
                    } else {
                        self.pc += 2;
                    }
                    self.cycles = 2;
                } // JR Z s8
                0x38 => {
                    if (self.f & 0x10) == 0x10 { // carry flag
                        self.jump_relative_s8(self.memory[(self.pc as usize) + 1] as i8);
                    } else {
                        self.pc += 2;
                    }
                    self.cycles = 2;
                } // JR C s8
                0xC3 => {self.jump_nn(); self.cycles = 2;} // JP nn
                
                // increment/decrement instructions
                0x3C => { self.a = self.increment_8bit_register(self.a); self.pc += 1; self.cycles = 0; } // INC A
                0x04 => { self.b = self.increment_8bit_register(self.b); self.pc += 1; self.cycles = 0; } // INC B
                0x0C => { self.c = self.increment_8bit_register(self.c); self.pc += 1; self.cycles = 0; } // INC C
                0x14 => { self.d = self.increment_8bit_register(self.d); self.pc += 1; self.cycles = 0; } // INC D
                0x1C => { self.e = self.increment_8bit_register(self.e); self.pc += 1; self.cycles = 0; } // INC E
                0x24 => { self.h = self.increment_8bit_register(self.h); self.pc += 1; self.cycles = 0; } // INC H
                0x2C => { self.l = self.increment_8bit_register(self.l); self.pc += 1; self.cycles = 0; } // INC L
                
                0x3D => { self.a = self.decrement_8bit_register(self.a); self.pc += 1; self.cycles = 0; } // DEC A
                0x05 => { self.b = self.decrement_8bit_register(self.b); self.pc += 1; self.cycles = 0; } // DEC B
                0x0D => { self.c = self.decrement_8bit_register(self.c); self.pc += 1; self.cycles = 0; } // DEC C
                0x15 => { self.d = self.decrement_8bit_register(self.d); self.pc += 1; self.cycles = 0; } // DEC D
                0x1D => { self.e = self.decrement_8bit_register(self.e); self.pc += 1; self.cycles = 0; } // DEC E
                0x25 => { self.h = self.decrement_8bit_register(self.h); self.pc += 1; self.cycles = 0; } // DEC H
                0x2D => { self.l = self.decrement_8bit_register(self.l); self.pc += 1; self.cycles = 0; } // DEC L
                
                0x03 => { self.increment_16_bit_register(self.b, self.c, Register16::BC); self.pc += 1; self.cycles = 1; } // INC BC
                0x13 => { self.increment_16_bit_register(self.d, self.e, Register16::DE); self.pc += 1; self.cycles = 1; } // INC DE
                0x23 => { self.increment_16_bit_register(self.h, self.l, Register16::HL); self.pc += 1; self.cycles = 1; } // INC HL
                0x33 => { let high = (self.sp >> 8) as u8; let low = (self.sp & 0xFF) as u8; self.increment_16_bit_register(high, low, Register16::SP); self.pc += 1; self.cycles = 1; } // INC SP
                
                0x0B => { self.decrement_16_bit_register(self.b, self.c, Register16::BC); self.pc += 1; self.cycles = 1; } // DEC BC
                0x1B => { self.decrement_16_bit_register(self.d, self.e, Register16::DE); self.pc += 1; self.cycles = 1; } // DEC DE
                0x2B => { self.decrement_16_bit_register(self.h, self.l, Register16::HL); self.pc += 1; self.cycles = 1; } // DEC HL
                0x3B => { let high = (self.sp >> 8) as u8; let low = (self.sp & 0xFF) as u8; self.decrement_16_bit_register(high, low, Register16::SP); self.pc += 1; self.cycles = 1; }
                
                // 16-bit adds
                0x09 => { self.add_16bit_to_hl(Register16::BC); self.sp += 1; self.pc += 1; self.cycles = 1; } // ADD HL, BC
                0x19 => { self.add_16bit_to_hl(Register16::DE); self.sp += 1; self.pc += 1; self.cycles = 1; } // ADD HL, DE
                0x29 => { self.add_16bit_to_hl(Register16::HL); self.sp += 1; self.pc += 1; self.cycles = 1; } // ADD HL, HL
                0x39 => { self.add_16bit_to_hl(Register16::SP); self.sp += 1; self.pc += 1; self.cycles = 1; } // ADD HL, SP
                0x07 => { self.rlca(); self.pc += 1; self.cycles = 0; } // RLCA
                0x17 => { self.rla(); self.pc += 1; self.cycles = 0; } // RLA
                0x0F => { self.rrca(); self.pc += 1; self.cycles = 0; } // RRCA
                0x1F => { self.rra(); self.pc += 1; self.cycles = 0; } // RRA
                
                // Carry flag instructions
                0x37 => { self.f = (self.f & 0x8F) | 0x10; self.pc += 1; self.cycles = 1; } // SCF (set the carry flag)
                0x3F => { self.f = (self.f ^ 0x10) & 0x9F; self.pc += 1; self.cycles = 1; } // CCF (flip the carry flag)
                
                _ => { self.pc += 1; self.cycles = 1; } // Unimplemented, do a NOP
            }
        } else {
            // If we aren't fetching a new instruction, just tick the clock down
            self.cycles -= 1; // timing
        }
    }

    // ============= HELPER FUNCTIONS ==========================

    fn advance_pc(&mut self, bytes: u16) {
        self.pc = self.pc.wrapping_add(bytes);
    }

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

    // Helper method to increment a 16-bit register
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

    // Helper method to decrement a 16-bit register
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

    // Helper method to increment a 16-bit register
    fn add_16bit_to_hl(&mut self, reg: Register16) {
        let (src_high, src_low) = match reg {
            Register16::BC => (self.b, self.c),
            Register16::DE => (self.d, self.e),
            Register16::HL => (self.h, self.l),
            Register16::SP => ((self.sp >> 8) as u8, (self.sp & 0xFF) as u8),
        };

        let current_hl = ((self.h as u16) << 8) | (self.l as u16);
        let source_val = ((src_high as u16) << 8) | (src_low as u16);
        
        let (result, overflow) = current_hl.overflowing_add(source_val);

        // Calculate half-carry: carry from bit 3 to 4 of the low byte
        let half_carry = ((current_hl & 0x0F) + (source_val & 0x0F)) > 0x0F;

        // Update registers
        self.assign_16bit_register(Register16::HL, result);

        // Update flags
        // Preserve zero flag (bit 7)
        self.f &= 0x80;
        // Clear subtract flag (bit 6)
        self.f &= !0x40;

        // Set carry flag (bit 4)
        if overflow {
            self.f |= 0x10;
        } else {
            self.f &= !0x10;
        }

        // Set half-carry flag (bit 5)
        if half_carry {
            self.f |= 0x20;
        } else {
            self.f &= !0x20;
        }
    }

    fn jump_nn(&mut self) {
        // 1. Fetch the 16-bit operand.
        // According to your rules:
        // Byte at (pc + 1) is the lower-order byte (bits 0-7)
        // Byte at (pc + 2) is the higher-order byte (bits 8-15)
        let low_byte = self.memory[self.pc as usize + 1] as u16;
        let high_byte = self.memory[self.pc as usize + 2] as u16;
        
        // 2. Combine them into a 16-bit address
        // Shift high_byte left by 8 bits and combine with low_byte
        let target_address = (high_byte << 8) | low_byte;
        
        // 3. Update the Program Counter
        // The PC now points directly to the start of the new instruction.
        self.pc = target_address as u16;
        
        // Note: We do NOT do self.pc += 3 here. 
        // The jump instruction's job is to move the PC to the new location.
        // If you added 3, you would skip the first 3 bytes of the target instruction.
    }

    fn jump_relative_s8(&mut self, s8: i8) {
        let offset = s8 as i32;
        self.pc = (self.pc as i32 + 2 + offset) as u16;
    }

    fn rla(&mut self) {
        let old_carry = (self.f & 0x10) != 0;
        self.a = (self.a << 1) | (if old_carry { 1 } else { 0 });
        
        // Update flags
        self.f = 0;
        if self.a == 0 {
            self.f |= 0x80; // Zero flag
        }
        if (self.a & 0x0F) == 0x0F {
            self.f |= 0x20; // Half-carry flag
        }
        if old_carry {
            self.f |= 0x10; // Carry flag (unchanged)
        }
    }

    fn rrca(&mut self) {
        let carry = (self.a & 0x01) != 0;
        self.a = (self.a >> 1) | (self.a << 7);
        
        // Update flags
        self.f = 0;
        if self.a == 0 {
            self.f |= 0x80; // Zero flag
        }
        if (self.a & 0x0F) == 0x0F {
            self.f |= 0x20; // Half-carry flag
        }
        if carry {
            self.f |= 0x10; // Carry flag
        }
    }

    fn rra(&mut self) {
        let old_carry = (self.f & 0x10) != 0;
        self.a = (self.a >> 1) | (if old_carry { 0x80 } else { 0 });
        
        // Update flags
        self.f = 0;
        if self.a == 0 {
            self.f |= 0x80; // Zero flag
        }
        if (self.a & 0x0F) == 0x0F {
            self.f |= 0x20; // Half-carry flag
        }
        if old_carry {
            self.f |= 0x10; // Carry flag (unchanged)
        }
    }

    fn rlca(&mut self) {
        let carry = (self.a & 0x80) != 0;
        self.a = self.a << 1;
        
        // Update flags
        self.f = 0;
        if self.a == 0 {
            self.f |= 0x80; // Zero flag
        }
        if (self.a & 0x0F) == 0x0F {
            self.f |= 0x20; // Half-carry flag
        }
        if carry {
            self.f |= 0x10; // Carry flag
        }
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
        let addr = self.memory[(self.pc as usize) + 1] as usize;
        self.a = self.memory[addr];
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

    fn ld_a_from_c_high(&mut self) {
        self.a = self.memory[(0xFF00 + self.c as u16) as usize];
    }

    fn ld_a_to_hl_minus(&mut self) {
        // 1. Get the current address from HL
        let current_addr = ((self.h as u16) << 8 | self.l as u16) as usize;

        // 2. Store the contents of that memory location into A (the "Store" part)
        self.a = self.memory[current_addr];

        // 3. Increment the contents of HL (The "Increment" part)
        let new_addr = (current_addr as u16) - 1;
        self.h = (new_addr >> 8) as u8;
        self.l = (new_addr & 0xFF) as u8;
    }

    fn ld_memory_at_hl_minus_from_a(&mut self) {
        // 1. Combine H and L to form the 16-bit address
        // We shift H left by 8 bits and OR it with L
        let address = ((self.h as usize) << 8) | (self.l as usize);

        // 2. Store the contents of Register A into that memory location
        // We check the bounds to ensure the address is valid (0-0xFFFF)
        if address <= 0xFFFF {
            self.memory[address] = self.a;
        }

        // 3. Decrement the HL pair
        // We treat HL as a 16-bit value to ensure that if L is 0x00, 
        // it correctly "borrows" from H.
        let current_hl = ((self.h as u16) << 8) | (self.l as u16);
        let new_hl = current_hl.wrapping_sub(1);

        // Split the new 16-bit value back into H and L
        self.h = (new_hl >> 8) as u8;
        self.l = (new_hl & 0xFF) as u8;
    }

    fn ld_a_to_hl_plus(&mut self) {
        // 1. Get the current address from HL
        let current_addr = ((self.h as u16) << 8 | self.l as u16) as usize;

        // 2. Store the contents of that memory location into A (the "Store" part)
        self.a = self.memory[current_addr];

        // 3. Increment the contents of HL (The "Increment" part)
        let new_addr = (current_addr as u16) + 1;
        self.h = (new_addr >> 8) as u8;
        self.l = (new_addr & 0xFF) as u8;
    }

    fn ld_memory_at_a16_from_a(&mut self) {
        let addr = self.get_16_bit_operand(self.pc);
        self.memory[addr as usize] = self.a;
    }

    fn ld_memory_at_a8_from_a(&mut self) {
        // 1. Fetch the 8-bit immediate operand from the memory 
        //    located immediately after the current Program Counter.
        let immediate_value = self.memory[self.pc as usize + 1] as usize;

        // 2. Construct the 16-bit address.
        //    The high byte is 0xFF (fixed) and the low byte is the immediate value.
        let address = 0xFF00 + immediate_value;

        // 3. Store the contents of Register A into that calculated address.
        if address <= 0xFFFF {
            self.memory[address] = self.a;
        }
    }

    fn ld_memory_at_bc_from_a(&mut self) {
        let addr = (self.b as u16) << 8 | self.c as u16;
        self.memory[addr as usize] = self.a;
    }

    fn ld_memory_at_c_from_a(&mut self) {
        // Calculate the target address by adding the base (0xFF00) 
        // to the offset provided by register C.
        let address = 0xFF00 + (self.c as usize);

        self.memory[address] = self.a;
    }

    fn ld_memory_at_de_from_a(&mut self) {
        let addr = (self.d as u16) << 8 | self.e as u16;
        self.memory[addr as usize] = self.a;
    }

    fn ld_memory_at_hl_plus_from_a(&mut self) {
        // 1. Calculate the CURRENT address from the HL pair
        // We use u16 to ensure the bitwise shift and OR work correctly.
        let current_hl = ((self.h as u16) << 8) | (self.l as u16);

        // 2. Store the contents of Register A into the CURRENT address
        // Check bounds to prevent the emulator from crashing.
        if (current_hl as usize) < self.memory.len() {
            self.memory[current_hl as usize] = self.a;
        }

        // 3. Increment the HL pair
        // We use wrapping_add(1) to ensure that if HL is 0xFFFF, 
        // it correctly wraps around to 0x0000 instead of panicking.
        let new_hl = current_hl.wrapping_add(1);

        // 4. Update the H and L registers with the new value
        self.h = (new_hl >> 8) as u8;
        self.l = (new_hl & 0xFF) as u8;
    }

    fn ld_memory_at_hl_from_d8(&mut self) {
        let hl = ((self.h as u16) << 8) | self.l as u16;
        let val = self.memory[(self.pc as usize) + 1];
        self.memory[hl as usize] = val;
    }

    fn ld_reg_from_hl(&mut self, reg: Register8) {
        let addr = (self.h as u16) << 8 | self.l as u16;
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

    fn get_16_bit_operand(&mut self, index: u16) -> u16 {                                                                                                                                        
        self.memory[(index + 1) as usize] as u16 | (self.memory[(index + 2) as usize] as u16) << 8
    }

    fn ld_memory_at_a16_from_sp(&mut self) {
        let addr = self.get_16_bit_operand(self.pc);
        self.memory[addr as usize] = (self.sp & 0xFF) as u8;
        self.memory[(addr + 1) as usize] = (self.sp >> 8) as u8;
    }
}
