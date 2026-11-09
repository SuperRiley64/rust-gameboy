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

        result
    }

    // Helper method to decrement a register and update flags
    fn decrement_8bit_register(&mut self, reg: u8) -> u8 {
        let result = reg.wrapping_sub(1);

        // Zero flag (bit 7): 1 if result is 0
        if result == 0 {
            self.f |= 0x80;
        } else {
            self.f &= !0x80;
        }

        // Half Carry (bit 5): 1 if lower 4 bits caused a borrow
        if (result & 0x0F) == 0 {
            self.f |= 0x20;
        } else {
            self.f &= !0x20;
        }

        result
    }

    // Runs the fetch decode execute cycle
    pub fn cycle(&mut self) {
        let opcode = self.memory[self.pc as usize];

        match opcode {
            0x00 => { // NOP
                // Do nothing
            }
            0x06 => { // LD B, n
                self.b = self.memory[self.pc as usize];
                self.pc += 1;
            }
            0x0E => { // LD C, n
                self.c = self.memory[self.pc as usize];
                self.pc += 1;
            }
            0x16 => { // LD D, n
                self.d = self.memory[self.pc as usize];
                self.pc += 1;
            }
            0x1E => { // LD E, n
                self.e = self.memory[self.pc as usize];
                self.pc += 1;
            }
            0x26 => { // LD H, n
                self.h = self.memory[self.pc as usize];
                self.pc += 1;
            }
            0x2E => { // LD L, n
                self.l = self.memory[self.pc as usize];
                self.pc += 1;
            }
            0x3E => { // LD A, n
                self.a = self.memory[self.pc as usize];
                self.pc += 1;
            }
            0x78 => { // LD A, B
                self.a = self.b;
            }
            0x79 => { // LD A, C
                self.a = self.c;
            }
            0x7A => { // LD A, D
                self.a = self.d;
            }
            0x7B => { // LD A, E
                self.a = self.e;
            }
            0x7C => { // LD A, H
                self.a = self.h;
            }
            0x7D => { // LD A, L
                self.a = self.l;
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
            0x01 => { // LD BC, nn
                let low = self.memory[self.pc as usize];   // Low byte (memory[pc])
                let high = self.memory[(self.pc + 1) as usize]; // High byte (memory[pc + 1])
                self.b = high;
                self.c = low;
                self.pc += 2;
            }
            0x11 => { // LD DE, nn
                let low = self.memory[self.pc as usize];
                let high = self.memory[(self.pc + 1) as usize];
                self.d = high;
                self.e = low;
                self.pc += 2;
            }
            0x21 => { // LD HL, nn
                let low = self.memory[self.pc as usize];
                let high = self.memory[(self.pc + 1) as usize];
                self.h = high;
                self.l = low;
                self.pc += 2;
            }
            0x31 => { // LD SP, nn
                let low = self.memory[self.pc as usize];
                let high = self.memory[(self.pc + 1) as usize];
                self.sp = ((high as u16) << 8) | (low as u16);
                self.pc += 2;
            }
            0x3C => { self.a = self.increment_8bit_register(self.a); self.pc += 2; } // INC A
            0x04 => { self.b = self.increment_8bit_register(self.b); self.pc += 2; } // INC B
            0x0C => { self.c = self.increment_8bit_register(self.c); self.pc += 2; } // INC C
            0x14 => { self.d = self.increment_8bit_register(self.d); self.pc += 2; } // INC D
            0x1C => { self.e = self.increment_8bit_register(self.e); self.pc += 2; } // INC E
            0x24 => { self.h = self.increment_8bit_register(self.h); self.pc += 2; } // INC H
            0x2C => { self.l = self.increment_8bit_register(self.l); self.pc += 2; } // INC L

            0x05 => { // DEC B
                self.b = self.b.wrapping_sub(1);

                self.f = 0x40; // Set Subtract Flag (bit 6)

                // Set Zero Flag (bit 7): 1 if result is 0
                if self.b == 0 {
                    self.f |= 0x80;
                }

                // Set Half Carry Flag (bit 5): 1 if lower 4 bits caused carry
                if (self.b & 0x0F) == 0x00 {
                    self.f |= 0x20;
                }
            }
            0x0D => { // DEC C
                self.c = self.c.wrapping_sub(1);

                self.f = 0x40; // Set Subtract Flag (bit 6)

                // Set Zero Flag (bit 7): 1 if result is 0
                if self.c == 0 {
                    self.f |= 0x80;
                }

                // Set Half Carry Flag (bit 5): 1 if lower 4 bits caused carry
                if (self.c & 0x0F) == 0x00 {
                    self.f |= 0x20;
                }
            }
            0x15 => { // DEC D
                self.d = self.d.wrapping_sub(1);

                self.f = 0x40; // Set Subtract Flag (bit 6)

                // Set Zero Flag (bit 7): 1 if result is 0
                if self.d == 0 {
                    self.f |= 0x80;
                }

                // Set Half Carry Flag (bit 5): 1 if lower 4 bits caused carry
                if (self.d & 0x0F) == 0x00 {
                    self.f |= 0x20;
                }
            }
            0x1D => { // DEC E
                self.e = self.e.wrapping_sub(1);

                self.f = 0x40; // Set Subtract Flag (bit 6)

                // Set Zero Flag (bit 7): 1 if result is 0
                if self.e == 0 {
                    self.f |= 0x80;
                }

                // Set Half Carry Flag (bit 5): 1 if lower 4 bits caused carry
                if (self.e & 0x0F) == 0x00 {
                    self.f |= 0x20;
                }
            }
            0x25 => { // DEC H
                self.h = self.h.wrapping_sub(1);

                self.f = 0x40; // Set Subtract Flag (bit 6)

                // Set Zero Flag (bit 7): 1 if result is 0
                if self.h == 0 {
                    self.f |= 0x80;
                }

                // Set Half Carry Flag (bit 5): 1 if lower 4 bits caused carry
                if (self.h & 0x0F) == 0x00 {
                    self.f |= 0x20;
                }
            }
            0x2D => { // DEC L
                self.l = self.l.wrapping_sub(1);

                self.f = 0x40; // Set Subtract Flag (bit 6)

                // Set Zero Flag (bit 7): 1 if result is 0
                if self.l == 0 {
                    self.f |= 0x80;
                }

                // Set Half Carry Flag (bit 5): 1 if lower 4 bits caused carry
                if (self.l & 0x0F) == 0x00 {
                    self.f |= 0x20;
                }
            }
            0x3D => { // DEC A
                self.a = self.a.wrapping_sub(1);

                self.f = 0x40; // Set Subtract Flag (bit 6)

                // Set Zero Flag (bit 7): 1 if result is 0
                if self.a == 0 {
                    self.f |= 0x80;
                }

                // Set Half Carry Flag (bit 5): 1 if lower 4 bits caused carry
                if (self.a & 0x0F) == 0x00 {
                    self.f |= 0x20;
                }
            }
            0x03 => { // INC BC
                let bc = ((self.b as u16) << 8) | self.c as u16;
                let new_bc = bc.wrapping_add(1);
                self.b = (new_bc >> 8) as u8;
                self.c = (new_bc & 0xFF) as u8;

                self.f = 0;
                if new_bc == 0 {
                    self.f |= 0x80; // Zero flag
                }
            }
            0x0B => { // DEC BC
                let bc = ((self.b as u16) << 8) | self.c as u16;
                let new_bc = bc.wrapping_sub(1);
                self.b = (new_bc >> 8) as u8;
                self.c = (new_bc & 0xFF) as u8;

                self.f = 0x40; // Subtract flag
                if new_bc == 0 {
                    self.f |= 0x80; // Zero flag
                }
            }
            0x13 => { // INC DE
                let de = ((self.d as u16) << 8) | self.e as u16;
                let new_de = de.wrapping_add(1);
                self.d = (new_de >> 8) as u8;
                self.e = (new_de & 0xFF) as u8;

                self.f = 0;
                if new_de == 0 {
                    self.f |= 0x80; // Zero flag
                }
            }
            0x1B => { // DEC DE
                let de = ((self.d as u16) << 8) | self.e as u16;
                let new_de = de.wrapping_sub(1);
                self.d = (new_de >> 8) as u8;
                self.e = (new_de & 0xFF) as u8;

                self.f = 0x40; // Subtract flag
                if new_de == 0 {
                    self.f |= 0x80; // Zero flag
                }
            }
            0x23 => { // INC HL
                let hl = ((self.h as u16) << 8) | self.l as u16;
                let new_hl = hl.wrapping_add(1);
                self.h = (new_hl >> 8) as u8;
                self.l = (new_hl & 0xFF) as u8;

                self.f = 0;
                if new_hl == 0 {
                    self.f |= 0x80; // Zero flag
                }
            }
            0x2B => { // DEC HL
                let hl = ((self.h as u16) << 8) | self.l as u16;
                let new_hl = hl.wrapping_sub(1);
                self.h = (new_hl >> 8) as u8;
                self.l = (new_hl & 0xFF) as u8;

                self.f = 0x40; // Subtract flag
                if new_hl == 0 {
                    self.f |= 0x80; // Zero flag
                }
            }
            0x33 => { // INC SP
                self.sp = self.sp.wrapping_add(1);
                self.f = 0;
                if self.sp == 0 {
                    self.f |= 0x80; // Zero flag
                }
            }
            0x3B => { // DEC SP
                self.sp = self.sp.wrapping_sub(1);
                self.f = 0x40; // Subtract flag
                if self.sp == 0 {
                    self.f |= 0x80; // Zero flag
                }
            }
            _ => {} // Unimplemented
        }
    }
}