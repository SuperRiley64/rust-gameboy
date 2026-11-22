use crate::gameboy::bus::Bus;

pub enum Register8 {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
}

pub enum Register16{
    BC,
    DE,
    HL,
    SP,
    AF,
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
    pub ime: bool, // interrupt master enable (write-only)
    pub halt: bool,
    pub sp: u16,
    pub pc: u16,
    pub cycles: u8,
    pub bus: Bus
}

impl CPU {
    pub fn new() -> Self {
        Self {
            a: 0,
            f: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            h: 0,
            l: 0,
            ime: false, // unset when game starts running
            halt: false,
            sp: 0xFFFE, // fffe because ffff is the interrupt enable register
            pc: 0,
            cycles: 0,
            bus: Bus::new()
        }
    }

    // Runs interrupts, halts, and the fetch decode execute cycle
    pub fn cycle(&mut self) {
        //println!("Cycle pc: {} | wait cycles: {}", self.pc, self.cycles);

        //TODO: tick serial from a reliable clock source
        self.bus.serial.cycle();

        // service interrupts
        if self.ime && self.cycles == 0 { // If interrupts are enabled...
            // Pull the Interrupt Enable Flags (IE) and Interrupt Requested Flags (IF)
            let ie_reg = self.bus.read(0xFFFF);
            let if_reg = self.bus.read(0xFF0F);


            let vblank_enabled: bool = (ie_reg & 0b00000001) != 0;
            let lcd_enabled: bool = (ie_reg & 0b00000010) != 0;
            let timer_enabled: bool = (ie_reg & 0b00000100) != 0;
            let serial_enabled: bool = (ie_reg & 0b00001000) != 0;
            let joypad_enabled: bool = (ie_reg & 0b00010000) != 0;

            let vblank_requested: bool = (if_reg & 0b00000001) != 0;
            let lcd_requested: bool = (if_reg & 0b00000010) != 0;
            let timer_requested: bool = (if_reg & 0b00000100) != 0;
            let serial_requested: bool = (if_reg & 0b00001000) != 0;
            let joypad_requested: bool = (if_reg & 0b00010000) != 0;

            // Service interrupts in order of priority, if enabled and requested
            if vblank_enabled && vblank_requested {
                let val = self.bus.read(0xFF0F);
                self.bus.write(0xFF0F, val & !0x01); // Clear VBlank flag

                self.ime = false;
                self.call_address(0x0040);
                self.cycles = 4;
                return;
            }
            if lcd_enabled && lcd_requested {
                let val = self.bus.read(0xFF0F);
                self.bus.write(0xFF0F, val & !0x02); // Clear LCD flag

                self.ime = false;
                self.call_address(0x0048);
                self.cycles = 4;
                return;
            }
            if timer_enabled && timer_requested {
                let val = self.bus.read(0xFF0F);
                self.bus.write(0xFF0F, val & !0x04); // Clear Timer flag

                self.ime = false;
                self.call_address(0x0050);
                self.cycles = 4;
                return;
            }
            if serial_enabled && serial_requested {
                let val = self.bus.read(0xFF0F);
                self.bus.write(0xFF0F, val & !0x08); // Clear Serial flag

                self.ime = false;
                self.call_address(0x0058);
                self.cycles = 4;
                return;
            }
            if joypad_enabled && joypad_requested {
                let val = self.bus.read(0xFF0F);
                self.bus.write(0xFF0F, val & !0x10); // Clear Joypad flag

                self.ime = false;
                self.call_address(0x0060);
                self.cycles = 4;
                return;
            }
        }

        // fetch decode execute
        if self.cycles == 0 {
            let opcode = self.bus.read(self.pc as u16); 
                //println!("    Running opcode: 0x{:02X}", opcode);
            // cycles is always the documented cycles minus 1.
            match opcode {
                0x00 => { self.pc = self.pc.wrapping_add(1); self.cycles = 0; } // NOP
                0x10 => { self.pc = self.pc.wrapping_add(2); self.cycles = 0; } // STOP //TODO: Handle RESET
                0x76 => { self.pc = self.pc.wrapping_add(1); self.cycles = 0; } // HALT //TODO: Interrupt/Reset
                
                // 8-bit transfer instructions
                0x3E => { self.a = self.bus.read((self.pc + 1) as u16); self.pc = self.pc.wrapping_add(2); self.cycles = 1; } // LD A, n
                0x06 => { self.b = self.bus.read((self.pc + 1) as u16); self.pc = self.pc.wrapping_add(2); self.cycles = 1; } // LD B, n
                0x0E => { self.c = self.bus.read((self.pc + 1) as u16); self.pc = self.pc.wrapping_add(2); self.cycles = 1; } // LD C, n
                0x16 => { self.d = self.bus.read((self.pc + 1) as u16); self.pc = self.pc.wrapping_add(2); self.cycles = 1; } // LD D, n
                0x1E => { self.e = self.bus.read((self.pc + 1) as u16); self.pc = self.pc.wrapping_add(2); self.cycles = 1; } // LD E, n
                0x26 => { self.h = self.bus.read((self.pc + 1) as u16); self.pc = self.pc.wrapping_add(2); self.cycles = 1; } // LD H, n
                0x2E => { self.l = self.bus.read((self.pc + 1) as u16); self.pc = self.pc.wrapping_add(2); self.cycles = 1; } // LD L, n
                
                0xFA => { self.ld_a_from_a16(); self.pc = self.pc.wrapping_add(3); self.cycles = 3;} // LD A, (a16)
                0xF0 => { self.ld_a_from_a8(); self.pc = self.pc.wrapping_add(2); self.cycles = 2;} // LD A, (a8)
                
                0xF2 => { self.ld_a_from_c_high(); self.pc = self.pc.wrapping_add(1); self.cycles = 1;} // LD A, (C)
                0x0A => { self.ld_a_from_bc(); self.pc = self.pc.wrapping_add(1); self.cycles = 1;} // LD A, (BC)
                0x1A => { self.ld_a_from_de(); self.pc = self.pc.wrapping_add(1); self.cycles = 1;} // LD A, (DE)
                
                0x2A => { self.ld_a_to_hl_plus(); self.pc = self.pc.wrapping_add(1); self.cycles = 1;} // LD A, (HL+)
                0x3A => { self.ld_a_to_hl_minus(); self.pc = self.pc.wrapping_add(1); self.cycles = 1;} // LD A, (HL-)
                0x02 => { self.ld_memory_at_bc_from_a(); self.pc = self.pc.wrapping_add(1); self.cycles = 1;} // LD (BC), A
                0x12 => { self.ld_memory_at_de_from_a(); self.pc = self.pc.wrapping_add(1); self.cycles = 1;} // LD (DE), A
                
                0x32 => { self.ld_memory_at_hl_minus_from_a(); self.pc = self.pc.wrapping_add(1); self.cycles = 1;} // LD (HL-), A
                0x22 => { self.ld_memory_at_hl_plus_from_a(); self.pc = self.pc.wrapping_add(1); self.cycles = 1;} // LD (HL+), A
                0x08 => { self.ld_memory_at_a16_from_sp(); self.pc = self.pc.wrapping_add(3); self.cycles = 4; } // LD (a16), SP
                0xEA => { self.ld_memory_at_a16_from_a(); self.pc = self.pc.wrapping_add(3); self.cycles = 3;} // LD (a16), A
                0xE2 => { self.ld_memory_at_c_from_a(); self.pc = self.pc.wrapping_add(1); self.cycles = 1;} // LD (C), A
                0xE0 => { self.ld_memory_at_a8_from_a(); self.pc = self.pc.wrapping_add(2); self.cycles = 2;} // LD (a8), A
                
                0x36 => { self.ld_memory_at_hl_from_d8(); self.pc = self.pc.wrapping_add(2); self.cycles = 3;} // LD (HL), d8 
                
                0x77 => { self.bus.write(((self.h as u16) << 8 | self.l as u16), self.a); self.pc = self.pc.wrapping_add(1); self.cycles = 1; } // LD (HL), A
                0x70 => { self.bus.write(((self.h as u16) << 8 | self.l as u16), self.b); self.pc = self.pc.wrapping_add(1); self.cycles = 1; } // LD (HL), B
                0x71 => { self.bus.write(((self.h as u16) << 8 | self.l as u16), self.c); self.pc = self.pc.wrapping_add(1); self.cycles = 1; } // LD (HL), C
                0x72 => { self.bus.write(((self.h as u16) << 8 | self.l as u16), self.d); self.pc = self.pc.wrapping_add(1); self.cycles = 1; } // LD (HL), D
                0x73 => { self.bus.write(((self.h as u16) << 8 | self.l as u16), self.e); self.pc = self.pc.wrapping_add(1); self.cycles = 1; } // LD (HL), E
                0x74 => { self.bus.write(((self.h as u16) << 8 | self.l as u16), self.h); self.pc = self.pc.wrapping_add(1); self.cycles = 1; } // LD (HL), H
                0x75 => { self.bus.write(((self.h as u16) << 8 | self.l as u16), self.l); self.pc = self.pc.wrapping_add(1); self.cycles = 1; } // LD (HL), L
                
                0x7E => { self.ld_reg_from_hl(Register8::A); self.pc = self.pc.wrapping_add(1); self.cycles = 1; } // LD A, (HL)
                0x46 => { self.ld_reg_from_hl(Register8::B); self.pc = self.pc.wrapping_add(1); self.cycles = 1; } // LD B, (HL)
                0x4E => { self.ld_reg_from_hl(Register8::C); self.pc = self.pc.wrapping_add(1); self.cycles = 1; } // LD C, (HL)
                0x56 => { self.ld_reg_from_hl(Register8::D); self.pc = self.pc.wrapping_add(1); self.cycles = 1; } // LD D, (HL)
                0x5E => { self.ld_reg_from_hl(Register8::E); self.pc = self.pc.wrapping_add(1); self.cycles = 1; } // LD E, (HL)
                0x66 => { self.ld_reg_from_hl(Register8::H); self.pc = self.pc.wrapping_add(1); self.cycles = 1; } // LD H, (HL)
                0x6E => { self.ld_reg_from_hl(Register8::L); self.pc = self.pc.wrapping_add(1); self.cycles = 1; } // LD L, (HL)
                
                // 16-bit load immediate instructions
                0x01 => { let val = self.get_16_bit_operand(self.pc); self.assign_16bit_register(Register16::BC, val); self.pc = self.pc.wrapping_add(3); self.cycles = 2; } // LD BC, nn
                0x11 => { let val = self.get_16_bit_operand(self.pc); self.assign_16bit_register(Register16::DE, val); self.pc = self.pc.wrapping_add(3); self.cycles = 2; } // LD DE, nn
                0x21 => { let val = self.get_16_bit_operand(self.pc); self.assign_16bit_register(Register16::HL, val); self.pc = self.pc.wrapping_add(3); self.cycles = 2; } // LD HL, nn
                0x31 => { let val = self.get_16_bit_operand(self.pc); self.assign_16bit_register(Register16::SP, val); self.pc = self.pc.wrapping_add(3); self.cycles = 2; } // LD SP, nn
                0xF9 => { self.sp = ((self.h as u16) << 8) | (self.l as u16); self.pc = self.pc.wrapping_add(1); self.cycles = 1; } // LD SP, HL
                0xF8 => { let s8 = self.bus.read((self.pc + 1) as u16) as i8; self.update_hl_from_sp_s8(s8); self.pc = self.pc.wrapping_add(2); self.cycles = 2; } // LD HL, SP+s8
                0xE8 => { let s8 = self.bus.read((self.pc + 1) as u16) as i8; self.add_16bit_to_sp_s8(s8); self.pc = self.pc.wrapping_add(2); self.cycles = 3; } // ADD SP, s8

                // 8-bit load register instructions
                0x7F => { self.ld_reg_to_reg(Register8::A, Register8::A); self.pc = self.pc.wrapping_add(1); self.cycles = 0; } // LD A, A
                0x78 => { self.ld_reg_to_reg(Register8::B, Register8::A); self.pc = self.pc.wrapping_add(1); self.cycles = 0; } // LD A, B
                0x79 => { self.ld_reg_to_reg(Register8::C, Register8::A); self.pc = self.pc.wrapping_add(1); self.cycles = 0; } // LD A, C
                0x7A => { self.ld_reg_to_reg(Register8::D, Register8::A); self.pc = self.pc.wrapping_add(1); self.cycles = 0; } // LD A, D
                0x7B => { self.ld_reg_to_reg(Register8::E, Register8::A); self.pc = self.pc.wrapping_add(1); self.cycles = 0; } // LD A, E
                0x7C => { self.ld_reg_to_reg(Register8::H, Register8::A); self.pc = self.pc.wrapping_add(1); self.cycles = 0; } // LD A, H
                0x7D => { self.ld_reg_to_reg(Register8::L, Register8::A); self.pc = self.pc.wrapping_add(1); self.cycles = 0; } // LD A, L
                
                0x40 => { self.ld_reg_to_reg(Register8::B, Register8::B); self.pc = self.pc.wrapping_add(1); self.cycles = 0; } // LD B, B
                0x47 => { self.ld_reg_to_reg(Register8::A, Register8::B); self.pc = self.pc.wrapping_add(1); self.cycles = 0; } // LD B, A
                0x41 => { self.ld_reg_to_reg(Register8::C, Register8::B); self.pc = self.pc.wrapping_add(1); self.cycles = 0; } // LD B, C
                0x42 => { self.ld_reg_to_reg(Register8::D, Register8::B); self.pc = self.pc.wrapping_add(1); self.cycles = 0; } // LD B, D
                0x43 => { self.ld_reg_to_reg(Register8::E, Register8::B); self.pc = self.pc.wrapping_add(1); self.cycles = 0; } // LD B, E
                0x44 => { self.ld_reg_to_reg(Register8::H, Register8::B); self.pc = self.pc.wrapping_add(1); self.cycles = 0; } // LD B, H
                0x45 => { self.ld_reg_to_reg(Register8::L, Register8::B); self.pc = self.pc.wrapping_add(1); self.cycles = 0; } // LD B, L
                
                0x4F => { self.ld_reg_to_reg(Register8::A, Register8::C); self.pc = self.pc.wrapping_add(1); self.cycles = 0; } // LD C, A
                0x48 => { self.ld_reg_to_reg(Register8::B, Register8::C); self.pc = self.pc.wrapping_add(1); self.cycles = 0; } // LD C, B
                0x49 => { self.ld_reg_to_reg(Register8::C, Register8::C); self.pc = self.pc.wrapping_add(1); self.cycles = 0; } // LD C, C
                0x4A => { self.ld_reg_to_reg(Register8::D, Register8::C); self.pc = self.pc.wrapping_add(1); self.cycles = 0; } // LD C, D
                0x4B => { self.ld_reg_to_reg(Register8::E, Register8::C); self.pc = self.pc.wrapping_add(1); self.cycles = 0; } // LD C, E
                0x4C => { self.ld_reg_to_reg(Register8::H, Register8::C); self.pc = self.pc.wrapping_add(1); self.cycles = 0; } // LD C, H
                0x4D => { self.ld_reg_to_reg(Register8::L, Register8::C); self.pc = self.pc.wrapping_add(1); self.cycles = 0; } // LD C, L
                
                0x57 => { self.ld_reg_to_reg(Register8::A, Register8::D); self.pc = self.pc.wrapping_add(1); self.cycles = 0; } // LD D, A
                0x50 => { self.ld_reg_to_reg(Register8::B, Register8::D); self.pc = self.pc.wrapping_add(1); self.cycles = 0; } // LD D, B
                0x51 => { self.ld_reg_to_reg(Register8::C, Register8::D); self.pc = self.pc.wrapping_add(1); self.cycles = 0; } // LD D, C
                0x52 => { self.ld_reg_to_reg(Register8::D, Register8::D); self.pc = self.pc.wrapping_add(1); self.cycles = 0; } // LD D, D
                0x53 => { self.ld_reg_to_reg(Register8::E, Register8::D); self.pc = self.pc.wrapping_add(1); self.cycles = 0; } // LD D, E
                0x54 => { self.ld_reg_to_reg(Register8::H, Register8::D); self.pc = self.pc.wrapping_add(1); self.cycles = 0; } // LD D, H
                0x55 => { self.ld_reg_to_reg(Register8::L, Register8::D); self.pc = self.pc.wrapping_add(1); self.cycles = 0; } // LD D, L
                
                0x5F => { self.ld_reg_to_reg(Register8::A, Register8::E); self.pc = self.pc.wrapping_add(1); self.cycles = 0; } // LD E, A
                0x58 => { self.ld_reg_to_reg(Register8::B, Register8::E); self.pc = self.pc.wrapping_add(1); self.cycles = 0; } // LD E, B
                0x59 => { self.ld_reg_to_reg(Register8::C, Register8::E); self.pc = self.pc.wrapping_add(1); self.cycles = 0; } // LD E, C
                0x5A => { self.ld_reg_to_reg(Register8::D, Register8::E); self.pc = self.pc.wrapping_add(1); self.cycles = 0; } // LD E, D
                0x5B => { self.ld_reg_to_reg(Register8::E, Register8::E); self.pc = self.pc.wrapping_add(1); self.cycles = 0; } // LD E, E
                0x5C => { self.ld_reg_to_reg(Register8::H, Register8::E); self.pc = self.pc.wrapping_add(1); self.cycles = 0; } // LD E, H
                0x5D => { self.ld_reg_to_reg(Register8::L, Register8::E); self.pc = self.pc.wrapping_add(1); self.cycles = 0; } // LD E, L
                
                0x67 => { self.ld_reg_to_reg(Register8::A, Register8::H); self.pc = self.pc.wrapping_add(1); self.cycles = 0; } // LD H, A
                0x60 => { self.ld_reg_to_reg(Register8::B, Register8::H); self.pc = self.pc.wrapping_add(1); self.cycles = 0; } // LD H, B
                0x61 => { self.ld_reg_to_reg(Register8::C, Register8::H); self.pc = self.pc.wrapping_add(1); self.cycles = 0; } // LD H, C
                0x62 => { self.ld_reg_to_reg(Register8::D, Register8::H); self.pc = self.pc.wrapping_add(1); self.cycles = 0; } // LD H, D
                0x63 => { self.ld_reg_to_reg(Register8::E, Register8::H); self.pc = self.pc.wrapping_add(1); self.cycles = 0; } // LD H, E
                0x64 => { self.ld_reg_to_reg(Register8::H, Register8::H); self.pc = self.pc.wrapping_add(1); self.cycles = 0; } // LD H, H
                0x65 => { self.ld_reg_to_reg(Register8::L, Register8::H); self.pc = self.pc.wrapping_add(1); self.cycles = 0; } // LD H, L
                
                0x6F => { self.ld_reg_to_reg(Register8::A, Register8::L); self.pc = self.pc.wrapping_add(1); self.cycles = 0; } // LD L, A
                0x68 => { self.ld_reg_to_reg(Register8::B, Register8::L); self.pc = self.pc.wrapping_add(1); self.cycles = 0; } // LD L, B
                0x69 => { self.ld_reg_to_reg(Register8::C, Register8::L); self.pc = self.pc.wrapping_add(1); self.cycles = 0; } // LD L, C
                0x6A => { self.ld_reg_to_reg(Register8::D, Register8::L); self.pc = self.pc.wrapping_add(1); self.cycles = 0; } // LD L, D
                0x6B => { self.ld_reg_to_reg(Register8::E, Register8::L); self.pc = self.pc.wrapping_add(1); self.cycles = 0; } // LD L, E
                0x6C => { self.ld_reg_to_reg(Register8::H, Register8::L); self.pc = self.pc.wrapping_add(1); self.cycles = 0; } // LD L, H
                0x6D => { self.ld_reg_to_reg(Register8::L, Register8::L); self.pc = self.pc.wrapping_add(1); self.cycles = 0; } // LD L, L
                
                // 8-bit arithmetic instructions
                0x87 => { self.a = self.add_8_bit(self.a, self.a); self.pc = self.pc.wrapping_add(1); self.cycles = 0;} // ADD A, A
                0x80 => { self.a = self.add_8_bit(self.a, self.b); self.pc = self.pc.wrapping_add(1); self.cycles = 0; } // ADD A, B
                0x81 => { self.a = self.add_8_bit(self.a, self.c); self.pc = self.pc.wrapping_add(1); self.cycles = 0; } // ADD A, C
                0x82 => { self.a = self.add_8_bit(self.a, self.d); self.pc = self.pc.wrapping_add(1); self.cycles = 0; } // ADD A, D
                0x83 => { self.a = self.add_8_bit(self.a, self.e); self.pc = self.pc.wrapping_add(1); self.cycles = 0; } // ADD A, E
                0x84 => { self.a = self.add_8_bit(self.a, self.h); self.pc = self.pc.wrapping_add(1); self.cycles = 0; } // ADD A, H
                0x85 => { self.a = self.add_8_bit(self.a, self.l); self.pc = self.pc.wrapping_add(1); self.cycles = 0; } // ADD A, L
                0x86 => {let addr = ((self.h as u16) << 8 | self.l as u16); self.a = self.add_8_bit(self.a, self.bus.read(addr)); self.pc = self.pc.wrapping_add(1); self.cycles = 1; } // ADD A, (HL)

                0x8F => { self.a = self.adc_8_bit(self.a, self.a); self.pc = self.pc.wrapping_add(1); self.cycles = 0; } // ADC A, B
                0x88 => { self.a = self.adc_8_bit(self.a, self.b); self.pc = self.pc.wrapping_add(1); self.cycles = 0; } // ADC A, B
                0x89 => { self.a = self.adc_8_bit(self.a, self.c); self.pc = self.pc.wrapping_add(1); self.cycles = 0; } // ADC A, C
                0x8A => { self.a = self.adc_8_bit(self.a, self.d); self.pc = self.pc.wrapping_add(1); self.cycles = 0; } // ADC A, D
                0x8B => { self.a = self.adc_8_bit(self.a, self.e); self.pc = self.pc.wrapping_add(1); self.cycles = 0; } // ADC A, E
                0x8C => { self.a = self.adc_8_bit(self.a, self.h); self.pc = self.pc.wrapping_add(1); self.cycles = 0; } // ADC A, H
                0x8D => { self.a = self.adc_8_bit(self.a, self.l); self.pc = self.pc.wrapping_add(1); self.cycles = 0; } // ADC A, L
                0x8E => {let addr = ((self.h as u16) << 8 | self.l as u16); self.a = self.adc_8_bit(self.a, self.bus.read(addr)); self.pc = self.pc.wrapping_add(1); self.cycles = 1; } // ADC A, (HL)

                0x97 => { self.a = self.sub_8_bit(self.a, self.a); self.pc = self.pc.wrapping_add(1); self.cycles = 0; } // SUB A, A
                0x90 => { self.a = self.sub_8_bit(self.a, self.b); self.pc = self.pc.wrapping_add(1); self.cycles = 0; } // SUB A, B
                0x91 => { self.a = self.sub_8_bit(self.a, self.c); self.pc = self.pc.wrapping_add(1); self.cycles = 0; } // SUB A, C
                0x92 => { self.a = self.sub_8_bit(self.a, self.d); self.pc = self.pc.wrapping_add(1); self.cycles = 0; } // SUB A, D
                0x93 => { self.a = self.sub_8_bit(self.a, self.e); self.pc = self.pc.wrapping_add(1); self.cycles = 0; } // SUB A, E
                0x94 => { self.a = self.sub_8_bit(self.a, self.h); self.pc = self.pc.wrapping_add(1); self.cycles = 0; } // SUB A, H
                0x95 => { self.a = self.sub_8_bit(self.a, self.l); self.pc = self.pc.wrapping_add(1); self.cycles = 0; } // SUB A, L
                0x96 => {let addr = ((self.h as u16) << 8 | self.l as u16); self.a = self.sub_8_bit(self.a, self.bus.read(addr)); self.pc = self.pc.wrapping_add(1); self.cycles = 1; } // SUB A, (HL)

                0x9F => { self.a = self.sbc_8_bit(self.a, self.a); self.pc = self.pc.wrapping_add(1); self.cycles = 0; } // SBC A, A
                0x98 => { self.a = self.sbc_8_bit(self.a, self.b); self.pc = self.pc.wrapping_add(1); self.cycles = 0; } // SBC A, B
                0x99 => { self.a = self.sbc_8_bit(self.a, self.c); self.pc = self.pc.wrapping_add(1); self.cycles = 0; } // SBC A, C
                0x9A => { self.a = self.sbc_8_bit(self.a, self.d); self.pc = self.pc.wrapping_add(1); self.cycles = 0; } // SBC A, D
                0x9B => { self.a = self.sbc_8_bit(self.a, self.e); self.pc = self.pc.wrapping_add(1); self.cycles = 0; } // SBC A, E
                0x9C => { self.a = self.sbc_8_bit(self.a, self.h); self.pc = self.pc.wrapping_add(1); self.cycles = 0; } // SBC A, H
                0x9D => { self.a = self.sbc_8_bit(self.a, self.l); self.pc = self.pc.wrapping_add(1); self.cycles = 0; } // SBC A, L
                0x9E => {let addr = ((self.h as u16) << 8 | self.l as u16); self.a = self.sbc_8_bit(self.a, self.bus.read(addr)); self.pc = self.pc.wrapping_add(1); self.cycles = 1; } // SBC A, (HL)

                0xA7 => { self.a = self.and_8_bit(self.a, self.a); self.pc = self.pc.wrapping_add(1); self.cycles = 0; } // AND A, A
                0xA0 => { self.a = self.and_8_bit(self.a, self.b); self.pc = self.pc.wrapping_add(1); self.cycles = 0; } // AND A, B
                0xA1 => { self.a = self.and_8_bit(self.a, self.c); self.pc = self.pc.wrapping_add(1); self.cycles = 0; } // AND A, C
                0xA2 => { self.a = self.and_8_bit(self.a, self.d); self.pc = self.pc.wrapping_add(1); self.cycles = 0; } // AND A, D
                0xA3 => { self.a = self.and_8_bit(self.a, self.e); self.pc = self.pc.wrapping_add(1); self.cycles = 0; } // AND A, E
                0xA4 => { self.a = self.and_8_bit(self.a, self.h); self.pc = self.pc.wrapping_add(1); self.cycles = 0; } // AND A, H
                0xA5 => { self.a = self.and_8_bit(self.a, self.l); self.pc = self.pc.wrapping_add(1); self.cycles = 0; } // AND A, L
                0xA6 => {let addr = ((self.h as u16) << 8 | self.l as u16); self.a = self.and_8_bit(self.a, self.bus.read(addr)); self.pc = self.pc.wrapping_add(1); self.cycles = 1; } // AND A, (HL)

                0xAF => { self.a = self.xor_8_bit(self.a, self.a); self.pc = self.pc.wrapping_add(1); self.cycles = 0; } // XOR A, A
                0xA8 => { self.a = self.xor_8_bit(self.a, self.b); self.pc = self.pc.wrapping_add(1); self.cycles = 0; } // XOR A, B
                0xA9 => { self.a = self.xor_8_bit(self.a, self.c); self.pc = self.pc.wrapping_add(1); self.cycles = 0; } // XOR A, C
                0xAA => { self.a = self.xor_8_bit(self.a, self.d); self.pc = self.pc.wrapping_add(1); self.cycles = 0; } // XOR A, D
                0xAB => { self.a = self.xor_8_bit(self.a, self.e); self.pc = self.pc.wrapping_add(1); self.cycles = 0; } // XOR A, E
                0xAC => { self.a = self.xor_8_bit(self.a, self.h); self.pc = self.pc.wrapping_add(1); self.cycles = 0; } // XOR A, H
                0xAD => { self.a = self.xor_8_bit(self.a, self.l); self.pc = self.pc.wrapping_add(1); self.cycles = 0; } // XOR A, L
                0xAE => {let addr = ((self.h as u16) << 8 | self.l as u16); self.a = self.xor_8_bit(self.a, self.bus.read(addr)); self.pc = self.pc.wrapping_add(1); self.cycles = 1; } // XOR A, (HL)

                0xB7 => { self.a = self.or_8_bit(self.a, self.a); self.pc = self.pc.wrapping_add(1); self.cycles = 0; } // OR A, A
                0xB0 => { self.a = self.or_8_bit(self.a, self.b); self.pc = self.pc.wrapping_add(1); self.cycles = 0; } // OR A, B
                0xB1 => { self.a = self.or_8_bit(self.a, self.c); self.pc = self.pc.wrapping_add(1); self.cycles = 0; } // OR A, C
                0xB2 => { self.a = self.or_8_bit(self.a, self.d); self.pc = self.pc.wrapping_add(1); self.cycles = 0; } // OR A, D
                0xB3 => { self.a = self.or_8_bit(self.a, self.e); self.pc = self.pc.wrapping_add(1); self.cycles = 0; } // OR A, E
                0xB4 => { self.a = self.or_8_bit(self.a, self.h); self.pc = self.pc.wrapping_add(1); self.cycles = 0; } // OR A, H
                0xB5 => { self.a = self.or_8_bit(self.a, self.l); self.pc = self.pc.wrapping_add(1); self.cycles = 0; } // OR A, L
                0xB6 => {let addr = ((self.h as u16) << 8 | self.l as u16); self.a = self.or_8_bit(self.a, self.bus.read(addr)); self.pc = self.pc.wrapping_add(1); self.cycles = 1; } // OR A, (HL)

                0xBF => { _ = self.sub_8_bit(self.a, self.a); self.pc = self.pc.wrapping_add(1); self.cycles = 0; } // CP A, A
                0xB8 => { _ = self.sub_8_bit(self.a, self.b); self.pc = self.pc.wrapping_add(1); self.cycles = 0; } // CP A, B
                0xB9 => { _ = self.sub_8_bit(self.a, self.c); self.pc = self.pc.wrapping_add(1); self.cycles = 0; } // CP A, C
                0xBA => { _ = self.sub_8_bit(self.a, self.d); self.pc = self.pc.wrapping_add(1); self.cycles = 0; } // CP A, D
                0xBB => { _ = self.sub_8_bit(self.a, self.e); self.pc = self.pc.wrapping_add(1); self.cycles = 0; } // CP A, E
                0xBC => { _ = self.sub_8_bit(self.a, self.h); self.pc = self.pc.wrapping_add(1); self.cycles = 0; } // CP A, H
                0xBD => { _ = self.sub_8_bit(self.a, self.l); self.pc = self.pc.wrapping_add(1); self.cycles = 0; } // CP A, L
                0xBE => {let addr = ((self.h as u16) << 8 | self.l as u16); _ = self.sub_8_bit(self.a, self.bus.read(addr)); self.pc = self.pc.wrapping_add(1); self.cycles = 1; } // CP A, (HL)

                // immediate arithmetic instructions
                0xC6 => { self.a = self.add_8_bit(self.a, self.bus.read((self.pc + 1) as u16)); self.pc = self.pc.wrapping_add(2); self.cycles = 1;} // ADD d8
                0xCE => { self.a = self.adc_8_bit(self.a, self.bus.read((self.pc + 1) as u16)); self.pc = self.pc.wrapping_add(2); self.cycles = 1;} // ADC d8
                0xD6 => { self.a = self.sub_8_bit(self.a, self.bus.read((self.pc + 1) as u16)); self.pc = self.pc.wrapping_add(2); self.cycles = 1;} // SUB d8
                0xDE => { self.a = self.sbc_8_bit(self.a, self.bus.read((self.pc + 1) as u16)); self.pc = self.pc.wrapping_add(2); self.cycles = 1;} // SBC d8
                0xE6 => { self.a = self.and_8_bit(self.a, self.bus.read((self.pc + 1) as u16)); self.pc = self.pc.wrapping_add(2); self.cycles = 1;} // AND d8
                0xEE => { self.a = self.xor_8_bit(self.a, self.bus.read((self.pc + 1) as u16)); self.pc = self.pc.wrapping_add(2); self.cycles = 1;} // XOR d8
                0xF6 => { self.a = self.or_8_bit(self.a, self.bus.read((self.pc + 1) as u16)); self.pc = self.pc.wrapping_add(2); self.cycles = 1;} // OR d8
                0xFE => { _ = self.sub_8_bit(self.a, self.bus.read((self.pc + 1) as u16)); self.pc = self.pc.wrapping_add(2); self.cycles = 1;} // CP d8
                
                // Jump instructions
                0x18 => { self.jump_r_s8(self.bus.read((self.pc + 1) as u16) as i8); self.cycles = 2; } // JR s8
                0x20 => { if (self.f & 0x80) == 0 { self.jump_r_s8(self.bus.read((self.pc + 1) as u16) as i8); self.cycles = 2; } else { self.pc = self.pc.wrapping_add(2); self.cycles = 1; }} // JR NZ s8
                0x30 => { if (self.f & 0x10) == 0 { self.jump_r_s8(self.bus.read((self.pc + 1) as u16) as i8); self.cycles = 2; } else { self.pc = self.pc.wrapping_add(2); self.cycles = 1; }} // JR NC s8
                0x28 => { if (self.f & 0x80) == 0x80 { self.jump_r_s8(self.bus.read((self.pc + 1) as u16) as i8); self.cycles = 2; } else { self.pc = self.pc.wrapping_add(2); self.cycles = 1; }} // JR Z s8
                0x38 => { if (self.f & 0x10) == 0x10 { self.jump_r_s8(self.bus.read((self.pc + 1) as u16) as i8); self.cycles = 2; } else { self.pc = self.pc.wrapping_add(2); self.cycles = 1; }} // JR C s8
                0xE9 => { self.pc = ((self.h as u16) << 8) | (self.l as u16); self.cycles = 0; } // JP HL

                0xC3 => { self.jump_nn(); self.cycles = 2; } // JP a16
                0xC2 => { if self.f & 0x80 == 0 { self.jump_nn(); self.cycles = 3;} else { self.pc = self.pc.wrapping_add(3); self.cycles = 2 } } // JP NZ a16
                0xD2 => { if self.f & 0x10 == 0 { self.jump_nn(); self.cycles = 3;} else { self.pc = self.pc.wrapping_add(3); self.cycles = 2 } } // JP NC a16
                0xCA => { if self.f & 0x80 == 0x80 { self.jump_nn(); self.cycles = 3;} else { self.pc = self.pc.wrapping_add(3); self.cycles = 2 } } // JP Z a16
                0xDA => { if self.f & 0x10 == 0x10 { self.jump_nn(); self.cycles = 3;} else { self.pc = self.pc.wrapping_add(3); self.cycles = 2 } } // JP C a16

                // Call/Ret instructions
                0xCD => { self.call(); self.cycles = 5; } // CALL a16
                0xC4 => { if self.f & 0x80 == 0 { self.call(); self.cycles = 5; } else { self.pc = self.pc.wrapping_add(3); self.cycles = 2; } } // CALL NZ a16
                0xD4 => { if self.f & 0x10 == 0 { self.call(); self.cycles = 5; } else { self.pc = self.pc.wrapping_add(3); self.cycles = 2; } } // CALL NC a16
                0xCC => { if self.f & 0x80 == 0x80 { self.call(); self.cycles = 5; } else { self.pc = self.pc.wrapping_add(3); self.cycles = 2; } } // CALL Z a16
                0xDC => { if self.f & 0x10 == 0x10 { self.call(); self.cycles = 5; } else { self.pc = self.pc.wrapping_add(3); self.cycles = 2; } } // CALL C a16

                0xC9 => { self.pc = self.pop_16bit_value(); self.cycles = 3; } // RET
                0xD9 => { self.pc = self.pop_16bit_value(); self.ime = true; self.cycles = 3; } // RETI
                0xC0 => { if self.f & 0x80 == 0 { self.pc = self.pop_16bit_value(); self.cycles = 4; } else { self.pc = self.pc.wrapping_add(1); self.cycles = 1; } } // RET NZ
                0xD0 => { if self.f & 0x10 == 0 { self.pc = self.pop_16bit_value(); self.cycles = 4; } else { self.pc = self.pc.wrapping_add(1); self.cycles = 1; } } // RET NC
                0xC8=> { if self.f & 0x80 == 0x80 { self.pc = self.pop_16bit_value(); self.cycles = 4; } else { self.pc = self.pc.wrapping_add(1); self.cycles = 1; } } // RET Z
                0xD8 => { if self.f & 0x10 == 0x10 { self.pc = self.pop_16bit_value(); self.cycles = 4; } else { self.pc = self.pc.wrapping_add(1); self.cycles = 1; } } // RET C
                                
                // increment/decrement instructions
                0x3C => { self.a = self.increment_8bit_register(self.a); self.pc = self.pc.wrapping_add(1); self.cycles = 0; } // INC A
                0x04 => { self.b = self.increment_8bit_register(self.b); self.pc = self.pc.wrapping_add(1); self.cycles = 0; } // INC B
                0x0C => { self.c = self.increment_8bit_register(self.c); self.pc = self.pc.wrapping_add(1); self.cycles = 0; } // INC C
                0x14 => { self.d = self.increment_8bit_register(self.d); self.pc = self.pc.wrapping_add(1); self.cycles = 0; } // INC D
                0x1C => { self.e = self.increment_8bit_register(self.e); self.pc = self.pc.wrapping_add(1); self.cycles = 0; } // INC E
                0x24 => { self.h = self.increment_8bit_register(self.h); self.pc = self.pc.wrapping_add(1); self.cycles = 0; } // INC H
                0x2C => { self.l = self.increment_8bit_register(self.l); self.pc = self.pc.wrapping_add(1); self.cycles = 0; } // INC L
                
                0x3D => { self.a = self.decrement_8bit_register(self.a); self.pc = self.pc.wrapping_add(1); self.cycles = 0; } // DEC A
                0x05 => { self.b = self.decrement_8bit_register(self.b); self.pc = self.pc.wrapping_add(1); self.cycles = 0; } // DEC B
                0x0D => { self.c = self.decrement_8bit_register(self.c); self.pc = self.pc.wrapping_add(1); self.cycles = 0; } // DEC C
                0x15 => { self.d = self.decrement_8bit_register(self.d); self.pc = self.pc.wrapping_add(1); self.cycles = 0; } // DEC D
                0x1D => { self.e = self.decrement_8bit_register(self.e); self.pc = self.pc.wrapping_add(1); self.cycles = 0; } // DEC E
                0x25 => { self.h = self.decrement_8bit_register(self.h); self.pc = self.pc.wrapping_add(1); self.cycles = 0; } // DEC H
                0x2D => { self.l = self.decrement_8bit_register(self.l); self.pc = self.pc.wrapping_add(1); self.cycles = 0; } // DEC L
                
                0x03 => { self.increment_16_bit_register(self.b, self.c, Register16::BC); self.pc = self.pc.wrapping_add(1); self.cycles = 1; } // INC BC
                0x13 => { self.increment_16_bit_register(self.d, self.e, Register16::DE); self.pc = self.pc.wrapping_add(1); self.cycles = 1; } // INC DE
                0x23 => { self.increment_16_bit_register(self.h, self.l, Register16::HL); self.pc = self.pc.wrapping_add(1); self.cycles = 1; } // INC HL
                0x33 => { let high = (self.sp >> 8) as u8; let low = (self.sp & 0xFF) as u8; self.increment_16_bit_register(high, low, Register16::SP); self.pc = self.pc.wrapping_add(1); self.cycles = 1; } // INC SP
                
                0x0B => { self.decrement_16_bit_register(self.b, self.c, Register16::BC); self.pc = self.pc.wrapping_add(1); self.cycles = 1; } // DEC BC
                0x1B => { self.decrement_16_bit_register(self.d, self.e, Register16::DE); self.pc = self.pc.wrapping_add(1); self.cycles = 1; } // DEC DE
                0x2B => { self.decrement_16_bit_register(self.h, self.l, Register16::HL); self.pc = self.pc.wrapping_add(1); self.cycles = 1; } // DEC HL
                0x3B => { let high = (self.sp >> 8) as u8; let low = (self.sp & 0xFF) as u8; self.decrement_16_bit_register(high, low, Register16::SP); self.pc = self.pc.wrapping_add(1); self.cycles = 1; }
                
                0x34 => { self.inc_memory_at_hl(); self.pc = self.pc.wrapping_add(1); self.cycles = 2; } // INC (HL)
                0x35 => { self.dec_memory_at_hl(); self.pc = self.pc.wrapping_add(1); self.cycles = 2; } // DEC (HL)

                // 16-bit adds
                0x09 => { self.add_16bit_to_hl(Register16::BC); self.pc = self.pc.wrapping_add(1); self.cycles = 1; } // ADD HL, BC
                0x19 => { self.add_16bit_to_hl(Register16::DE); self.pc = self.pc.wrapping_add(1); self.cycles = 1; } // ADD HL, DE
                0x29 => { self.add_16bit_to_hl(Register16::HL); self.pc = self.pc.wrapping_add(1); self.cycles = 1; } // ADD HL, HL
                0x39 => { self.add_16bit_to_hl(Register16::SP); self.pc = self.pc.wrapping_add(1); self.cycles = 1; } // ADD HL, SP
                0x07 => { self.rlca(); self.pc = self.pc.wrapping_add(1); self.cycles = 0; } // RLCA
                0x17 => { self.rla(); self.pc = self.pc.wrapping_add(1); self.cycles = 0; } // RLA
                0x0F => { self.rrca(); self.pc = self.pc.wrapping_add(1); self.cycles = 0; } // RRCA
                0x1F => { self.rra(); self.pc = self.pc.wrapping_add(1); self.cycles = 0; } // RRA
                
                // Carry flag instructions
                0x2F => { self.a = !self.a; self.f |= 0x60; self.pc = self.pc.wrapping_add(1); self.cycles = 0; } // CPL
                0x27 => { self.da_adjust(); self.pc = self.pc.wrapping_add(1); self.cycles = 0; } // DAA
                0x37 => { self.f = (self.f & 0x8F) | 0x10; self.pc = self.pc.wrapping_add(1); self.cycles = 1; } // SCF (set the carry flag)
                0x3F => { self.f = (self.f ^ 0x10) & 0x9F; self.pc = self.pc.wrapping_add(1); self.cycles = 1; } // CCF (flip the carry flag)

                // Stack instructions
                0xC1 => { self.pop(Register16::BC); self.pc = self.pc.wrapping_add(1); self.cycles = 2; } // POP BC
                0xD1 => { self.pop(Register16::DE); self.pc = self.pc.wrapping_add(1); self.cycles = 2; } // POP DE
                0xE1 => { self.pop(Register16::HL); self.pc = self.pc.wrapping_add(1); self.cycles = 2; } // POP HL
                0xF1 => { self.pop(Register16::AF); self.pc = self.pc.wrapping_add(1); self.cycles = 2; } // POP AF

                0xC5 => { self.push(Register16::BC); self.pc = self.pc.wrapping_add(1); self.cycles = 2; } // PUSH BC
                0xD5 => { self.push(Register16::DE); self.pc = self.pc.wrapping_add(1); self.cycles = 2; } // PUSH DE
                0xE5 => { self.push(Register16::HL); self.pc = self.pc.wrapping_add(1); self.cycles = 2; } // PUSH HL
                0xF5 => { self.push(Register16::AF); self.pc = self.pc.wrapping_add(1); self.cycles = 2; } // PUSH AF

                // Interrupt master enable setters
                0xF3 => { self.ime = false; self.pc = self.pc.wrapping_add(1); self.cycles = 0; } // DI
                0xFB => { self.ime = true; self.pc = self.pc.wrapping_add(1); self.cycles = 0; } // EI

                // Reset instructions
                0xC7 => { let return_address = self.pc.wrapping_add(1); self.push_16bit_value(return_address); self.pc = 0x0000; self.cycles = 3; } // RST 0
                0xCF => { let return_address = self.pc.wrapping_add(1); self.push_16bit_value(return_address); self.pc = 0x0008; self.cycles = 3; } // RST 1
                0xD7 => { let return_address = self.pc.wrapping_add(1); self.push_16bit_value(return_address); self.pc = 0x0010; self.cycles = 3; } // RST 2
                0xDF => { let return_address = self.pc.wrapping_add(1); self.push_16bit_value(return_address); self.pc = 0x0018; self.cycles = 3; } // RST 3
                0xE7 => { let return_address = self.pc.wrapping_add(1); self.push_16bit_value(return_address); self.pc = 0x0020; self.cycles = 3; } // RST 4
                0xEF => { let return_address = self.pc.wrapping_add(1); self.push_16bit_value(return_address); self.pc = 0x0028; self.cycles = 3; } // RST 5
                0xF7 => { let return_address = self.pc.wrapping_add(1); self.push_16bit_value(return_address); self.pc = 0x0030; self.cycles = 3; } // RST 6
                0xFF => { let return_address = self.pc.wrapping_add(1); self.push_16bit_value(return_address); self.pc = 0x0038; self.cycles = 3; } // RST 7

                // CB-prefixed instructions
                0xCB => {
                    // The byte after the CB prefix selects the operation and operand.
                    let op = self.bus.read(self.pc.wrapping_add(1) as u16);
                    let family = op >> 6;
                    let operation = (op >> 3) & 0b111;
                    let target = op & 0b111;

                    // CB instructions are always two bytes long.
                    self.pc = self.pc.wrapping_add(2);

                    // This assumes `self.cycles` stores the remaining T-cycles minus one,
                    // as is common in instruction-dispatch loops. Check against your CPU loop.
                    // CB instructions: documented T-cycles minus one.
                    self.cycles = match (family, target) {
                        (1, 6) => 11, // BIT b,(HL): 12 cycles
                        (_, 6) => 15, // Other (HL) operations: 16 cycles
                        _ => 7,       // Register operations: 8 cycles
                    };

                    const FLAG_Z: u8 = 0x80;
                    const FLAG_H: u8 = 0x20;
                    const FLAG_C: u8 = 0x10;

                    // Resolve the 8-bit operand. Target 6 means memory[HL], NOT the HL pair.
                    let hl = ((self.h as u16) << 8) | self.l as u16;
                    let value = match target {
                        0 => self.b,
                        1 => self.c,
                        2 => self.d,
                        3 => self.e,
                        4 => self.h,
                        5 => self.l,
                        6 => self.bus.read(hl),
                        7 => self.a,
                        _ => unreachable!(),
                    };

                    match family {
                        // 00: rotate and shift group
                        0 => {
                            let old_carry = (self.f & FLAG_C) != 0;
                            let bit_7 = (value & 0x80) != 0;
                            let bit_0 = (value & 0x01) != 0;

                            let (result, carry) = match operation {
                                0 => (value.rotate_left(1), bit_7), // RLC
                                1 => (value.rotate_right(1), bit_0), // RRC
                                2 => ((value << 1) | u8::from(old_carry), bit_7), // RL
                                3 => ((value >> 1) | (u8::from(old_carry) << 7), bit_0), // RR
                                4 => (value << 1, bit_7), // SLA
                                5 => ((value >> 1) | (value & 0x80), bit_0), // SRA
                                6 => (value.rotate_left(4), false), // SWAP
                                7 => (value >> 1, bit_0), // SRL
                                _ => unreachable!(),
                            };

                            // All CB rotate/shift operations clear N and H.
                            self.f = (if result == 0 { FLAG_Z } else { 0 })
                                | (if carry { FLAG_C } else { 0 });

                            match target {
                                0 => self.b = result,
                                1 => self.c = result,
                                2 => self.d = result,
                                3 => self.e = result,
                                4 => self.h = result,
                                5 => self.l = result,
                                6 => self.bus.write(hl, result),
                                7 => self.a = result,
                                _ => unreachable!(),
                            }
                        }

                        // 01: BIT b, r -- test only; do not write the operand back.
                        1 => {
                            let bit = 1u8 << operation;
                            let zero = (value & bit) == 0;

                            // Z is the test result, N=0, H=1, C is preserved.
                            self.f = (self.f & FLAG_C)
                                | FLAG_H
                                | (if zero { FLAG_Z } else { 0 });
                        }

                        // 10: RES b, r -- reset a bit; flags are untouched.
                        2 => {
                            let result = value & !(1u8 << operation);
                            match target {
                                0 => self.b = result,
                                1 => self.c = result,
                                2 => self.d = result,
                                3 => self.e = result,
                                4 => self.h = result,
                                5 => self.l = result,
                                6 => self.bus.write(hl, result),
                                7 => self.a = result,
                                _ => unreachable!(),
                            }
                        }

                        // 11: SET b, r -- set a bit; flags are untouched.
                        3 => {
                            let result = value | (1u8 << operation);
                            match target {
                                0 => self.b = result,
                                1 => self.c = result,
                                2 => self.d = result,
                                3 => self.e = result,
                                4 => self.h = result,
                                5 => self.l = result,
                                6 => self.bus.write(hl, result),
                                7 => self.a = result,
                                _ => unreachable!(),
                            }
                        }

                        _ => unreachable!(),
                    }
                },

                _ => { panic!("Unknown opcode: {}", opcode) } // Unimplemented, panic
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
            Register16::AF => {
                self.a = (value >> 8) as u8;   // High byte goes to `a`
                self.f = (value & 0xF0) as u8; // Low byte goes to `f`
            },
        }
    }

    // Helper method to set flags for an 8-bit addition (including optional carry)
    fn set_flags_for_addition(&mut self, reg1: u8, reg2: u8, carry_input: bool) {
        let carry_val = if carry_input { 1 } else { 0 };
        let result = reg1.wrapping_add(reg2).wrapping_add(carry_val);
        
        // Preserve bits 0-3 of the flags register
        self.f &= 0xF0; 
        
        // Set Zero Flag (bit 7): 1 if result is 0
        if result == 0 {
            self.f |= 0x80;
        } else {
            self.f &= !0x80;
        }
        
        // Set Half-Carry flag (bit 5): 1 if lower 4 bits caused carry
        if (reg1 as u16 & 0x0F) + (reg2 as u16 & 0x0F) + carry_val as u16 > 0x0F {
            self.f |= 0x20;
        } else {
            self.f &= !0x20;
        }
        
        // Set Carry Flag (bit 4): 1 if overflow occurred
        if (reg1 as u16 + reg2 as u16 + carry_val as u16) > 0xFF {
            self.f |= 0x10;
        } else {
            self.f &= !0x10;
        }
        
        // Clear Subtract flag (bit 6)
        self.f &= !0x40;
    }

    // Helper method for 8-bit add instructions
    fn add_8_bit(&mut self, reg1: u8, reg2: u8) -> u8 {
        let result = reg1.wrapping_add(reg2);
        self.set_flags_for_addition(reg1, reg2, false);
        result
    }

    fn adc_8_bit(&mut self, reg1: u8, reg2: u8) -> u8 {
        let carry_input = (self.f & 0x10) != 0;
        let result = reg1.wrapping_add(reg2).wrapping_add(if carry_input { 1 } else { 0 });
        self.set_flags_for_addition(reg1, reg2, carry_input);
        result
    }

    fn sub_8_bit(&mut self, reg1: u8, reg2: u8) -> u8 {
        let result = reg1.wrapping_sub(reg2);

        // Preserve bits 0-3 of the flags register
        self.f &= 0xF0;

        // Set Zero Flag (bit 7): 1 if result is 0
        if result == 0 {
            self.f |= 0x80;
        } else {
            self.f &= !0x80;
        }

        // Set Half-Carry flag (bit 5): 1 if lower 4 bits caused a borrow
        if (reg1 as u16 & 0x0F) < (reg2 as u16 & 0x0F) {
            self.f |= 0x20;
        } else {
            self.f &= !0x20;
        }

        // Set Carry Flag (bit 4): 1 if an 8-bit borrow occurred
        if reg1 < reg2 {
            self.f |= 0x10;
        } else {
            self.f &= !0x10;
        }

        // Set Subtract flag (bit 6)
        self.f |= 0x40;

        result
    }

    // Helper method for 8-bit Subtract with Carry (SBC) instructions
    fn sbc_8_bit(&mut self, reg1: u8, reg2: u8) -> u8 {
        let carry_input = (self.f & 0x10) != 0;
        // The value to subtract is the register value plus the carry bit
        let val_to_sub = reg2.wrapping_add(if carry_input { 1 } else { 0 });
        let result = reg1.wrapping_sub(val_to_sub);

        // Preserve bits 0-3 of the flags register
        self.f &= 0xF0;

        // Set Zero Flag (bit 7): 1 if result is 0
        if result == 0 {
            self.f |= 0x80;
        } else {
            self.f &= !0x80;
        }

        // Set Half-Carry flag (bit 5): 1 if lower 4 bits caused a borrow
        if (reg1 as u16 & 0x0F) < (val_to_sub as u16 & 0x0F) {
            self.f |= 0x20;
        } else {
            self.f &= !0x20;
        }

        // Set Carry Flag (bit 4): 1 if an 8-bit borrow occurred
        if reg1 < val_to_sub {
            self.f |= 0x10;
        } else {
            self.f &= !0x10;
        }

        // Set Subtract flag (bit 6)
        self.f |= 0x40;

        result
    }

    // Helper method for 8-bit AND instructions
    fn and_8_bit(&mut self, reg1: u8, reg2: u8) -> u8 {
        let result = reg1 & reg2;

        // Preserve bits 0-3 of the flags register
        self.f &= 0xF0;

        // Set Zero Flag (bit 7): 1 if result is 0
        if result == 0 {
            self.f |= 0x80;
        } else {
            self.f &= !0x80;
        }

        // Set Half-Carry flag (bit 5):
        self.f |= 0x20;

        // Clear Carry Flag (bit 4)
        self.f &= !0x10;

        // Clear Subtract flag (bit 6)
        self.f &= !0x40;

        result
    }

    // Helper method for 8-bit XOR instructions
    fn xor_8_bit(&mut self, reg1: u8, reg2: u8) -> u8 {
        let result = reg1 ^ reg2;
        self.f = if result == 0 { 0x80 } else { 0x00 };
        result
    }

    // Helper method for 8-bit OR instructions
    fn or_8_bit(&mut self, reg1: u8, reg2: u8) -> u8 {
        let result = reg1 | reg2;

        // Preserve bits 0-3 of the flags register
        self.f &= 0xF0;

        // Set Zero Flag (bit 7): 1 if result is 0
        if result == 0 {
            self.f |= 0x80;
        } else {
            self.f &= !0x80;
        }

        // Clear Half-Carry flag (bit 5):
        self.f &= !0x20;

        // Clear Carry Flag (bit 4)
        self.f &= !0x10;

        // Clear Subtract flag (bit 6)
        self.f &= !0x40;

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
            Register16::SP => ((self.sp & 0xFF) as u8, (self.sp >> 8) as u8),
            Register16::AF => (self.a, self.f),
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
        let low_byte = self.bus.read((self.pc + 1) as u16) as u16;
        let high_byte = self.bus.read((self.pc + 2) as u16) as u16;
        
        // 2. Combine them into a 16-bit address
        // Shift high_byte left by 8 bits and combine with low_byte
        let target_address = (high_byte << 8) | low_byte;
        
        // 3. Update the Program Counter
        // The PC now points directly to the start of the new instruction.
        self.pc = target_address as u16;
        
        // Note: We do NOT do self.pc += 3 here. 
        // The jump instruction's job is to move the PC to the new location.https://pcpartpicker.com/product/MM7scf/jonsbo-jonsplus-z20-microatx-desktop-case-z20-black
        // If you added 3, you would skip the first 3 bytes of the target instruction.
    }

    fn jump_r_s8(&mut self, s8: i8) {
        let offset = s8 as i32;
        self.pc = (self.pc as i32 + 2 + offset) as u16;
    }

    fn rla(&mut self) {
        // 1. Capture the bit 7 of register A BEFORE the shift.
        // This value will become the new Carry flag.
        let carry_bit = (self.a & 0x80) != 0;
        
        // 2. Capture the current Carry flag.
        // This value will be moved into bit 0 of register A.
        let old_carry = (self.f & 0x10) != 0;

        // 3. Perform the left rotation.
        // Bit 0 of A is replaced by the old Carry flag.
        self.a = (self.a << 1) | (if old_carry { 1 } else { 0 });
        
        // 4. Update flags.
        // Reset all flags to 0.
        self.f = 0;
        
        // As per requirements: Zero, Subtract, and Half-carry should be 0.
        // Only the Carry flag (bit 4) is set based on the bit 7 of A before the shift.
        if carry_bit {
            self.f |= 0x10; // Set Carry flag
        }
    }

    fn rrca(&mut self) {
        // 1. Capture bit 0 of register A BEFORE the shift.
        // This value will become the new Carry flag.
        let carry_bit = (self.a & 0x01) != 0;

        // 2. Capture the current Carry flag.
        // This value will be moved into bit 7 of register A.
        let old_carry = (self.f & 0x10) != 0;

        // 3. Perform the right rotation.
        // Bit 0 moves to the carry flag (handled in step 4).
        // The old carry flag is moved into bit 7.
        self.a = (self.a >> 1) | (if old_carry { 0x80 } else { 0 });

        // 4. Update flags.
        // Clear all flags first.
        self.f = 0;

        // As per requirements: Zero (bit 7), Subtract (bit 6), and Half-carry (bit 5)
        // are set to 0. Only the Carry flag (bit 4) is updated.
        if carry_bit {
            self.f |= 0x10; // Set Carry flag
        }
    }

    fn rra(&mut self) {
        // 1. Capture bit 0 of register A BEFORE the shift.
        // This value will become the new Carry flag.
        let carry_bit = (self.a & 0x01) != 0;

        // 2. Capture the current Carry flag.
        // This value will be moved into bit 7 of register A.
        let old_carry = (self.f & 0x10) != 0;

        // 3. Perform the right rotation.
        // Bit 0 moves to the carry flag (handled in step 4).
        // The old carry flag is moved into bit 7.
        self.a = (self.a >> 1) | (if old_carry { 0x80 } else { 0 });

        // 4. Update flags.
        // Clear all flags first.
        self.f = 0;

        // As per requirements: Zero (bit 7), Subtract (bit 6), and Half-carry (bit 5)
        // are set to 0. Only the Carry flag (bit 4) is updated.
        if carry_bit {
            self.f |= 0x10; // Set Carry flag
        }
    }

    fn rlca(&mut self) {
        // 1. Capture the bit 7 of register A BEFORE the shift.
        // This value will become the new Carry flag.
        let carry_bit = (self.a & 0x80) != 0;

        // 2. Capture the current Carry flag.
        // This value will be moved into bit 0 of register A.
        let old_carry = (self.f & 0x10) != 0;

        // 3. Perform the left rotation.
        // Bit 0 is filled by the old Carry flag.
        self.a = (self.a << 1) | (if old_carry { 1 } else { 0 });

        // 4. Update flags.
        // Clear all flags first.
        self.f = 0;

        // The Carry flag (bit 4) is updated based on the bit 7 of A 
        // that was "pushed out" before the shift.
        if carry_bit {
            self.f |= 0x10; // Set Carry flag
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
        self.a = self.bus.read(self.get_16_bit_operand(self.pc) as u16);
    }

    fn ld_a_from_a8(&mut self) {
        let addr = 0xFF00 | self.bus.read(self.pc.wrapping_add(1)) as u16;
        self.a = self.bus.read(addr);
    }

    fn ld_a_from_bc(&mut self) {
        self.a = self.bus.read(((self.b as u16) << 8 | self.c as u16));
    }

    fn ld_a_from_c(&mut self) {
        self.a = self.bus.read(self.c as u16);
    }

    fn ld_a_from_de(&mut self) {
        self.a = self.bus.read(((self.d as u16) << 8 | self.e as u16));
    }

    fn ld_a_from_c_high(&mut self) {
        self.a = self.bus.read(0xFF00 + self.c as u16);
    }

    fn ld_a_to_hl_minus(&mut self) {
        // 1. Get the current address from HL
        let current_addr = ((self.h as u16) << 8 | self.l as u16) as usize;

        // 2. Store the contents of that memory location into A (the "Store" part)
        self.a = self.bus.read(current_addr as u16);

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
            self.bus.write(address as u16, self.a);
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
        self.a = self.bus.read(current_addr as u16);

        // 3. Increment the contents of HL (The "Increment" part)
        let new_hl = current_addr.wrapping_add(1);
        self.h = (new_hl >> 8) as u8;
        self.l = (new_hl & 0xFF) as u8;
    }

    fn ld_memory_at_a16_from_a(&mut self) {
        let addr = self.get_16_bit_operand(self.pc);
        self.bus.write(addr, self.a);
    }

    fn ld_memory_at_a8_from_a(&mut self) {
        let addr = 0xFF00 | self.bus.read(self.pc.wrapping_add(1)) as u16;
        self.bus.write(addr, self.a);
    }

    fn ld_memory_at_bc_from_a(&mut self) {
        let addr = (self.b as u16) << 8 | self.c as u16;
        self.bus.write(addr, self.a);
    }

    fn ld_memory_at_c_from_a(&mut self) {
        // Calculate the target address by adding the base (0xFF00) 
        // to the offset provided by register C.
        let address = 0xFF00 + (self.c as usize);

        self.bus.write(address as u16, self.a);
    }

    fn ld_memory_at_de_from_a(&mut self) {
        let addr = (self.d as u16) << 8 | self.e as u16;
        self.bus.write(addr, self.a);
    }

    fn ld_memory_at_hl_plus_from_a(&mut self) {
        // 1. Calculate the CURRENT address from the HL pair
        // We use u16 to ensure the bitwise shift and OR work correctly.
        let current_hl = ((self.h as u16) << 8) | (self.l as u16);

        // 2. Store the contents of Register A into the CURRENT address
        // Check bounds to prevent the emulator from crashing.
        self.bus.write(current_hl, self.a);

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
        let val = self.bus.read((self.pc + 1) as u16);
        self.bus.write(hl, val);
    }

    fn ld_reg_from_hl(&mut self, reg: Register8) {
        let addr = (self.h as u16) << 8 | self.l as u16;
        let val = self.bus.read(addr);
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

    fn get_16_bit_operand(&self, index: u16) -> u16 {
        let low = self.bus.read(index.wrapping_add(1)) as u16;
        let high = self.bus.read(index.wrapping_add(2)) as u16;

        low | (high << 8)
    }

    fn ld_memory_at_a16_from_sp(&mut self) {
        let addr = self.get_16_bit_operand(self.pc);
        self.bus.write(addr, (self.sp & 0xFF) as u8);
        self.bus.write(addr + 1, (self.sp >> 8) as u8);
    }

    fn da_adjust(&mut self) {
        let a_value = self.a;
        let half_carry = (self.f & 0x20) != 0;
        let carry = (self.f & 0x10) != 0;
        let subtract = (self.f & 0x40) != 0;

        let mut offset = 0u8;

        // Logic for lower nibble adjustment
        if (!subtract && (a_value & 0xF > 0x09)) || half_carry {
            offset |= 0x06;
        }

        // Logic for upper nibble adjustment
        // Note: Matching your example's specific check of > 0x99
        if (!subtract && a_value > 0x99) || carry {
            offset |= 0x60;
        }

        if !subtract {
            self.a = a_value.wrapping_add(offset);
        } else {
            self.a = a_value.wrapping_sub(offset);
        }
    }

    fn inc_memory_at_hl(&mut self) {
        let addr = ((self.h as u16) << 8) | self.l as u16;
        let val = self.bus.read(addr);
        let result = val.wrapping_add(1);

        // Preserve Carry flag (bit 4)
        self.f &= 0x10;
        // Update Zero flag (bit 7)
        if result == 0 {
            self.f |= 0x80;
        } else {
            self.f &= !0x80;
        }
        // Update Half-Carry flag (bit 5)
        if (val & 0x0F) == 0x0F {
            self.f |= 0x20;
        } else {
            self.f &= !0x20;
        }
        // Clear Subtract flag (bit 6)
        self.f &= !0x40;

        self.bus.write(addr, result);
    }

    fn dec_memory_at_hl(&mut self) {
        let addr = ((self.h as u16) << 8) | self.l as u16;
        let val = self.bus.read(addr);
        let result = val.wrapping_sub(1);

        // Preserve Carry flag (bit 4)
        self.f &= 0x10;
        // Update Zero flag (bit 7)
        if result == 0 {
            self.f |= 0x80;
        } else {
            self.f &= !0x80;
        }
        // Update Half-Carry flag (bit 5)
        if (val & 0x0F) == 0x00 {
            self.f |= 0x20;
        } else {
            self.f &= !0x20;
        }
        // Set Subtract flag (bit 6)
        self.f |= 0x40;

        self.bus.write(addr, result);
    }

    // Helper method to pop a 16-bit value from the stack into a register pair
    fn pop(&mut self, reg: Register16) {
        match reg {
            Register16::BC => {
                self.c = self.bus.read(self.sp as u16);
                self.sp += 1;
                self.b = self.bus.read(self.sp as u16);
                self.sp += 1;
            }
            Register16::DE => {
                self.e = self.bus.read(self.sp as u16);
                self.sp += 1;
                self.d = self.bus.read(self.sp as u16);
                self.sp += 1;
            }
            Register16::HL => {
                self.l = self.bus.read(self.sp as u16);
                self.sp += 1;
                self.h = self.bus.read(self.sp as u16);
                self.sp += 1;
            }
            Register16::SP => {
                let low = self.bus.read(self.sp as u16) as u16;
                self.sp += 1;
                let high = self.bus.read(self.sp as u16) as u16;
                self.sp += 1;
                self.sp = (high << 8) | low;
            }
            Register16::AF => {
                self.f = self.bus.read(self.sp as u16) & 0xF0;
                self.sp += 1;
                self.a = self.bus.read(self.sp as u16);
                self.sp += 1;
            }
        }
    }

    // Helper method to push a 16-bit value onto the stack
    fn push(&mut self, reg: Register16) {
        match reg {
            Register16::BC => {
                self.sp -= 1;
                self.bus.write(self.sp as u16, self.b);
                self.sp -= 1;
                self.bus.write(self.sp as u16, self.c);
            }
            Register16::DE => {
                self.sp -= 1;
                self.bus.write(self.sp as u16, self.d);
                self.sp -= 1;
                self.bus.write(self.sp as u16, self.e);
            }
            Register16::HL => {
                self.sp -= 1;
                self.bus.write(self.sp as u16, self.h);
                self.sp -= 1;
                self.bus.write(self.sp as u16, self.l);
            }
            Register16::SP => {
                let high = (self.sp >> 8) as u8;
                let low = (self.sp & 0xFF) as u8;
                self.sp -= 1;
                self.bus.write(self.sp as u16, high);
                self.sp -= 1;
                self.bus.write(self.sp as u16, low);
            }
            Register16::AF => {
                self.sp -= 1;
                self.bus.write(self.sp as u16, self.a);
                self.sp -= 1;
                self.bus.write(self.sp as u16, self.f & 0xF0);
            }
        }
    }

    // Helper method to push a raw 16-bit value onto the stack
    fn push_16bit_value(&mut self, value: u16) {
        self.sp = self.sp.wrapping_sub(1);
        self.bus.write(self.sp, (value >> 8) as u8);

        self.sp = self.sp.wrapping_sub(1);
        self.bus.write(self.sp, (value & 0xFF) as u8);
    }

    // Helper method to pop a raw 16-bit value from the stack
    fn pop_16bit_value(&mut self) -> u16 {
        let low = self.bus.read(self.sp as u16) as u16;
        self.sp += 1;
        let high = self.bus.read(self.sp as u16) as u16;
        self.sp += 1;
        (high << 8) | low
    }

    fn call(&mut self){
        let return_address = self.pc.wrapping_add(3);
        self.push_16bit_value(return_address);
        self.pc = self.get_16_bit_operand(self.pc);
    }

    fn call_address(&mut self, address: u16) {
        self.push_16bit_value(self.pc);
        self.pc = address;
    }

    fn add_16bit_to_sp_s8(&mut self, s8: i8) {
        let val_to_add = s8 as u16;
        let current_sp = self.sp;
        
        // Perform 16-bit addition
        let (result, overflow) = current_sp.overflowing_add(val_to_add);
        
        // Calculate half-carry: carry from bit 3 to 4 of the low byte
        let half_carry = ((current_sp & 0x0F) + (val_to_add & 0x0F)) > 0x0F;
        
        // Update the stack pointer
        self.sp = result;

        // Update flags
        // First, clear the top 4 bits (Z, N, H, C) 
        // This ensures Zero and Subtract flags are cleared.
        self.f &= !0xF0;
        
        // Set half-carry flag (bit 5)
        if half_carry {
            self.f |= 0x20;
        }
        
        // Set carry flag (bit 4)
        if overflow {
            self.f |= 0x10;
        }
    }

    fn update_hl_from_sp_s8(&mut self, s8: i8) {
        let val_to_add = s8 as u16;
        let current_sp = self.sp;
        
        // Perform 16-bit addition
        let (result, overflow) = current_sp.overflowing_add(val_to_add);
        
        // Calculate half-carry: carry from bit 3 to 4 of the low byte
        let half_carry = ((current_sp & 0x0F) + (val_to_add & 0x0F)) > 0x0F;
        
        // Update HL register
        self.assign_16bit_register(Register16::HL, result);

        // Clear all flags (Zero, Subtract, Half-Carry, Carry)
        self.f = 0;
        
        // Set half-carry flag (bit 5)
        if half_carry {
            self.f |= 0x20;
        }
        
        // Set carry flag (bit 4)
        if overflow {
            self.f |= 0x10;
        }
    }
}
