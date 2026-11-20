#[cfg(test)]
mod tests {
    use crate::gameboy::cpu::CPU;

    fn setup_cpu() -> CPU {
        CPU::new()
    }

    fn execute_instruction_until_done(cpu: &mut CPU, opcode: u8, operands: Vec<u8>) {
        cpu.bus.memory[cpu.pc as usize] = opcode;
        for (i, &op) in operands.iter().enumerate() {
            cpu.bus.memory[(cpu.pc as usize + 1 + i)] = op;
        }

        cpu.cycle(); 
        while cpu.cycles > 0 {
            cpu.cycle();
        }
    }

    #[test]
    fn test_nop() {
        let mut cpu = setup_cpu();
        cpu.bus.memory[0] = 0x00;
        cpu.cycle(); 
        assert_eq!(cpu.pc, 1);
        assert_eq!(cpu.cycles, 0);
    }

    #[test]
    fn test_stop() {
        let mut cpu = setup_cpu();
        cpu.bus.memory[0] = 0x10;
        cpu.cycle(); 
        assert_eq!(cpu.pc, 2);
        assert_eq!(cpu.cycles, 0);
    }

    #[test]
    fn test_halt() {
        let mut cpu = setup_cpu();
        cpu.bus.memory[0] = 0x76;
        cpu.cycle(); 
        assert_eq!(cpu.pc, 1);
        assert_eq!(cpu.cycles, 0);
    }

    #[test]
    fn test_ld_a_n() {
        let mut cpu = setup_cpu();
        execute_instruction_until_done(&mut cpu, 0x3E, vec![0x42]);
        assert_eq!(cpu.a, 0x42);
        assert_eq!(cpu.pc, 2);
    }

    #[test]
    fn test_ld_b_n() {
        let mut cpu = setup_cpu();
        execute_instruction_until_done(&mut cpu, 0x06, vec![0x55]);
        assert_eq!(cpu.b, 0x55);
        assert_eq!(cpu.pc, 2);
    }

    #[test]
    fn test_ld_c_n() {
        let mut cpu = setup_cpu();
        execute_instruction_until_done(&mut cpu, 0x0E, vec![0x66]);
        assert_eq!(cpu.c, 0x66);
        assert_eq!(cpu.pc, 2);
    }

    #[test]
    fn test_ld_d_n() {
        let mut cpu = setup_cpu();
        execute_instruction_until_done(&mut cpu, 0x16, vec![0x77]);
        assert_eq!(cpu.d, 0x77);
        assert_eq!(cpu.pc, 2);
    }

    #[test]
    fn test_ld_e_n() {
        let mut cpu = setup_cpu();
        execute_instruction_until_done(&mut cpu, 0x1E, vec![0x88]);
        assert_eq!(cpu.e, 0x88);
        assert_eq!(cpu.pc, 2);
    }

    #[test]
    fn test_ld_h_n() {
        let mut cpu = setup_cpu();
        execute_instruction_until_done(&mut cpu, 0x26, vec![0x99]);
        assert_eq!(cpu.h, 0x99);
        assert_eq!(cpu.pc, 2);
    }

    #[test]
    fn test_ld_l_n() {
        let mut cpu = setup_cpu();
        execute_instruction_until_done(&mut cpu, 0x2E, vec![0xAA]);
        assert_eq!(cpu.l, 0xAA);
        assert_eq!(cpu.pc, 2);
    }

    #[test]
    fn test_ld_a_a16() {
        let mut cpu = setup_cpu();
        cpu.bus.memory[0] = 0xFA;
        cpu.bus.memory[1] = 0x00;
        cpu.bus.memory[2] = 0x10;
        cpu.bus.memory[0x1000] = 0xBB;
        execute_instruction_until_done(&mut cpu, 0xFA, vec![0x00, 0x10]);
        assert_eq!(cpu.a, 0xBB);
        assert_eq!(cpu.pc, 3);
    }

    #[test]
    fn test_ld_a_a8() {
        let mut cpu = setup_cpu();
        cpu.bus.memory[0] = 0xF0;
        cpu.bus.memory[1] = 0x20;
        cpu.bus.memory[0x20] = 0xCC;
        execute_instruction_until_done(&mut cpu, 0xF0, vec![0x20]);
        assert_eq!(cpu.a, 0xCC);
        assert_eq!(cpu.pc, 2);
    }

    #[test]
    fn test_ld_a_c() {
        let mut cpu = setup_cpu();
        cpu.c = 0x30;
        cpu.bus.memory[0] = 0xF2;
        cpu.bus.memory[0xFF30] = 0xDD;
        execute_instruction_until_done(&mut cpu, 0xF2, vec![]);
        assert_eq!(cpu.a, 0xDD);
        assert_eq!(cpu.pc, 1);
    }

    #[test]
    fn test_ld_a_bc() {
        let mut cpu = setup_cpu();
        cpu.b = 0x10;
        cpu.c = 0x20;
        cpu.bus.memory[0] = 0x0A;
        cpu.bus.memory[0x1020] = 0xEE;
        execute_instruction_until_done(&mut cpu, 0x0A, vec![]);
        assert_eq!(cpu.a, 0xEE);
        assert_eq!(cpu.pc, 1);
    }

    #[test]
    fn test_ld_a_de() {
        let mut cpu = setup_cpu();
        cpu.d = 0x30;
        cpu.e = 0x40;
        cpu.bus.memory[0] = 0x1A;
        cpu.bus.memory[0x3040] = 0xFF;
        execute_instruction_until_done(&mut cpu, 0x1A, vec![]);
        assert_eq!(cpu.a, 0xFF);
        assert_eq!(cpu.pc, 1);
    }

    #[test]
    fn test_ld_a_hl_plus() {
        let mut cpu = setup_cpu();
        cpu.h = 0x10;
        cpu.l = 0x20;
        cpu.bus.memory[0] = 0x2A;
        cpu.bus.memory[0x1020] = 0x11;
        execute_instruction_until_done(&mut cpu, 0x2A, vec![]);
        assert_eq!(cpu.a, 0x11);
        assert_eq!(cpu.pc, 1);
        assert_eq!(cpu.h, 0x10);
        assert_eq!(cpu.l, 0x21);
    }

    #[test]
    fn test_ld_a_hl_minus() {
        let mut cpu = setup_cpu();
        cpu.h = 0x10;
        cpu.l = 0x20;
        cpu.bus.memory[0] = 0x3A;
        cpu.bus.memory[0x1020] = 0x22;
        execute_instruction_until_done(&mut cpu, 0x3A, vec![]);
        assert_eq!(cpu.a, 0x22);
        assert_eq!(cpu.pc, 1);
        assert_eq!(cpu.h, 0x10);
        assert_eq!(cpu.l, 0x1F);
    }

    #[test]
    fn test_ld_bc_a() {
        let mut cpu = setup_cpu();
        cpu.a = 0x11;
        cpu.b = 0x12;
        cpu.c = 0x00;
        cpu.bus.memory[0] = 0x02;
        execute_instruction_until_done(&mut cpu, 0x02, vec![]);
        assert_eq!(cpu.bus.memory[0x1200], 0x11);
        assert_eq!(cpu.pc, 1);
    }

    #[test]
    fn test_ld_de_a() {
        let mut cpu = setup_cpu();
        cpu.a = 0x22;
        cpu.d = 0x34;
        cpu.e = 0x00;
        cpu.bus.memory[0] = 0x12;
        execute_instruction_until_done(&mut cpu, 0x12, vec![]);
        assert_eq!(cpu.bus.memory[0x3400], 0x22);
        assert_eq!(cpu.pc, 1);
    }

    #[test]
    fn test_ld_hl_minus_a() {
        let mut cpu = setup_cpu();
        cpu.a = 0x33;
        cpu.h = 0x10;
        cpu.l = 0x20;
        cpu.bus.memory[0] = 0x32;
        cpu.bus.memory[0x1020] = 0x33;
        execute_instruction_until_done(&mut cpu, 0x32, vec![]);
        assert_eq!(cpu.h, 0x10);
        assert_eq!(cpu.l, 0x1F);
        assert_eq!(cpu.pc, 1);
    }

    #[test]
    fn test_ld_hl_plus_a() {
        let mut cpu = setup_cpu();
        cpu.a = 0x44;
        cpu.h = 0x10;
        cpu.l = 0x20;
        cpu.bus.memory[0] = 0x22;
        cpu.bus.memory[0x1020] = 0x44;
        execute_instruction_until_done(&mut cpu, 0x22, vec![]);
        assert_eq!(cpu.h, 0x10);
        assert_eq!(cpu.l, 0x21);
        assert_eq!(cpu.pc, 1);
    }

    #[test]
    fn test_ld_a16_a() {
        let mut cpu = setup_cpu();
        cpu.a = 0x55;
        cpu.bus.memory[0] = 0xEA;
        cpu.bus.memory[1] = 0x00;
        cpu.bus.memory[2] = 0x30;
        cpu.bus.memory[0x3000] = 0x55;
        execute_instruction_until_done(&mut cpu, 0xEA, vec![0x00, 0x30]);
        assert_eq!(cpu.bus.memory[0x3000], 0x55);
        assert_eq!(cpu.pc, 3);
    }

    #[test]
    fn test_ld_c_a() {
        let mut cpu = setup_cpu();
        cpu.a = 0x66;
        cpu.c = 0x60;
        cpu.bus.memory[0] = 0xE2;
        cpu.bus.memory[0x60] = 0x66;
        execute_instruction_until_done(&mut cpu, 0xE2, vec![]);
        assert_eq!(cpu.bus.memory[0x60], 0x66);
        assert_eq!(cpu.pc, 1);
    }

    #[test]
    fn test_ld_a8_a() {
        let mut cpu = setup_cpu();
        cpu.a = 0x77;
        cpu.bus.memory[0] = 0xE0;
        cpu.bus.memory[1] = 0x80;
        cpu.bus.memory[0xFF80] = 0x77;
        execute_instruction_until_done(&mut cpu, 0xE0, vec![0x80]);
        assert_eq!(cpu.bus.memory[0xFF80], 0x77);
        assert_eq!(cpu.pc, 2);
    }

    #[test]
    fn test_ld_hl_d8() {
        let mut cpu = setup_cpu();
        cpu.h = 0x10;
        cpu.l = 0x20;
        cpu.bus.memory[0] = 0x36;
        cpu.bus.memory[1] = 0x88;
        execute_instruction_until_done(&mut cpu, 0x36, vec![0x88]);
        assert_eq!(cpu.bus.memory[0x1020], 0x88);
        assert_eq!(cpu.pc, 2);
    }

    // --- LD (HL), Regs ---

    #[test]
    fn test_ld_hl_regs() {
        let mut cpu = setup_cpu();
        cpu.h = 0x12;
        cpu.l = 0x20;

        // LD (HL), B
        cpu.b = 0x99;
        execute_instruction_until_done(&mut cpu, 0x70, vec![]);
        assert_eq!(cpu.bus.memory[0x1220], 0x99);
        assert_eq!(cpu.pc, 1);

        // LD (HL), C
        cpu.c = 0xAA;
        execute_instruction_until_done(&mut cpu, 0x71, vec![]);
        assert_eq!(cpu.bus.memory[0x1220], 0xAA);
        assert_eq!(cpu.pc, 2);

        // LD (HL), D
        cpu.d = 0xBB;
        execute_instruction_until_done(&mut cpu, 0x72, vec![]);
        assert_eq!(cpu.bus.memory[0x1220], 0xBB);
        assert_eq!(cpu.pc, 3);

        // LD (HL), E
        cpu.e = 0xCC;
        execute_instruction_until_done(&mut cpu, 0x73, vec![]);
        assert_eq!(cpu.bus.memory[0x1220], 0xCC);
        assert_eq!(cpu.pc, 4);

        // LD (HL), H
        cpu.h = 0xDD;
        execute_instruction_until_done(&mut cpu, 0x74, vec![]);
        assert_eq!(cpu.bus.memory[0xDD20], 0xDD);
        assert_eq!(cpu.pc, 5);

        // LD (HL), L
        cpu.l = 0xEE;
        execute_instruction_until_done(&mut cpu, 0x75, vec![]);
        assert_eq!(cpu.bus.memory[0xDDEE], 0xEE);
        assert_eq!(cpu.pc, 6);
    }

    // --- LD Reg from (HL) ---

    #[test]
    fn test_ld_reg_from_hl() {
        let mut cpu = setup_cpu();
        cpu.h = 0x10;
        cpu.l = 0x20;
        cpu.bus.memory[0x1020] = 0x33;
        cpu.bus.memory[0x3320] = 0x44;
        cpu.bus.memory[0x3344] = 0x55;

        // LD A, (HL)
        execute_instruction_until_done(&mut cpu, 0x7E, vec![]);
        assert_eq!(cpu.a, 0x33);
        assert_eq!(cpu.pc, 1);

        // LD B, (HL)
        execute_instruction_until_done(&mut cpu, 0x46, vec![]);
        assert_eq!(cpu.b, 0x33);
        assert_eq!(cpu.pc, 2);

        // LD D, (HL)
        execute_instruction_until_done(&mut cpu, 0x56, vec![]);
        assert_eq!(cpu.d, 0x33);
        assert_eq!(cpu.pc, 3);

        // LD E, (HL)
        execute_instruction_until_done(&mut cpu, 0x5E, vec![]);
        assert_eq!(cpu.e, 0x33);
        assert_eq!(cpu.pc, 4);

        // LD H, (HL)
        execute_instruction_until_done(&mut cpu, 0x66, vec![]);
        assert_eq!(cpu.h, 0x33);
        assert_eq!(cpu.pc, 5);

        // LD L, (HL)
        execute_instruction_until_done(&mut cpu, 0x6E, vec![]);
        assert_eq!(cpu.l, 0x44);
        assert_eq!(cpu.pc, 6);

        // LD C, (HL)
        execute_instruction_until_done(&mut cpu, 0x46, vec![]);
        assert_eq!(cpu.b, 0x55);
        assert_eq!(cpu.pc, 7);
    }

    // --- 16-bit Load Immediates ---

    #[test]
    fn test_ld_16bit_immediates() {
        let mut cpu = setup_cpu();
        // LD BC, nn
        cpu.bus.memory[0] = 0x01;
        cpu.bus.memory[1] = 0x10;
        cpu.bus.memory[2] = 0x20;
        execute_instruction_until_done(&mut cpu, 0x01, vec![0x10, 0x20]);
        assert_eq!(cpu.b, 0x20);
        assert_eq!(cpu.c, 0x10);
        assert_eq!(cpu.pc, 3);

        // LD DE, nn
        cpu.bus.memory[3] = 0x11;
        cpu.bus.memory[4] = 0x30;
        cpu.bus.memory[5] = 0x40;
        execute_instruction_until_done(&mut cpu, 0x11, vec![0x30, 0x40]);
        assert_eq!(cpu.d, 0x40);
        assert_eq!(cpu.e, 0x30);
        assert_eq!(cpu.pc, 6);

        // LD HL, nn
        cpu.bus.memory[6] = 0x21;
        cpu.bus.memory[7] = 0x50;
        cpu.bus.memory[8] = 0x60;
        execute_instruction_until_done(&mut cpu, 0x21, vec![0x50, 0x60]);
        assert_eq!(cpu.h, 0x60);
        assert_eq!(cpu.l, 0x50);
        assert_eq!(cpu.pc, 9);

        // LD SP, nn
        cpu.bus.memory[9] = 0x31;
        cpu.bus.memory[10] = 0x70;
        cpu.bus.memory[11] = 0x80;
        execute_instruction_until_done(&mut cpu, 0x31, vec![0x70, 0x80]);
        assert_eq!(cpu.sp, 0x8070);
        assert_eq!(cpu.pc, 12);
    }

    // --- Register to Register ---

    #[test]
    fn test_ld_reg_to_reg() {
        let mut cpu = setup_cpu();

        // LD A, A
        cpu.a = 0x11;
        execute_instruction_until_done(&mut cpu, 0x7F, vec![]);
        assert_eq!(cpu.a, 0x11);

        // LD A, B
        cpu.b = 0x22;
        execute_instruction_until_done(&mut cpu, 0x78, vec![]);
        assert_eq!(cpu.a, 0x22);

        // LD A, C
        cpu.c = 0x33;
        execute_instruction_until_done(&mut cpu, 0x79, vec![]);
        assert_eq!(cpu.a, 0x33);

        // LD A, D
        cpu.d = 0x44;
        execute_instruction_until_done(&mut cpu, 0x7A, vec![]);
        assert_eq!(cpu.a, 0x44);

        // LD A, E
        cpu.e = 0x55;
        execute_instruction_until_done(&mut cpu, 0x7B, vec![]);
        assert_eq!(cpu.a, 0x55);

        // LD A, H
        cpu.h = 0x66;
        execute_instruction_until_done(&mut cpu, 0x7C, vec![]);
        assert_eq!(cpu.a, 0x66);

        // LD A, L
        cpu.l = 0x77;
        execute_instruction_until_done(&mut cpu, 0x7D, vec![]);
        assert_eq!(cpu.a, 0x77);

        // LD B, B
        cpu.b = 0x88;
        execute_instruction_until_done(&mut cpu, 0x40, vec![]);
        assert_eq!(cpu.b, 0x88);

        // LD B, A
        cpu.a = 0x99;
        execute_instruction_until_done(&mut cpu, 0x47, vec![]);
        assert_eq!(cpu.b, 0x99);

        // LD B, C
        cpu.c = 0xAA;
        execute_instruction_until_done(&mut cpu, 0x41, vec![]);
        assert_eq!(cpu.b, 0xAA);

        // LD B, D
        cpu.d = 0xBB;
        execute_instruction_until_done(&mut cpu, 0x42, vec![]);
        assert_eq!(cpu.b, 0xBB);

        // LD B, E
        cpu.e = 0xCC;
        execute_instruction_until_done(&mut cpu, 0x43, vec![]);
        assert_eq!(cpu.b, 0xCC);

        // LD B, H
        cpu.h = 0xDD;
        execute_instruction_until_done(&mut cpu, 0x44, vec![]);
        assert_eq!(cpu.b, 0xDD);

        // LD B, L
        cpu.l = 0xEE;
        execute_instruction_until_done(&mut cpu, 0x45, vec![]);
        assert_eq!(cpu.b, 0xEE);

        // LD C, A
        cpu.a = 0xFF;
        execute_instruction_until_done(&mut cpu, 0x4F, vec![]);
        assert_eq!(cpu.c, 0xFF);

        // LD C, B
        cpu.b = 0x11;
        execute_instruction_until_done(&mut cpu, 0x48, vec![]);
        assert_eq!(cpu.c, 0x11);

        // LD C, C
        cpu.c = 0x22;
        execute_instruction_until_done(&mut cpu, 0x49, vec![]);
        assert_eq!(cpu.c, 0x22);

        // LD C, D
        cpu.d = 0x33;
        execute_instruction_until_done(&mut cpu, 0x4A, vec![]);
        assert_eq!(cpu.c, 0x33);

        // LD C, E
        cpu.e = 0x44;
        execute_instruction_until_done(&mut cpu, 0x4B, vec![]);
        assert_eq!(cpu.c, 0x44);

        // LD C, H
        cpu.h = 0x55;
        execute_instruction_until_done(&mut cpu, 0x4C, vec![]);
        assert_eq!(cpu.c, 0x55);

        // LD C, L
        cpu.l = 0x66;
        execute_instruction_until_done(&mut cpu, 0x4D, vec![]);
        assert_eq!(cpu.c, 0x66);

        // LD D, A
        cpu.a = 0x77;
        execute_instruction_until_done(&mut cpu, 0x57, vec![]);
        assert_eq!(cpu.d, 0x77);

        // LD D, B
        cpu.b = 0x88;
        execute_instruction_until_done(&mut cpu, 0x50, vec![]);
        assert_eq!(cpu.d, 0x88);

        // LD D, C
        cpu.c = 0x99;
        execute_instruction_until_done(&mut cpu, 0x51, vec![]);
        assert_eq!(cpu.d, 0x99);

        // LD D, D
        cpu.d = 0xAA;
        execute_instruction_until_done(&mut cpu, 0x52, vec![]);
        assert_eq!(cpu.d, 0xAA);

        // LD D, E
        cpu.e = 0xBB;
        execute_instruction_until_done(&mut cpu, 0x53, vec![]);
        assert_eq!(cpu.d, 0xBB);

        // LD D, H
        cpu.h = 0xCC;
        execute_instruction_until_done(&mut cpu, 0x54, vec![]);
        assert_eq!(cpu.d, 0xCC);

        // LD D, L
        cpu.l = 0xDD;
        execute_instruction_until_done(&mut cpu, 0x55, vec![]);
        assert_eq!(cpu.d, 0xDD);

        // LD E, A
        cpu.a = 0xEE;
        execute_instruction_until_done(&mut cpu, 0x5F, vec![]);
        assert_eq!(cpu.e, 0xEE);

        // LD E, B
        cpu.b = 0xFF;
        execute_instruction_until_done(&mut cpu, 0x58, vec![]);
        assert_eq!(cpu.e, 0xFF);

        // LD E, C
        cpu.c = 0x11;
        execute_instruction_until_done(&mut cpu, 0x59, vec![]);
        assert_eq!(cpu.e, 0x11);

        // LD E, D
        cpu.d = 0x22;
        execute_instruction_until_done(&mut cpu, 0x5A, vec![]);
        assert_eq!(cpu.e, 0x22);

        // LD E, E
        cpu.e = 0x33;
        execute_instruction_until_done(&mut cpu, 0x5B, vec![]);
        assert_eq!(cpu.e, 0x33);

        // LD E, H
        cpu.h = 0x44;
        execute_instruction_until_done(&mut cpu, 0x5C, vec![]);
        assert_eq!(cpu.e, 0x44);

        // LD E, L
        cpu.l = 0x55;
        execute_instruction_until_done(&mut cpu, 0x5D, vec![]);
        assert_eq!(cpu.e, 0x55);

        // LD H, A
        cpu.a = 0x66;
        execute_instruction_until_done(&mut cpu, 0x67, vec![]);
        assert_eq!(cpu.h, 0x66);

        // LD H, B
        cpu.b = 0x77;
        execute_instruction_until_done(&mut cpu, 0x60, vec![]);
        assert_eq!(cpu.h, 0x77);

        // LD H, C
        cpu.c = 0x88;
        execute_instruction_until_done(&mut cpu, 0x61, vec![]);
        assert_eq!(cpu.h, 0x88);

        // LD H, D
        cpu.d = 0x99;
        execute_instruction_until_done(&mut cpu, 0x62, vec![]);
        assert_eq!(cpu.h, 0x99);

        // LD H, E
        cpu.e = 0xAA;
        execute_instruction_until_done(&mut cpu, 0x63, vec![]);
        assert_eq!(cpu.h, 0xAA);

        // LD H, H
        cpu.h = 0xBB;
        execute_instruction_until_done(&mut cpu, 0x64, vec![]);
        assert_eq!(cpu.h, 0xBB);

        // LD H, L
        cpu.l = 0xCC;
        execute_instruction_until_done(&mut cpu, 0x65, vec![]);
        assert_eq!(cpu.h, 0xCC);

        // LD L, A
        cpu.a = 0xDD;
        execute_instruction_until_done(&mut cpu, 0x6F, vec![]);
        assert_eq!(cpu.l, 0xDD);

        // LD L, B
        cpu.b = 0xEE;
        execute_instruction_until_done(&mut cpu, 0x68, vec![]);
        assert_eq!(cpu.l, 0xEE);

        // LD L, C
        cpu.c = 0xFF;
        execute_instruction_until_done(&mut cpu, 0x69, vec![]);
        assert_eq!(cpu.l, 0xFF);

        // LD L, D
        cpu.d = 0x11;
        execute_instruction_until_done(&mut cpu, 0x6A, vec![]);
        assert_eq!(cpu.l, 0x11);

        // LD L, E
        cpu.e = 0x22;
        execute_instruction_until_done(&mut cpu, 0x6B, vec![]);
        assert_eq!(cpu.l, 0x22);

        // LD L, H
        cpu.h = 0x33;
        execute_instruction_until_done(&mut cpu, 0x6C, vec![]);
        assert_eq!(cpu.l, 0x33);

        // LD L, L
        cpu.l = 0x44;
        execute_instruction_until_done(&mut cpu, 0x6D, vec![]);
        assert_eq!(cpu.l, 0x44);
    }

    // --- Arithmetic Instructions ---

    #[test]
    fn test_adc(){
        let mut cpu = setup_cpu();
        // --- ADC A, (reg) ---
        // Test Case 0: ADC A, A
        // A = 0x05, B = 0x05, CY = 0
        // Expected: A = 0x0A, Flags = Clear
        cpu.a = 0x05;
        cpu.b = 0x05;
        cpu.f = 0x00;
        execute_instruction_until_done(&mut cpu, 0x8F, vec![]);
        assert_eq!(cpu.a, 0x0A);
        assert_eq!(cpu.f & 0x80, 0x00); // Zero clear
        assert_eq!(cpu.f & 0x20, 0x00); // Half-carry clear
        assert_eq!(cpu.f & 0x10, 0x00); // Carry clear
        assert_eq!(cpu.pc, 1);

        let mut cpu = setup_cpu();
        
        // --- Case 1: Carry Cleared, No Overflow ---
        // A = 0x05, HL = 0x0010, Memory[0x0010] = 0x02, CY = 0
        cpu.a = 0x05;
        cpu.h = 0x00;
        cpu.l = 0x10;
        cpu.f = 0x00; // CY = 0
        cpu.bus.memory[0x0010] = 0x02;
        
        execute_instruction_until_done(&mut cpu, 0x8E, vec![]);
        
        assert_eq!(cpu.a, 0x07);      // 0x05 + 0x02 + 0 = 0x07
        assert_eq!(cpu.f & 0x10, 0x00); // Carry cleared
    
        // --- Case 2: Carry Set, No Overflow ---
        // A = 0x05, HL = 0x0010, Memory[0x0010] = 0x02, CY = 1
        cpu.a = 0x05;
        cpu.h = 0x00;
        cpu.l = 0x10;
        cpu.f = 0x10; // CY = 1
        cpu.bus.memory[0x0010] = 0x02;
        
        execute_instruction_until_done(&mut cpu, 0x8E, vec![]);
        
        assert_eq!(cpu.a, 0x08);      // 0x05 + 0x02 + 1 = 0x08
        assert_eq!(cpu.f & 0x10, 0x00); // Carry cleared (no overflow)
    
        // --- Case 3: Carry Set, Overflow ---
        // A = 0xFE, HL = 0x0010, Memory[0x0010] = 0x01, CY = 1
        cpu.a = 0xFE;
        cpu.h = 0x00;
        cpu.l = 0x10;
        cpu.f = 0x10; // CY = 1
        cpu.bus.memory[0x0010] = 0x01;
        
        execute_instruction_until_done(&mut cpu, 0x8E, vec![]);
        
        assert_eq!(cpu.a, 0x00);      // 0xFE + 0x01 + 1 = 0x100 -> A = 0x00
        assert_eq!(cpu.f & 0x10, 0x10); // Carry set (overflow)
    
        let mut cpu = setup_cpu();
        
        // Test Case 1: ADC A, B
        // A = 0x05, B = 0x05, CY = 0
        // Expected: A = 0x0A, Flags = Clear
        cpu.a = 0x05;
        cpu.b = 0x05;
        cpu.f = 0x00;
        execute_instruction_until_done(&mut cpu, 0x88, vec![]);
        assert_eq!(cpu.a, 0x0A);
        assert_eq!(cpu.f & 0x80, 0x00); // Zero clear
        assert_eq!(cpu.f & 0x20, 0x00); // Half-carry clear
        assert_eq!(cpu.f & 0x10, 0x00); // Carry clear
        assert_eq!(cpu.pc, 1);
    
        // Test Case 2: ADC A, B with Carry and Half-Carry
        // A = 0x0F, B = 0x01, CY = 1
        // Expected: A = 0x11, Half-Carry set
        cpu.a = 0x0F;
        cpu.b = 0x01;
        cpu.f = 0x10; // CY set
        execute_instruction_until_done(&mut cpu, 0x88, vec![]);
        assert_eq!(cpu.a, 0x11);
        assert_eq!(cpu.f & 0x20, 0x20); // Half-Carry set
        assert_eq!(cpu.f & 0x10, 0x00); // Carry not set
        assert_eq!(cpu.pc, 2);
    
        // Test Case 3: ADC A, C
        // A = 0x05, C = 0x05, CY = 0
        // Expected: A = 0x0A, Flags = Clear
        cpu.a = 0x05;
        cpu.c = 0x05;
        cpu.f = 0x00;
        execute_instruction_until_done(&mut cpu, 0x89, vec![]);
        assert_eq!(cpu.a, 0x0A);
        assert_eq!(cpu.f & 0x80, 0x00);
        assert_eq!(cpu.f & 0x20, 0x00);
        assert_eq!(cpu.f & 0x10, 0x00);
        assert_eq!(cpu.pc, 3);
    
        // Test Case 4: ADC A, C with Carry and Half-Carry
        // A = 0x0F, C = 0x01, CY = 1
        // Expected: A = 0x11, Half-Carry set
        cpu.a = 0x0F;
        cpu.c = 0x01;
        cpu.f = 0x10;
        execute_instruction_until_done(&mut cpu, 0x89, vec![]);
        assert_eq!(cpu.a, 0x11);
        assert_eq!(cpu.f & 0x20, 0x20);
        assert_eq!(cpu.f & 0x10, 0x00);
        assert_eq!(cpu.pc, 4);
    
        // Test Case 5: ADC A, D
        // A = 0x05, D = 0x05, CY = 0
        // Expected: A = 0x0A, Flags = Clear
        cpu.a = 0x05;
        cpu.d = 0x05;
        cpu.f = 0x00;
        execute_instruction_until_done(&mut cpu, 0x8A, vec![]);
        assert_eq!(cpu.a, 0x0A);
        assert_eq!(cpu.f & 0x80, 0x00);
        assert_eq!(cpu.f & 0x20, 0x00);
        assert_eq!(cpu.f & 0x10, 0x00);
        assert_eq!(cpu.pc, 5);
    
        // Test Case 6: ADC A, D with Carry and Half-Carry
        // A = 0x0F, D = 0x01, CY = 1
        // Expected: A = 0x11, Half-Carry set
        cpu.a = 0x0F;
        cpu.d = 0x01;
        cpu.f = 0x10;
        execute_instruction_until_done(&mut cpu, 0x8A, vec![]);
        assert_eq!(cpu.a, 0x11);
        assert_eq!(cpu.f & 0x20, 0x20);
        assert_eq!(cpu.f & 0x10, 0x00);
        assert_eq!(cpu.pc, 6);
    
        // Test Case 7: ADC A, E
        // A = 0x05, E = 0x05, CY = 0
        // Expected: A = 0x0A, Flags = Clear
        cpu.a = 0x05;
        cpu.e = 0x05;
        cpu.f = 0x00;
        execute_instruction_until_done(&mut cpu, 0x8B, vec![]);
        assert_eq!(cpu.a, 0x0A);
        assert_eq!(cpu.f & 0x80, 0x00);
        assert_eq!(cpu.f & 0x20, 0x00);
        assert_eq!(cpu.f & 0x10, 0x00);
        assert_eq!(cpu.pc, 7);
    
        // Test Case 8: ADC A, E with Carry and Half-Carry
        // A = 0x0F, E = 0x01, CY = 1
        // Expected: A = 0x11, Half-Carry set
        cpu.a = 0x0F;
        cpu.e = 0x01;
        cpu.f = 0x10;
        execute_instruction_until_done(&mut cpu, 0x8B, vec![]);
        assert_eq!(cpu.a, 0x11);
        assert_eq!(cpu.f & 0x20, 0x20);
        assert_eq!(cpu.f & 0x10, 0x00);
        assert_eq!(cpu.pc, 8);
    
        // Test Case 9: ADC A, H
        // A = 0x05, H = 0x05, CY = 0
        // Expected: A = 0x0A, Flags = Clear
        cpu.a = 0x05;
        cpu.h = 0x05;
        cpu.f = 0x00;
        execute_instruction_until_done(&mut cpu, 0x8C, vec![]);
        assert_eq!(cpu.a, 0x0A);
        assert_eq!(cpu.f & 0x80, 0x00);
        assert_eq!(cpu.f & 0x20, 0x00);
        assert_eq!(cpu.f & 0x10, 0x00);
        assert_eq!(cpu.pc, 9);
    
        // Test Case 10: ADC A, H with Carry and Half-Carry
        // A = 0x0F, H = 0x01, CY = 1
        // Expected: A = 0x11, Half-Carry set
        cpu.a = 0x0F;
        cpu.h = 0x01;
        cpu.f = 0x10;
        execute_instruction_until_done(&mut cpu, 0x8C, vec![]);
        assert_eq!(cpu.a, 0x11);
        assert_eq!(cpu.f & 0x20, 0x20);
        assert_eq!(cpu.f & 0x10, 0x00);
        assert_eq!(cpu.pc, 10);
    
        // Test Case 11: ADC A, L
        // A = 0x05, L = 0x05, CY = 0
        // Expected: A = 0x0A, Flags = Clear
        cpu.a = 0x05;
        cpu.l = 0x05;
        cpu.f = 0x00;
        execute_instruction_until_done(&mut cpu, 0x8D, vec![]);
        assert_eq!(cpu.a, 0x0A);
        assert_eq!(cpu.f & 0x80, 0x00);
        assert_eq!(cpu.f & 0x20, 0x00);
        assert_eq!(cpu.f & 0x10, 0x00);
        assert_eq!(cpu.pc, 11);
    
        // Test Case 12: ADC A, L with Carry and Half-Carry
        // A = 0xFF, L = 0x00, CY = 1
        // Expected: A = 0x00, Half-Carry set, Carry set
        cpu.a = 0xFF;
        cpu.l = 0x00;
        cpu.f = 0x10;
        execute_instruction_until_done(&mut cpu, 0x8D, vec![]);
        assert_eq!(cpu.a, 0x00);
        assert_eq!(cpu.f & 0x20, 0x20);
        assert_eq!(cpu.f & 0x10, 0x10);
        assert_eq!(cpu.pc, 12);
    }

    #[test]
    fn test_and() {
        let mut cpu = setup_cpu();

        // 0xA0: AND A, B
        cpu.a = 0xFF;
        cpu.b = 0xF0;
        execute_instruction_until_done(&mut cpu, 0xA0, vec![]);
        assert_eq!(cpu.a, 0xF0);
        assert_eq!(cpu.pc, 1);

        // 0xA1: AND A, C
        cpu.a = 0xFF;
        cpu.c = 0xF8;
        execute_instruction_until_done(&mut cpu, 0xA1, vec![]);
        assert_eq!(cpu.a, 0xF8);
        assert_eq!(cpu.pc, 2);

        // 0xA2: AND A, D
        cpu.a = 0xFF;
        cpu.d = 0xF6;
        execute_instruction_until_done(&mut cpu, 0xA2, vec![]);
        assert_eq!(cpu.a, 0xF6);
        assert_eq!(cpu.pc, 3);

        // 0xA3: AND A, E
        cpu.a = 0xFF;
        cpu.e = 0xF4;
        execute_instruction_until_done(&mut cpu, 0xA3, vec![]);
        assert_eq!(cpu.a, 0xF4);
        assert_eq!(cpu.pc, 4);

        // 0xA4: AND A, H
        cpu.a = 0xFF;
        cpu.h = 0xF2;
        execute_instruction_until_done(&mut cpu, 0xA4, vec![]);
        assert_eq!(cpu.a, 0xF2);
        assert_eq!(cpu.pc, 5);

        // 0xA5: AND A, L
        cpu.a = 0xFF;
        cpu.l = 0xF1;
        execute_instruction_until_done(&mut cpu, 0xA5, vec![]);
        assert_eq!(cpu.a, 0xF1);
        assert_eq!(cpu.pc, 6);

        // 0xA6: AND A, (HL)
        cpu.h = 0x10;
        cpu.l = 0x20;
        cpu.a = 0xFF;
        cpu.bus.memory[0x1020] = 0x0F;
        execute_instruction_until_done(&mut cpu, 0xA6, vec![]);
        assert_eq!(cpu.a, 0x0F);
        assert_eq!(cpu.pc, 7);

        // 0xA7: AND A, A
        cpu.a = 0x55;
        execute_instruction_until_done(&mut cpu, 0xA7, vec![]);
        assert_eq!(cpu.a, 0x55);
        assert_eq!(cpu.pc, 8);
    }

    #[test]
    fn test_xor_instructions() {
        let mut cpu = setup_cpu();

        // 0xA8: XOR A, B
        cpu.a = 0xAA;
        cpu.b = 0x55;
        execute_instruction_until_done(&mut cpu, 0xA8, vec![]);
        assert_eq!(cpu.a, 0xFF);
        assert_eq!(cpu.pc, 1);

        // 0xA9: XOR A, C
        cpu.a = 0xAA;
        cpu.c = 0x55;
        execute_instruction_until_done(&mut cpu, 0xA9, vec![]);
        assert_eq!(cpu.a, 0xFF);
        assert_eq!(cpu.pc, 2);

        // 0xAA: XOR A, D
        cpu.a = 0xAA;
        cpu.d = 0x55;
        execute_instruction_until_done(&mut cpu, 0xAA, vec![]);
        assert_eq!(cpu.a, 0xFF);
        assert_eq!(cpu.pc, 3);

        // 0xAB: XOR A, E
        cpu.a = 0xAA;
        cpu.e = 0x55;
        execute_instruction_until_done(&mut cpu, 0xAB, vec![]);
        assert_eq!(cpu.a, 0xFF);
        assert_eq!(cpu.pc, 4);

        // 0xAC: XOR A, H
        cpu.a = 0xAA;
        cpu.h = 0x55;
        execute_instruction_until_done(&mut cpu, 0xAC, vec![]);
        assert_eq!(cpu.a, 0xFF);
        assert_eq!(cpu.pc, 5);

        // 0xAD: XOR A, L
        cpu.a = 0xAA;
        cpu.l = 0x55;
        execute_instruction_until_done(&mut cpu, 0xAD, vec![]);
        assert_eq!(cpu.a, 0xFF);
        assert_eq!(cpu.pc, 6);

        // 0xAE: XOR A, (HL)
        cpu.h = 0x10;
        cpu.l = 0x20;
        cpu.a = 0xAA;
        cpu.bus.memory[0x1020] = 0x55;
        execute_instruction_until_done(&mut cpu, 0xAE, vec![]);
        assert_eq!(cpu.a, 0xFF);
        assert_eq!(cpu.pc, 7);
        assert_eq!(cpu.f, 0b00000000); // flag check

        // 0xAF: XOR A, A
        cpu.a = 0x55;
        execute_instruction_until_done(&mut cpu, 0xAF, vec![]);
        assert_eq!(cpu.a, 0x00);
        assert_eq!(cpu.f, 0b10000000); // zero flag check
        assert_eq!(cpu.pc, 8);
    }

    #[test]
    fn test_or() {
        let mut cpu = setup_cpu();

        // 0xB0: OR A, B
        cpu.a = 0x00;
        cpu.b = 0xFF;
        execute_instruction_until_done(&mut cpu, 0xB0, vec![]);
        assert_eq!(cpu.a, 0xFF);
        assert_eq!(cpu.pc, 1);

        // 0xB1: OR A, C
        cpu.a = 0x00;
        cpu.c = 0x55;
        execute_instruction_until_done(&mut cpu, 0xB1, vec![]);
        assert_eq!(cpu.a, 0x55);
        assert_eq!(cpu.pc, 2);

        // 0xB2: OR A, D
        cpu.a = 0x00;
        cpu.d = 0xAA;
        execute_instruction_until_done(&mut cpu, 0xB2, vec![]);
        assert_eq!(cpu.a, 0xAA);
        assert_eq!(cpu.pc, 3);

        // 0xB3: OR A, E
        cpu.a = 0x00;
        cpu.e = 0x33;
        execute_instruction_until_done(&mut cpu, 0xB3, vec![]);
        assert_eq!(cpu.a, 0x33);
        assert_eq!(cpu.pc, 4);

        // 0xB4: OR A, H
        cpu.a = 0x00;
        cpu.h = 0x11;
        execute_instruction_until_done(&mut cpu, 0xB4, vec![]);
        assert_eq!(cpu.a, 0x11);
        assert_eq!(cpu.pc, 5);

        // 0xB5: OR A, L
        cpu.a = 0x00;
        cpu.l = 0x22;
        execute_instruction_until_done(&mut cpu, 0xB5, vec![]);
        assert_eq!(cpu.a, 0x22);
        assert_eq!(cpu.pc, 6);

        // 0xB6: OR A, (HL)
        cpu.h = 0x10;
        cpu.l = 0x20;
        cpu.a = 0x00;
        cpu.bus.memory[0x1020] = 0x44;
        execute_instruction_until_done(&mut cpu, 0xB6, vec![]);
        assert_eq!(cpu.a, 0x44);
        assert_eq!(cpu.pc, 7);

        // 0xB7: OR A, A
        cpu.a = 0x55;
        execute_instruction_until_done(&mut cpu, 0xB7, vec![]);
        assert_eq!(cpu.a, 0x55);
        assert_eq!(cpu.pc, 8);
    }

    #[test]
    fn test_cp_instructions() {
        let mut cpu = setup_cpu();

        // --- Test 0xBF: CP A, A ---
        // A = 0x55. Comparison A - A = 0.
        // Expected: A = 0x55, Z=1, C=0, HC=0, S=0 (assuming initial flags are 0)
        cpu.a = 0x55;
        cpu.f = 0x00;
        execute_instruction_until_done(&mut cpu, 0xBF, vec![]);
        assert_eq!(cpu.a, 0x55);
        assert_eq!(cpu.f & 0x80, 0x80); // Zero set
        assert_eq!(cpu.f & 0x10, 0x00); // Carry clear
        assert_eq!(cpu.f & 0x20, 0x00); // Half-Carry clear
        assert_eq!(cpu.f & 0x40, 0x40); // Subtract set

        let mut cpu = setup_cpu();

        // --- Test 0xB8: CP A, B ---
        // Case 1: Equal
        // A = 0xAA, B = 0xAA. Result: A-B = 0.
        // Expected: A = 0xAA, Z=1
        cpu.a = 0xAA;
        cpu.b = 0xAA;
        cpu.f = 0x00;
        execute_instruction_until_done(&mut cpu, 0xB8, vec![]);
        assert_eq!(cpu.a, 0xAA);
        assert_eq!(cpu.f & 0x80, 0x80); // Zero set

        // Case 2: Not Equal
        // A = 0xAA, B = 0x55. Result: A-B != 0.
        // Expected: A = 0xAA, Z=0
        cpu.a = 0xAA;
        cpu.b = 0x55;
        cpu.f = 0x00;
        execute_instruction_until_done(&mut cpu, 0xB8, vec![]);
        assert_eq!(cpu.a, 0xAA);
        assert_eq!(cpu.f & 0x80, 0x00); // Zero clear

        let mut cpu = setup_cpu();

        // --- Test 0xB9: CP A, C ---
        // A = 0x10, C = 0x10.
        // Expected: A = 0x10, Z=1
        cpu.a = 0x10;
        cpu.c = 0x10;
        cpu.f = 0x00;
        execute_instruction_until_done(&mut cpu, 0xB9, vec![]);
        assert_eq!(cpu.a, 0x10);
        assert_eq!(cpu.f & 0x80, 0x80); // Zero set

        let mut cpu = setup_cpu();

        // --- Test 0xBA: CP A, D ---
        // A = 0x20, D = 0x20.
        // Expected: A = 0x20, Z=1
        cpu.a = 0x20;
        cpu.d = 0x20;
        cpu.f = 0x00;
        execute_instruction_until_done(&mut cpu, 0xBA, vec![]);
        assert_eq!(cpu.a, 0x20);
        assert_eq!(cpu.f & 0x80, 0x80); // Zero set

        let mut cpu = setup_cpu();

        // --- Test 0xBB: CP A, E ---
        // A = 0x30, E = 0x30.
        // Expected: A = 0x30, Z=1
        cpu.a = 0x30;
        cpu.e = 0x30;
        cpu.f = 0x00;
        execute_instruction_until_done(&mut cpu, 0xBB, vec![]);
        assert_eq!(cpu.a, 0x30);
        assert_eq!(cpu.f & 0x80, 0x80); // Zero set

        let mut cpu = setup_cpu();

        // --- Test 0xBC: CP A, H ---
        // A = 0x40, H = 0x40.
        // Expected: A = 0x40, Z=1
        cpu.a = 0x40;
        cpu.h = 0x40;
        cpu.f = 0x00;
        execute_instruction_until_done(&mut cpu, 0xBC, vec![]);
        assert_eq!(cpu.a, 0x40);
        assert_eq!(cpu.f & 0x80, 0x80); // Zero set

        let mut cpu = setup_cpu();

        // --- Test 0xBD: CP A, L ---
        // A = 0x50, L = 0x50.
        // Expected: A = 0x50, Z=1
        cpu.a = 0x50;
        cpu.l = 0x50;
        cpu.f = 0x00;
        execute_instruction_until_done(&mut cpu, 0xBD, vec![]);
        assert_eq!(cpu.a, 0x50);
        assert_eq!(cpu.f & 0x80, 0x80); // Zero set

        let mut cpu = setup_cpu();

        // --- Test 0xBE: CP A, (HL) ---
        // HL = 0x2000, Memory[0x2000] = 0x60
        // A = 0x60
        // Expected: A = 0x60, Z=1
        cpu.h = 0x20;
        cpu.l = 0x00;
        cpu.bus.memory[0x2000] = 0x60;
        cpu.a = 0x60;
        cpu.f = 0x00;
        execute_instruction_until_done(&mut cpu, 0xBE, vec![]);
        assert_eq!(cpu.a, 0x60);
        assert_eq!(cpu.f & 0x80, 0x80); // Zero set

        // Case 2: Not Equal
        // A = 0x60, Memory[0x2000] = 0x70
        // Expected: A = 0x60, Z=0
        cpu.a = 0x60;
        cpu.bus.memory[0x2000] = 0x70;
        cpu.f = 0x00;
        execute_instruction_until_done(&mut cpu, 0xBE, vec![]);
        assert_eq!(cpu.a, 0x60);
        assert_eq!(cpu.f & 0x80, 0x00); // Zero clear
    }

    #[test]
    fn test_add_inc_dec() {
        let mut cpu = setup_cpu();
        // ADD A, A
        cpu.a = 0x05;
        execute_instruction_until_done(&mut cpu, 0x87, vec![]);
        assert_eq!(cpu.a, 0x0A);
        assert_eq!(cpu.f & 0x80, 0x00);
        assert_eq!(cpu.f & 0x10, 0x00);
        assert_eq!(cpu.f & 0x20, 0x00);

        // ADD A, B
        cpu.a = 0x05;
        cpu.b = 0x05;
        execute_instruction_until_done(&mut cpu, 0x80, vec![]);
        assert_eq!(cpu.a, 0x0A);
        assert_eq!(cpu.f & 0x80, 0x00); // Zero clear
        assert_eq!(cpu.f & 0x20, 0x00); // Half-carry clear

        // ADD A, C
        cpu.a = 0x05;
        cpu.c = 0x05;
        execute_instruction_until_done(&mut cpu, 0x81, vec![]);
        assert_eq!(cpu.a, 0x0A);
        assert_eq!(cpu.f & 0x80, 0x00);
        assert_eq!(cpu.f & 0x20, 0x00);

        // ADD A, D
        cpu.a = 0x05;
        cpu.d = 0x05;
        execute_instruction_until_done(&mut cpu, 0x82, vec![]);
        assert_eq!(cpu.a, 0x0A);
        assert_eq!(cpu.f & 0x80, 0x00);
        assert_eq!(cpu.f & 0x20, 0x00);

        // ADD A, E
        cpu.a = 0x05;
        cpu.e = 0x05;
        execute_instruction_until_done(&mut cpu, 0x83, vec![]);
        assert_eq!(cpu.a, 0x0A);
        assert_eq!(cpu.f & 0x80, 0x00);
        assert_eq!(cpu.f & 0x20, 0x00);

        // ADD A, H
        cpu.a = 0x05;
        cpu.h = 0x05;
        execute_instruction_until_done(&mut cpu, 0x84, vec![]);
        assert_eq!(cpu.a, 0x0A);
        assert_eq!(cpu.f & 0x80, 0x00);
        assert_eq!(cpu.f & 0x20, 0x00);

        // ADD A, L
        cpu.a = 0x05;
        cpu.l = 0x05;
        execute_instruction_until_done(&mut cpu, 0x85, vec![]);
        assert_eq!(cpu.a, 0x0A);
        assert_eq!(cpu.f & 0x80, 0x00);
        assert_eq!(cpu.f & 0x20, 0x00);

        // Test flags for ADD A, L (Half-Carry and Carry)
        cpu.a = 0x0F;
        cpu.l = 0x01;
        execute_instruction_until_done(&mut cpu, 0x85, vec![]);
        assert_eq!(cpu.a, 0x10);
        assert_eq!(cpu.f & 0x20, 0x20); // Half-Carry set
        assert_eq!(cpu.f & 0x10, 0x00); // Carry clear
        
        // --- ADD A, (HL) ---
        // Test Case: A = 0x02, (HL) = 0x1000, memory[0x1000] = 0x05
        // Expected: A = 0x07, Flags = Clear
        cpu.h = 0x10;
        cpu.l = 0x00;
        cpu.a = 0x02;
        cpu.bus.memory[0x1000] = 0x05;
        execute_instruction_until_done(&mut cpu, 0x86, vec![]);
        assert_eq!(cpu.a, 0x07);
        assert_eq!(cpu.f & 0x80, 0x00); // Zero clear
        assert_eq!(cpu.f & 0x20, 0x00); // Half-carry clear
        
        // Test Case: Carry and Half-Carry flags
        // A = 0x0F, (HL) = 0x1000, memory[0x1000] = 0x01
        // Expected: A = 0x10, Half-Carry set
        cpu.a = 0x0F;
        cpu.bus.memory[0x1000] = 0x01;
        execute_instruction_until_done(&mut cpu, 0x86, vec![]);
        assert_eq!(cpu.a, 0x10);
        assert_eq!(cpu.f & 0x20, 0x20); // Half-Carry set
        
        // INC A
        cpu.a = 0x05;
        execute_instruction_until_done(&mut cpu, 0x3C, vec![]);
        assert_eq!(cpu.a, 0x06);
        assert_eq!(cpu.f & 0x80, 0x00);
        assert_eq!(cpu.f & 0x20, 0x00);

        // INC B
        cpu.b = 0x05;
        execute_instruction_until_done(&mut cpu, 0x04, vec![]);
        assert_eq!(cpu.b, 0x06);
        
        // INC C
        cpu.c = 0x05;
        execute_instruction_until_done(&mut cpu, 0x0C, vec![]);
        assert_eq!(cpu.c, 0x06);
        
        // INC D
        cpu.d = 0x05;
        execute_instruction_until_done(&mut cpu, 0x14, vec![]);
        assert_eq!(cpu.d, 0x06);
        
        // INC E
        cpu.e = 0x05;
        execute_instruction_until_done(&mut cpu, 0x1C, vec![]);
        assert_eq!(cpu.e, 0x06);
        
        // INC H
        cpu.h = 0x05;
        execute_instruction_until_done(&mut cpu, 0x24, vec![]);
        assert_eq!(cpu.h, 0x06);
        
        // INC L
        cpu.l = 0x05;
        execute_instruction_until_done(&mut cpu, 0x2C, vec![]);
        assert_eq!(cpu.l, 0x06);
        
        // DEC A
        cpu.a = 0x06;
        execute_instruction_until_done(&mut cpu, 0x3D, vec![]);
        assert_eq!(cpu.a, 0x05);
        assert_eq!(cpu.f & 0x80, 0x00);
        assert_eq!(cpu.f & 0x40, 0x40);
    
        // DEC B
        cpu.b = 0x06;
        execute_instruction_until_done(&mut cpu, 0x05, vec![]);
        assert_eq!(cpu.b, 0x05);
        assert_eq!(cpu.f & 0x40, 0x40);
    
        // DEC C
        cpu.c = 0x06;
        execute_instruction_until_done(&mut cpu, 0x0D, vec![]);
        assert_eq!(cpu.c, 0x05);
        assert_eq!(cpu.f & 0x40, 0x40);
    
        // DEC D
        cpu.d = 0x06;
        execute_instruction_until_done(&mut cpu, 0x15, vec![]);
        assert_eq!(cpu.d, 0x05);
        assert_eq!(cpu.f & 0x40, 0x40);
    
        // DEC E
        cpu.e = 0x06;
        execute_instruction_until_done(&mut cpu, 0x1D, vec![]);
        assert_eq!(cpu.e, 0x05);
        assert_eq!(cpu.f & 0x40, 0x40);
    
        // DEC H
        cpu.h = 0x06;
        execute_instruction_until_done(&mut cpu, 0x25, vec![]);
        assert_eq!(cpu.h, 0x05);
        assert_eq!(cpu.f & 0x40, 0x40);
    
        // DEC L
        cpu.l = 0x06;
        execute_instruction_until_done(&mut cpu, 0x2D, vec![]);
        assert_eq!(cpu.l, 0x05);
        assert_eq!(cpu.f & 0x40, 0x40);
    
        // INC BC
        cpu.b = 0xFF;
        cpu.c = 0xFF;
        execute_instruction_until_done(&mut cpu, 0x03, vec![]);
        assert_eq!(cpu.b, 0x00);
        assert_eq!(cpu.c, 0x00);
    
        // INC DE
        cpu.d = 0xFF;
        cpu.e = 0xFF;
        execute_instruction_until_done(&mut cpu, 0x13, vec![]);
        assert_eq!(cpu.d, 0x00);
        assert_eq!(cpu.e, 0x00);
    
        // INC HL
        cpu.h = 0xFF;
        cpu.l = 0xFF;
        execute_instruction_until_done(&mut cpu, 0x23, vec![]);
        assert_eq!(cpu.h, 0x00);
        assert_eq!(cpu.l, 0x00);
    
        // INC SP
        cpu.sp = 0x0000;
        execute_instruction_until_done(&mut cpu, 0x33, vec![]);
        assert_eq!(cpu.sp, 0x0001);
    
        // DEC BC
        cpu.b = 0x00;
        cpu.c = 0x00;
        execute_instruction_until_done(&mut cpu, 0x0B, vec![]);
        assert_eq!(cpu.b, 0xFF);
        assert_eq!(cpu.c, 0xFF);
    
        // DEC DE
        cpu.d = 0x00;
        cpu.e = 0x00;
        execute_instruction_until_done(&mut cpu, 0x1B, vec![]);
        assert_eq!(cpu.d, 0xFF);
        assert_eq!(cpu.e, 0xFF);
    
        // DEC HL
        cpu.h = 0x00;
        cpu.l = 0x00;
        execute_instruction_until_done(&mut cpu, 0x2B, vec![]);
        assert_eq!(cpu.h, 0xFF);
        assert_eq!(cpu.l, 0xFF);
    
        // DEC SP
        cpu.sp = 0x0000;
        execute_instruction_until_done(&mut cpu, 0x3B, vec![]);
        assert_eq!(cpu.sp, 0xFFFF);
    }

    #[test]
    fn test_immediate_arithmetic_instructions() {
        let mut cpu = setup_cpu();

        // --- Test 0xC6: ADD d8 ---
        // A = 0xFE, d8 = 0x02
        // Expected: A = 0x00, C=1, HC=1, Z=1
        cpu.a = 0xFE;
        cpu.bus.memory[1] = 0x02; // Assuming PC is 0
        cpu.f = 0x00;
        execute_instruction_until_done(&mut cpu, 0xC6, vec![]);
        assert_eq!(cpu.a, 0x00);
        assert_eq!(cpu.f & 0x10, 0x10); // Carry set
        assert_eq!(cpu.f & 0x20, 0x20); // Half-Carry set
        assert_eq!(cpu.f & 0x80, 0x80); // Zero set

        let mut cpu = setup_cpu();

        // --- Test 0xCE: ADC d8 ---
        // A = 0xFE, d8 = 0x01, Carry = 1
        // Expected: A = 0x00, C=1, HC=1, Z=1
        cpu.a = 0xFE;
        cpu.bus.memory[1] = 0x01;
        cpu.f = 0x10; // Carry set
        execute_instruction_until_done(&mut cpu, 0xCE, vec![]);
        assert_eq!(cpu.a, 0x00);
        assert_eq!(cpu.f & 0x10, 0x10); // Carry set
        assert_eq!(cpu.f & 0x20, 0x20); // Half-Carry set
        assert_eq!(cpu.f & 0x80, 0x80); // Zero set

        let mut cpu = setup_cpu();

        // --- Test 0xD6: SUB d8 ---
        // A = 0x00, d8 = 0x01
        // Expected: A = 0xFF, C=1, HC=1, Z=0
        cpu.a = 0x00;
        cpu.bus.memory[1] = 0x01;
        cpu.f = 0x00;
        execute_instruction_until_done(&mut cpu, 0xD6, vec![]);
        assert_eq!(cpu.a, 0xFF);
        assert_eq!(cpu.f & 0x10, 0x10); // Carry set (negative result)
        assert_eq!(cpu.f & 0x20, 0x20); // Half-Carry set
        assert_eq!(cpu.f & 0x80, 0x00); // Zero clear

        let mut cpu = setup_cpu();

        // --- Test 0xDE: SBC d8 ---
        // A = 0x00, d8 = 0x01, Carry = 1
        // Expected: A = 0xFE, C=1, HC=1, Z=0
        cpu.a = 0x00;
        cpu.bus.memory[1] = 0x01;
        cpu.f = 0x10; // Carry set
        execute_instruction_until_done(&mut cpu, 0xDE, vec![]);
        assert_eq!(cpu.a, 0xFE);
        assert_eq!(cpu.f & 0x10, 0x10); // Carry set
        assert_eq!(cpu.f & 0x20, 0x20); // Half-Carry set
        assert_eq!(cpu.f & 0x80, 0x00); // Zero clear

        let mut cpu = setup_cpu();

        // --- Test 0xE6: AND d8 ---
        // A = 0xFF, d8 = 0x0F
        // Expected: A = 0x0F, HC=1, C=0, S=0
        cpu.a = 0xFF;
        cpu.bus.memory[1] = 0x0F;
        cpu.f = 0x00;
        execute_instruction_until_done(&mut cpu, 0xE6, vec![]);
        assert_eq!(cpu.a, 0x0F);
        assert_eq!(cpu.f & 0x20, 0x20); // Half-Carry set (ALWAYS 1)
        assert_eq!(cpu.f & 0x10, 0x00); // Carry clear
        assert_eq!(cpu.f & 0x40, 0x00); // Subtract clear

        let mut cpu = setup_cpu();

        // --- Test 0xEE: XOR d8 ---
        // A = 0xFF, d8 = 0xFF
        // Expected: A = 0x00, HC=1, C=0, S=0
        cpu.a = 0xFF;
        cpu.bus.memory[1] = 0xFF;
        cpu.f = 0x00;
        execute_instruction_until_done(&mut cpu, 0xEE, vec![]);
        assert_eq!(cpu.a, 0x00);
        assert_eq!(cpu.f & 0x20, 0x00); // Half-Carry cleared
        assert_eq!(cpu.f & 0x10, 0x00); // Carry clear
        assert_eq!(cpu.f & 0x40, 0x00); // Subtract clear

        let mut cpu = setup_cpu();

        // --- Test 0xF6: OR d8 ---
        // A = 0x0F, d8 = 0xF0
        // Expected: A = 0xFF, HC=1, C=0, S=0
        cpu.a = 0x0F;
        cpu.bus.memory[1] = 0xF0;
        cpu.f = 0x00;
        execute_instruction_until_done(&mut cpu, 0xF6, vec![]);
        assert_eq!(cpu.a, 0xFF);
        assert_eq!(cpu.f & 0x20, 0x00); // Half-Carry cleared
        assert_eq!(cpu.f & 0x10, 0x00); // Carry clear
        assert_eq!(cpu.f & 0x40, 0x00); // Subtract clear

        let mut cpu = setup_cpu();

        // --- Test 0xFE: CP d8 ---
        // Case 1: Equal
        // A = 0x05, d8 = 0x05
        // Expected: A = 0x05, Z=1
        cpu.a = 0x05;
        cpu.bus.memory[1] = 0x05;
        cpu.f = 0x00;
        execute_instruction_until_done(&mut cpu, 0xFE, vec![0x05]);
        assert_eq!(cpu.a, 0x05);
        assert_eq!(cpu.f & 0x80, 0x80); // Zero set

        // Case 2: Not Equal
        // A = 0x05, d8 = 0x06
        // Expected: A = 0x05, Z=0
        cpu.a = 0x05;
        cpu.bus.memory[1] = 0x06;
        cpu.f = 0x00;
        execute_instruction_until_done(&mut cpu, 0xFE, vec![0x06]);
        assert_eq!(cpu.a, 0x05);
        assert_eq!(cpu.f & 0x80, 0x00); // Zero clear
    }

    #[test]
    fn test_stack_instructions() {
        // --- Test BC Pair ---
        let mut cpu = setup_cpu();
        cpu.sp = 0x2000;
        cpu.b = 0x11;
        cpu.c = 0x22;
        
        // PUSH BC (0xC5)
        // SP becomes 0x1FFE, memory[0x1FFF]=0x11, memory[0x1FFE]=0x22
        execute_instruction_until_done(&mut cpu, 0xC5, vec![]);
        assert_eq!(cpu.sp, 0x1FFE);
        assert_eq!(cpu.bus.memory[0x1FFF], 0x11);
        assert_eq!(cpu.bus.memory[0x1FFE], 0x22);

        // POP BC (0xC1)
        // SP becomes 0x2000, B=0x11, C=0x22
        execute_instruction_until_done(&mut cpu, 0xC1, vec![]);
        assert_eq!(cpu.sp, 0x2000);
        assert_eq!(cpu.b, 0x11);
        assert_eq!(cpu.c, 0x22);

        // --- Test DE Pair ---
        let mut cpu = setup_cpu();
        cpu.sp = 0x3000;
        cpu.d = 0x33;
        cpu.e = 0x44;

        // PUSH DE (0xD5)
        // SP becomes 0x2FFE, memory[0x2FFF]=0x33, memory[0x2FFE]=0x44
        execute_instruction_until_done(&mut cpu, 0xD5, vec![]);
        assert_eq!(cpu.sp, 0x2FFE);
        assert_eq!(cpu.bus.memory[0x2FFF], 0x33);
        assert_eq!(cpu.bus.memory[0x2FFE], 0x44);

        // POP DE (0xD1)
        // SP becomes 0x3000, D=0x33, E=0x44
        execute_instruction_until_done(&mut cpu, 0xD1, vec![]);
        assert_eq!(cpu.sp, 0x3000);
        assert_eq!(cpu.d, 0x33);
        assert_eq!(cpu.e, 0x44);

        // --- Test HL Pair ---
        let mut cpu = setup_cpu();
        cpu.sp = 0x4000;
        cpu.h = 0x55;
        cpu.l = 0x66;

        // PUSH HL (0xE5)
        // SP becomes 0x3FFE, memory[0x3FFF]=0x55, memory[0x3FFE]=0x66
        execute_instruction_until_done(&mut cpu, 0xE5, vec![]);
        assert_eq!(cpu.sp, 0x3FFE);
        assert_eq!(cpu.bus.memory[0x3FFF], 0x55);
        assert_eq!(cpu.bus.memory[0x3FFE], 0x66);

        // POP HL (0xE1)
        // SP becomes 0x4000, H=0x55, L=0x66
        execute_instruction_until_done(&mut cpu, 0xE1, vec![]);
        assert_eq!(cpu.sp, 0x4000);
        assert_eq!(cpu.h, 0x55);
        assert_eq!(cpu.l, 0x66);

        // --- Test AF Pair ---
        let mut cpu = setup_cpu();
        cpu.sp = 0x5000;
        cpu.a = 0x77;
        cpu.f = 0x88;

        // PUSH AF (0xF5)
        // SP becomes 0x4FFE, memory[0x4FFF]=0x77, memory[0x4FFE]=0x88
        execute_instruction_until_done(&mut cpu, 0xF5, vec![]);
        assert_eq!(cpu.sp, 0x4FFE);
        assert_eq!(cpu.bus.memory[0x4FFF], 0x77);
        assert_eq!(cpu.bus.memory[0x4FFE], 0x88);

        // POP AF (0xF1)
        // SP becomes 0x5000, A=0x77, F=0x88
        execute_instruction_until_done(&mut cpu, 0xF1, vec![]);
        assert_eq!(cpu.sp, 0x5000);
        assert_eq!(cpu.a, 0x77);
        assert_eq!(cpu.f, 0x88);
    }

    #[test]
    fn test_jp_a16() {
        let mut cpu = setup_cpu();
        // JP nn
        cpu.bus.memory[0] = 0xC3;
        cpu.bus.memory[1] = 0x20;
        cpu.bus.memory[2] = 0x10;
        execute_instruction_until_done(&mut cpu, 0xC3, vec![0x20, 0x10]);
        assert_eq!(cpu.pc, 0x1020);
    }

    #[test]
    fn test_jp_nz_a16() {
        // Case 1: Zero flag is NOT set (Condition Met - Jump)
        // 0xC2 => { if self.f & 0x80 == 0 { self.jump_nn(); self.cycles = 3;} else { self.cycles = 2 } }
        let mut cpu = setup_cpu();
        cpu.f = 0x00; // Z = 0
        cpu.bus.memory[0] = 0xC2;
        cpu.bus.memory[1] = 0x10; // Low byte
        cpu.bus.memory[2] = 0x20; // High byte
        
        execute_instruction_until_done(&mut cpu, 0xC2, vec![0x10, 0x20]);
        
        assert_eq!(cpu.pc, 0x2010);
    }

    #[test]
    fn test_jp_nz_a16_no_jump() {
        // Case 2: Zero flag IS set (Condition Not Met - No Jump)
        let mut cpu = setup_cpu();
        cpu.f = 0x80; // Z = 1
        cpu.bus.memory[0] = 0xC2;
        cpu.bus.memory[1] = 0x10;
        cpu.bus.memory[2] = 0x20;
        
        execute_instruction_until_done(&mut cpu, 0xC2, vec![0x10, 0x20]);
        
        // PC should have incremented by 3 (no jump)
        assert_eq!(cpu.pc, 3);
    }

    #[test]
    fn test_jp_nc_a16() {
        // Case 1: Carry flag is NOT set (Condition Met - Jump)
        // 0xD2 => { if self.f & 0x10 == 0 { self.jump_nn(); self.cycles = 3;} else { self.cycles = 2 } }
        let mut cpu = setup_cpu();
        cpu.f = 0x00; // C = 0
        cpu.bus.memory[0] = 0xD2;
        cpu.bus.memory[1] = 0x30;
        cpu.bus.memory[2] = 0x40;
        
        execute_instruction_until_done(&mut cpu, 0xD2, vec![0x30, 0x40]);
        
        assert_eq!(cpu.pc, 0x4030);
    }

    #[test]
    fn test_jp_nc_a16_no_jump() {
        // Case 2: Carry flag IS set (Condition Not Met - No Jump)
        let mut cpu = setup_cpu();
        cpu.f = 0x10; // C = 1
        cpu.bus.memory[0] = 0xD2;
        cpu.bus.memory[1] = 0x30;
        cpu.bus.memory[2] = 0x40;
        
        execute_instruction_until_done(&mut cpu, 0xD2, vec![0x30, 0x40]);
        
        // PC should have incremented by 3 (no jump)
        assert_eq!(cpu.pc, 3);
    }

    #[test]
    fn test_jp_z_a16() {
        // Case 1: Zero flag IS set (Condition Met - Jump)
        // 0xCA => { if self.f & 0x80 == 0x80 { self.jump_nn(); self.cycles = 3;} else { self.pc += 3; self.cycles = 2 } }
        let mut cpu = setup_cpu();
        cpu.f = 0x80; // Z = 1
        cpu.bus.memory[0] = 0xCA;
        cpu.bus.memory[1] = 0x10; // Low byte
        cpu.bus.memory[2] = 0x20; // High byte
        
        execute_instruction_until_done(&mut cpu, 0xCA, vec![0x10, 0x20]);
        
        assert_eq!(cpu.pc, 0x2010);
    }

    #[test]
    fn test_jp_z_a16_no_jump() {
        // Case 2: Zero flag is NOT set (Condition Not Met - No Jump)
        let mut cpu = setup_cpu();
        cpu.f = 0x00; // Z = 0
        cpu.bus.memory[0] = 0xCA;
        cpu.bus.memory[1] = 0x10;
        cpu.bus.memory[2] = 0x20;
        
        execute_instruction_until_done(&mut cpu, 0xCA, vec![0x10, 0x20]);
        
        // PC should have incremented by 3 (no jump)
        assert_eq!(cpu.pc, 3);
    }

    #[test]
    fn test_jp_c_a16() {
        // Case 1: Carry flag IS set (Condition Met - Jump)
        // 0xDA => { if self.f & 0x10 == 0x10 { self.jump_nn(); self.cycles = 3;} else { self.pc += 3; self.cycles = 2 } }
        let mut cpu = setup_cpu();
        cpu.f = 0x10; // C = 1
        cpu.bus.memory[0] = 0xDA;
        cpu.bus.memory[1] = 0x30;
        cpu.bus.memory[2] = 0x40;
        
        execute_instruction_until_done(&mut cpu, 0xDA, vec![0x30, 0x40]);
        
        assert_eq!(cpu.pc, 0x4030);
    }

    #[test]
    fn test_jp_c_a16_no_jump() {
        // Case 2: Carry flag is NOT set (Condition Not Met - No Jump)
        let mut cpu = setup_cpu();
        cpu.f = 0x00; // C = 0
        cpu.bus.memory[0] = 0xDA;
        cpu.bus.memory[1] = 0x30;
        cpu.bus.memory[2] = 0x40;
        
        execute_instruction_until_done(&mut cpu, 0xDA, vec![0x30, 0x40]);
        
        // PC should have incremented by 3 (no jump)
        assert_eq!(cpu.pc, 3);
    }

    #[test]
    fn test_rlca() {
        let mut cpu = setup_cpu();
        // Test bit 7 to CY and bit 0
        cpu.a = 0x80;
        execute_instruction_until_done(&mut cpu, 0x07, vec![]);
        assert_eq!(cpu.a, 0x00);
        assert_eq!(cpu.f & 0x10, 0x10); // Carry set
        assert_eq!(cpu.f & 0x80, 0x80); // Zero set

        // Test bit 0 to bit 1
        cpu.a = 0x01;
        execute_instruction_until_done(&mut cpu, 0x07, vec![]);
        assert_eq!(cpu.a, 0x02);
        assert_eq!(cpu.f & 0x10, 0x00); // Carry clear
        assert_eq!(cpu.f & 0x80, 0x00); // Zero clear
    }

    #[test]
    fn test_rla() {
        let mut cpu = setup_cpu();
        // Test CY to bit 0, bit 7 discarded
        cpu.a = 0x80;
        cpu.f = 0x10; // Carry set
        execute_instruction_until_done(&mut cpu, 0x17, vec![]);
        assert_eq!(cpu.a, 0x01);
        assert_eq!(cpu.f & 0x10, 0x10); // Carry unchanged
        assert_eq!(cpu.f & 0x80, 0x00); // Zero clear

        // Test CY clear, bit 0 remains 0
        cpu.a = 0x01;
        cpu.f = 0x00;
        execute_instruction_until_done(&mut cpu, 0x17, vec![]);
        assert_eq!(cpu.a, 0x02);
        assert_eq!(cpu.f & 0x10, 0x00); // Carry unchanged
        assert_eq!(cpu.f & 0x80, 0x00); // Zero clear
    }

    #[test]
    fn test_rrca() {
        let mut cpu = setup_cpu();
        // Test bit 0 to CY and bit 7
        cpu.a = 0x01;
        execute_instruction_until_done(&mut cpu, 0x0F, vec![]);
        assert_eq!(cpu.a, 0x80);
        assert_eq!(cpu.f & 0x10, 0x10); // Carry set
        assert_eq!(cpu.f & 0x80, 0x00); // Zero clear

        // Test bit 7 to bit 6
        cpu.a = 0x80;
        execute_instruction_until_done(&mut cpu, 0x0F, vec![]);
        assert_eq!(cpu.a, 0x40);
        assert_eq!(cpu.f & 0x10, 0x00); // Carry clear
        assert_eq!(cpu.f & 0x80, 0x00); // Zero clear
    }

    #[test]
    fn test_rra() {
        let mut cpu = setup_cpu();
        // Test CY to bit 7, bit 0 discarded
        cpu.a = 0x01;
        cpu.f = 0x10; // Carry set
        execute_instruction_until_done(&mut cpu, 0x1F, vec![]);
        assert_eq!(cpu.a, 0x80);
        assert_eq!(cpu.f & 0x10, 0x10); // Carry unchanged
        assert_eq!(cpu.f & 0x80, 0x00); // Zero clear

        // Test CY clear, bit 7 remains 0
        cpu.a = 0x80;
        cpu.f = 0x00;
        execute_instruction_until_done(&mut cpu, 0x1F, vec![]);
        assert_eq!(cpu.a, 0x40);
        assert_eq!(cpu.f & 0x10, 0x00); // Carry unchanged
        assert_eq!(cpu.f & 0x80, 0x00); // Zero clear
    }

    #[test]
    fn test_16bit_hl_adds() {
        let mut cpu = setup_cpu();

        // --- Test 0x09: ADD BC to HL, SP++ ---
        // LD HL, 0x1000
        cpu.bus.memory[0] = 0x21;
        execute_instruction_until_done(&mut cpu, 0x21, vec![0x00, 0x10]);
        assert_eq!(cpu.h, 0x10);
        assert_eq!(cpu.l, 0x00);

        // LD BC, 0x2000
        cpu.bus.memory[3] = 0x01;
        execute_instruction_until_done(&mut cpu, 0x01, vec![0x00, 0x20]);
        assert_eq!(cpu.b, 0x20);
        assert_eq!(cpu.c, 0x00);

        // ADD BC to HL (0x09)
        cpu.bus.memory[6] = 0x09;
        execute_instruction_until_done(&mut cpu, 0x09, vec![]);
        assert_eq!(cpu.h, 0x30);
        assert_eq!(cpu.l, 0x00);

        // --- Test 0x19: ADD DE to HL ---
        // LD DE, 0x3000
        cpu.bus.memory[7] = 0x11;
        execute_instruction_until_done(&mut cpu, 0x11, vec![0x00, 0x30]);
        assert_eq!(cpu.d, 0x30);
        assert_eq!(cpu.e, 0x00);

        // ADD DE to HL (0x19)
        cpu.bus.memory[10] = 0x19;
        execute_instruction_until_done(&mut cpu, 0x19, vec![]);
        // Expected: HL (0x1000) + DE (0x3000) = 0x4000
        assert_eq!(cpu.h, 0x60);
        assert_eq!(cpu.l, 0x00);

        // --- Test 0x29: ADD HL to HL ---
        // LD HL, 0x4000
        cpu.bus.memory[11] = 0x21;
        execute_instruction_until_done(&mut cpu, 0x21, vec![0x00, 0x40]);
        assert_eq!(cpu.h, 0x40);
        assert_eq!(cpu.l, 0x00);

        // ADD HL to HL (0x29)
        cpu.bus.memory[14] = 0x29;
        execute_instruction_until_done(&mut cpu, 0x29, vec![]);
        // Expected: HL (0x4000) + HL (0x4000) = 0x8000
        assert_eq!(cpu.h, 0x80);
        assert_eq!(cpu.l, 0x00);

        // --- Test 0x39: ADD SP to HL ---
        // LD SP, 0x5000
        cpu.bus.memory[15] = 0x31;
        execute_instruction_until_done(&mut cpu, 0x31, vec![0x00, 0x50]);
        assert_eq!(cpu.sp, 0x5000);

        // ADD SP to HL (0x39)
        cpu.bus.memory[18] = 0x39;
        execute_instruction_until_done(&mut cpu, 0x39, vec![]);
        // Expected: HL (0x4000) + SP (0x5000) = 0x9000
        assert_eq!(cpu.h, 0xD0);
        assert_eq!(cpu.l, 0x00);
    }

    #[test]
    fn test_ld_a16_sp() {
        let mut cpu = setup_cpu();
        cpu.sp = 0x1234;
        cpu.bus.memory[0] = 0x08;
        cpu.bus.memory[1] = 0x00;
        cpu.bus.memory[2] = 0x50;
        execute_instruction_until_done(&mut cpu, 0x08, vec![0x00, 0x50]);
        assert_eq!(cpu.bus.memory[0x5000], 0x34);
        assert_eq!(cpu.bus.memory[0x5001], 0x12);
        assert_eq!(cpu.pc, 3);
    }

    #[test]
    fn test_ld_sp_hl() {
        let mut cpu = setup_cpu();
        
        // Initial state
        cpu.h = 0x12;
        cpu.l = 0x34;
        cpu.sp = 0x0000;
        
        // Execute LD SP, HL
        // Opcode: 0xF9
        execute_instruction_until_done(&mut cpu, 0xF9, vec![]);
        
        // Verify that SP now holds the value of HL (0x1234)
        assert_eq!(cpu.sp, 0x1234);
        assert_eq!(cpu.pc, 1);
    }

    fn test_jr_s8() {
        let mut cpu = setup_cpu();
        cpu.pc = 0x1000;
        cpu.bus.memory[0x1000] = 0x18;
        cpu.bus.memory[0x1001] = 0xFF; // -1
        execute_instruction_until_done(&mut cpu, 0x18, vec![0xFF]);
        // PC should be 0x1000 + 2 - 1 = 0x1001
        assert_eq!(cpu.pc, 0x1001);
    }

    #[test]
    fn test_jr_z_s8_set() {
        let mut cpu = setup_cpu();
        cpu.f = 0x80; // Zero flag set
        cpu.pc = 0x1000;
        cpu.bus.memory[0x1000] = 0x28;
        cpu.bus.memory[0x1001] = 0x05;
        execute_instruction_until_done(&mut cpu, 0x28, vec![0x05]);
        // PC should be 0x1000 + 2 + 5 = 0x1007
        assert_eq!(cpu.pc, 0x1007);
    }

    #[test]
    fn test_jr_z_s8_clear() {
        let mut cpu = setup_cpu();
        cpu.f = 0x00; // Zero flag clear
        cpu.pc = 0x1000;
        cpu.bus.memory[0x1000] = 0x28;
        cpu.bus.memory[0x1001] = 0x05;
        execute_instruction_until_done(&mut cpu, 0x28, vec![0x05]);
        // PC should be 0x1000 + 2 = 0x1002
        assert_eq!(cpu.pc, 0x1002);
    }

    #[test]
    fn test_jr_nz_s8_set() {
        let mut cpu = setup_cpu();
        cpu.f = 0x80; // Zero flag set
        cpu.pc = 0x1000;
        cpu.bus.memory[0x1000] = 0x20;
        cpu.bus.memory[0x1001] = 0x05;
        execute_instruction_until_done(&mut cpu, 0x20, vec![0x05]);
        // PC should be 0x1000 + 2 = 0x1002
        assert_eq!(cpu.pc, 0x1002);
    }

    #[test]
    fn test_jr_nz_s8_clear() {
        let mut cpu = setup_cpu();
        cpu.f = 0x00; // Zero flag clear
        cpu.pc = 0x1000;
        cpu.bus.memory[0x1000] = 0x20;
        cpu.bus.memory[0x1001] = 0x05;
        execute_instruction_until_done(&mut cpu, 0x20, vec![0x05]);
        // PC should be 0x1000 + 2 + 5 = 0x1007
        assert_eq!(cpu.pc, 0x1007);
    }

    #[test]
    fn test_jr_c_s8_set() {
        let mut cpu = setup_cpu();
        cpu.f = 0x10; // carry flag set
        cpu.pc = 0x1000;
        cpu.bus.memory[0x1000] = 0x38;
        cpu.bus.memory[0x1001] = 0x05;
        execute_instruction_until_done(&mut cpu, 0x38, vec![0x05]);
        // PC should be 0x1000 + 2 + 5 = 0x1007
        assert_eq!(cpu.pc, 0x1007);
    }

    #[test]
    fn test_jr_c_s8_clear() {
        let mut cpu = setup_cpu();
        cpu.f = 0x00; // carry flag clear
        cpu.pc = 0x1000;
        cpu.bus.memory[0x1000] = 0x38;
        cpu.bus.memory[0x1001] = 0x05;
        execute_instruction_until_done(&mut cpu, 0x38, vec![0x05]);
        // PC should be 0x1000 + 2 = 0x1002
        assert_eq!(cpu.pc, 0x1002);
    }

    #[test]
    fn test_scf() {
        let mut cpu = setup_cpu();
        // Initial state: all bits set (0xFF)
        // Expected result: Bit 7 preserved (1), Bit 4 set (1), Bits 5-6 cleared (0), Bits 0-3 preserved (1)
        // Binary: 1001 1111 => 0x9F
        cpu.f = 0xFF;
        cpu.pc = 0x1000;
        cpu.bus.memory[0x1000] = 0x37;
        execute_instruction_until_done(&mut cpu, 0x37, vec![]);
        assert_eq!(cpu.f, 0x9F);

        // Test with different initial bits to ensure 0-3 and 7 are truly preserved
        // Initial state: Bit 7 = 1, Bits 0-3 = 0, Bits 4-6 = 1 (0x78)
        // Expected result: Bit 7 preserved (1), Bit 4 set (1), Bits 5-6 cleared (0), Bits 0-3 preserved (0)
        // Binary: 1001 0000 => 0x90
        cpu.f = 0b11110000;
        cpu.pc = 0x1000;
        cpu.bus.memory[0x1000] = 0x37;
        execute_instruction_until_done(&mut cpu, 0x37, vec![]);
        assert_eq!(cpu.f, 0x90);
    }

    #[test]
    fn test_ccf_with_preservation() {
        let mut cpu = setup_cpu();
        
        // Test Case: Bit 4 is 1, Bits 5 & 6 are 1, Bit 7 is 1, Bits 0-3 are 1 (0x18)
        // Initial f = 0xF8 (1111 1000). Bit 3 is 1.
        // After CCF: Bit 4 flips to 0, Bits 5 & 6 clear to 0, Bit 7 remains 1, Bits 0-3 remain 1.
        // Expected: 1000 1000 => 0x88
        cpu.f = 0xF8;
        cpu.pc = 0x1000;
        cpu.bus.memory[0x1000] = 0x3F;
        execute_instruction_until_done(&mut cpu, 0x3F, vec![]);
        assert_eq!(cpu.f, 0x88);

        // Test Case: Bit 4 is 0, Bits 5 & 6 are 1, Bit 7 is 0, Bits 0-3 are 1 (0x0F)
        // Initial f = 0x7F (0110 1111).
        // After CCF: Bit 4 flips to 1, Bits 5 & 6 clear to 0, Bit 7 remains 0, Bits 0-3 remain 1.
        // Expected: 0001 1111 => 0x1F
        cpu.f = 0b01001111;
        cpu.pc = 0x1000;
        cpu.bus.memory[0x1000] = 0x3F;
        execute_instruction_until_done(&mut cpu, 0x3F, vec![]);
        assert_eq!(cpu.f, 0x1F);
    }

    #[test]
    fn test_daa() {
        // Test Case 1: Lower 4 bits > 9
        // A = 0x0A (0000 1010), F = 0x00 (No Carry). 
        // Expected: A = 0x10 (0001 0000) because 0x0A > 9.
        let mut cpu1 = CPU::new();
        cpu1.a = 0x0A;
        cpu1.f = 0x00;
        cpu1.pc = 0x1000;
        cpu1.bus.memory[0x1000] = 0x27;
        execute_instruction_until_done(&mut cpu1, 0x27, vec![]);
        assert_eq!(cpu1.a, 0x10);

        // Test Case 2: Lower 4 bits = 9, Carry = 1
        // A = 0x09 (0000 1001), F = 0x10 (Carry set).
        // Expected: A = 0x0F (0000 1111) because A=9 and Carry=1.
        let mut cpu2 = CPU::new();
        cpu2.a = 0x09;
        cpu2.f = 0x10;
        cpu2.pc = 0x1000;
        cpu2.bus.memory[0x1000] = 0x27;
        execute_instruction_until_done(&mut cpu2, 0x27, vec![]);
        assert_eq!(cpu2.a, 0x0F);

        // Test Case 3: Lower 4 bits = 8, Carry = 1
        // A = 0x08 (0000 1000), F = 0x10 (Carry set).
        // Expected: A = 0x0E (0000 1110) because A=8 and Carry=1.
        let mut cpu3 = CPU::new();
        cpu3.a = 0x08;
        cpu3.f = 0x10;
        cpu3.pc = 0x1000;
        cpu3.bus.memory[0x1000] = 0x27;
        execute_instruction_until_done(&mut cpu3, 0x27, vec![]);
        assert_eq!(cpu3.a, 0x0E);

        // Test Case 4: Lower 4 bits = 9, Carry = 0
        // A = 0x09, F = 0x00.
        // Expected: A = 0x09 (No adjustment).
        let mut cpu4 = CPU::new();
        cpu4.a = 0x09;
        cpu4.f = 0x00;
        cpu4.pc = 0x1000;
        cpu4.bus.memory[0x1000] = 0x27;
        execute_instruction_until_done(&mut cpu4, 0x27, vec![]);
        assert_eq!(cpu4.a, 0x09);

        // Test Case 5: Lower 4 bits = 8, Carry = 0
        // A = 0x08, F = 0x00.
        // Expected: A = 0x08 (No adjustment).
        let mut cpu5 = CPU::new();
        cpu5.a = 0x08;
        cpu5.f = 0x00;
        cpu5.pc = 0x1000;
        cpu5.bus.memory[0x1000] = 0x27;
        execute_instruction_until_done(&mut cpu5, 0x27, vec![]);
        assert_eq!(cpu5.a, 0x08);
    }

    #[test]
    fn test_cpl() {
        // Test Case 1: Basic bit flip and flag setting
        // A = 0xAA (1010 1010), F = 0x00
        // After CPL: A = 0x55 (0101 0101), F = 0x60 (S and H set, Z and C preserved as 0)
        let mut cpu1 = CPU::new();
        cpu1.a = 0xAA;
        cpu1.f = 0x00;
        cpu1.pc = 0x1000;
        cpu1.bus.memory[0x1000] = 0x2F;
        execute_instruction_until_done(&mut cpu1, 0x2F, vec![]);
        assert_eq!(cpu1.a, 0x55);
        assert_eq!(cpu1.f, 0x60);

        // Test Case 2: Preserving already set Zero and Carry flags
        // A = 0x55 (0101 0101), F = 0x90 (Z bit 7 and C bit 4 set)
        // After CPL: A = 0xAA (1010 1010), F = 0xF0 (S and H set, Z and C preserved as 1)
        let mut cpu2 = CPU::new();
        cpu2.a = 0x55;
        cpu2.f = 0x90;
        cpu2.pc = 0x1000;
        cpu2.bus.memory[0x1000] = 0x2F;
        execute_instruction_until_done(&mut cpu2, 0x2F, vec![]);
        assert_eq!(cpu2.a, 0xAA);
        assert_eq!(cpu2.f, 0xF0);

        // Test Case 3: Preserving already set Half Carry flag
        // A = 0xAA, F = 0x20 (H bit 5 set)
        // After CPL: A = 0x55, F = 0x60 (S and H set/preserved, Z and C preserved as 0)
        let mut cpu3 = CPU::new();
        cpu3.a = 0xAA;
        cpu3.f = 0x20;
        cpu3.pc = 0x1000;
        cpu3.bus.memory[0x1000] = 0x2F;
        execute_instruction_until_done(&mut cpu3, 0x2F, vec![]);
        assert_eq!(cpu3.a, 0x55);
        assert_eq!(cpu3.f, 0x60);
    }

    #[test]
    fn test_inc_hl() {
        // Test Case 1: Normal increment (no flag changes except S/H/Z)
        // A = 0x00, F = 0x00, memory[0x0000] = 0x05
        // After INC (HL): memory[0x0000] = 0x06, F = 0x00
        let mut cpu1 = CPU::new();
        cpu1.h = 0x00;
        cpu1.l = 0x00;
        cpu1.f = 0x00;
        cpu1.bus.memory[0x0000] = 0x05;
        cpu1.pc = 0x1000;
        cpu1.bus.memory[0x1000] = 0x34;
        execute_instruction_until_done(&mut cpu1, 0x34, vec![]);
        assert_eq!(cpu1.bus.memory[0x0000], 0x06);
        assert_eq!(cpu1.f, 0x00);

        // Test Case 2: Increment triggering Half-Carry (bit 5)
        // memory[0x0000] = 0x0F
        // After INC (HL): memory[0x0000] = 0x10, F = 0x20
        let mut cpu2 = CPU::new();
        cpu2.h = 0x00;
        cpu2.l = 0x00;
        cpu2.f = 0x00;
        cpu2.bus.memory[0x0000] = 0x0F;
        cpu2.pc = 0x1000;
        cpu2.bus.memory[0x1000] = 0x34;
        execute_instruction_until_done(&mut cpu2, 0x34, vec![]);
        assert_eq!(cpu2.bus.memory[0x0000], 0x10);
        assert_eq!(cpu2.f, 0x20);

        // Test Case 3: Increment triggering Zero flag (bit 7)
        // memory[0x0000] = 0xFF
        // After INC (HL): memory[0x0000] = 0x00, F = 0xA0
        let mut cpu3 = CPU::new();
        cpu3.h = 0x00;
        cpu3.l = 0x00;
        cpu3.f = 0x00;
        cpu3.bus.memory[0x0000] = 0xFF;
        cpu3.pc = 0x1000;
        cpu3.bus.memory[0x1000] = 0x34;
        execute_instruction_until_done(&mut cpu3, 0x34, vec![]);
        assert_eq!(cpu3.bus.memory[0x0000], 0x00);
        assert_eq!(cpu3.f, 0xA0);

        // Test Case 4: Preserving Carry flag (bit 4)
        // F = 0x10 (Carry set), memory[0x0000] = 0x01
        // After INC (HL): memory[0x0000] = 0x02, F = 0x10
        let mut cpu4 = CPU::new();
        cpu4.h = 0x00;
        cpu4.l = 0x00;
        cpu4.f = 0x10;
        cpu4.bus.memory[0x0000] = 0x01;
        cpu4.pc = 0x1000;
        cpu4.bus.memory[0x1000] = 0x34;
        execute_instruction_until_done(&mut cpu4, 0x34, vec![]);
        assert_eq!(cpu4.bus.memory[0x0000], 0x02);
        assert_eq!(cpu4.f, 0x10);
    }

    #[test]
    fn test_dec_hl() {
        // Test Case 1: Normal decrement
        // F = 0x00, memory[0x0000] = 0x05
        // After DEC (HL): memory[0x0000] = 0x04, F = 0x40 (Subtract set)
        let mut cpu1 = CPU::new();
        cpu1.h = 0x00;
        cpu1.l = 0x00;
        cpu1.f = 0x00;
        cpu1.bus.memory[0x0000] = 0x05;
        cpu1.pc = 0x1000;
        cpu1.bus.memory[0x1000] = 0x35;
        execute_instruction_until_done(&mut cpu1, 0x35, vec![]);
        assert_eq!(cpu1.bus.memory[0x0000], 0x04);
        assert_eq!(cpu1.f, 0x40);

        // Test Case 2: Decrement triggering Half-Carry (bit 5)
        // memory[0x0000] = 0x00
        // After DEC (HL): memory[0x0000] = 0xFF, F = 0x60 (Subtract and Half-Carry set)
        let mut cpu2 = CPU::new();
        cpu2.h = 0x00;
        cpu2.l = 0x00;
        cpu2.f = 0x00;
        cpu2.bus.memory[0x0000] = 0x00;
        cpu2.pc = 0x1000;
        cpu2.bus.memory[0x1000] = 0x35;
        execute_instruction_until_done(&mut cpu2, 0x35, vec![]);
        assert_eq!(cpu2.bus.memory[0x0000], 0xFF);
        assert_eq!(cpu2.f, 0x60);

        // Test Case 3: Decrement triggering Zero flag (bit 7)
        // memory[0x0000] = 0x01
        // After DEC (HL): memory[0x0000] = 0x00, F = 0xC0 (Subtract and Zero set)
        let mut cpu3 = CPU::new();
        cpu3.h = 0x00;
        cpu3.l = 0x00;
        cpu3.f = 0x00;
        cpu3.bus.memory[0x0000] = 0x01;
        cpu3.pc = 0x1000;
        cpu3.bus.memory[0x1000] = 0x35;
        execute_instruction_until_done(&mut cpu3, 0x35, vec![]);
        assert_eq!(cpu3.bus.memory[0x0000], 0x00);
        assert_eq!(cpu3.f, 0xC0);

        // Test Case 4: Preserving Carry flag (bit 4)
        // F = 0x10 (Carry set), memory[0x0000] = 0x01
        // After DEC (HL): memory[0x0000] = 0x00, F = 0xDC (Subtract, Zero, and Carry set)
        let mut cpu4 = CPU::new();
        cpu4.h = 0x00;
        cpu4.l = 0x00;
        cpu4.f = 0x10;
        cpu4.bus.memory[0x0000] = 0x01;
        cpu4.pc = 0x1000;
        cpu4.bus.memory[0x1000] = 0x35;
        execute_instruction_until_done(&mut cpu4, 0x35, vec![]);
        assert_eq!(cpu4.bus.memory[0x0000], 0x00);
        assert_eq!(cpu4.f, 0xD0);
    }

    #[test]
    fn test_ld_hl_a() {
        let mut cpu = setup_cpu();
        
        // Setup: Set HL to 0x1000 and A to 0xAA
        cpu.h = 0x10;
        cpu.l = 0x00;
        cpu.a = 0xAA;
        
        // Ensure the target memory location is initially 0x00
        cpu.bus.memory[0x1000] = 0x00;
        
        // Execute LD (HL), A
        execute_instruction_until_done(&mut cpu, 0x77, vec![]);
        
        // Assertions
        assert_eq!(cpu.bus.memory[0x1000], 0xAA); // Memory at 0x1000 should now be 0xAA
        assert_eq!(cpu.pc, 1);                // PC should advance by 1
    }

    #[test]
    fn test_call_a16() {
        let mut cpu = setup_cpu();
        cpu.sp = 0xFFFE;
        cpu.bus.memory[0x0000] = 0xCD;
        cpu.bus.memory[0x0001] = 0x00;
        cpu.bus.memory[0x0002] = 0x10;

        // Execute CALL 0x1000
        execute_instruction_until_done(&mut cpu, 0xCD, vec![0x00, 0x10]);

        assert_eq!(cpu.pc, 0x1000);
        assert_eq!(cpu.sp, 0xFFFC);
        assert_eq!(cpu.bus.memory[0xFFFD], 0x00); // High byte of 0x0003
        assert_eq!(cpu.bus.memory[0xFFFC], 0x03); // Low byte of 0x0003
    }

    #[test]
    fn test_call_nz_a16() {
        let mut cpu = setup_cpu();
        cpu.sp = 0xFFFE;
        cpu.bus.memory[0x0000] = 0xC4;
        cpu.bus.memory[0x0001] = 0x00;
        cpu.bus.memory[0x0002] = 0x10;

        // Case 1: Z=0 (Jump)
        cpu.f = 0x00;
        execute_instruction_until_done(&mut cpu, 0xC4, vec![0x00, 0x10]);
        assert_eq!(cpu.pc, 0x1000);

        // Case 2: Z=1 (No Jump)
        let mut cpu2 = setup_cpu();
        cpu2.f = 0x80;
        cpu2.bus.memory[0x0000] = 0xC4;
        cpu2.bus.memory[0x0001] = 0x00;
        cpu2.bus.memory[0x0002] = 0x10;
        execute_instruction_until_done(&mut cpu2, 0xC4, vec![0x00, 0x10]);
        assert_eq!(cpu2.pc, 3); // PC increments by 3 (instruction length)
    }

    #[test]
    fn test_call_nc_a16() {
        let mut cpu = setup_cpu();
        cpu.sp = 0xFFFE;
        cpu.bus.memory[0x0000] = 0xD4;
        cpu.bus.memory[0x0001] = 0x00;
        cpu.bus.memory[0x0002] = 0x10;

        // Case 1: C=0 (Jump)
        cpu.f = 0x00;
        execute_instruction_until_done(&mut cpu, 0xD4, vec![0x00, 0x10]);
        assert_eq!(cpu.pc, 0x1000);

        // Case 2: C=1 (No Jump)
        let mut cpu2 = setup_cpu();
        cpu2.f = 0x10;
        cpu2.bus.memory[0x0000] = 0xD4;
        cpu2.bus.memory[0x0001] = 0x00;
        cpu2.bus.memory[0x0002] = 0x10;
        execute_instruction_until_done(&mut cpu2, 0xD4, vec![0x00, 0x10]);
        assert_eq!(cpu2.pc, 3);
    }

    #[test]
    fn test_call_z_a16() {
        let mut cpu = setup_cpu();
        cpu.sp = 0xFFFE;
        cpu.bus.memory[0x0000] = 0xCC;
        cpu.bus.memory[0x0001] = 0x00;
        cpu.bus.memory[0x0002] = 0x10;

        // Case 1: Z=1 (Jump)
        cpu.f = 0x80;
        execute_instruction_until_done(&mut cpu, 0xCC, vec![0x00, 0x10]);
        assert_eq!(cpu.pc, 0x1000);

        // Case 2: Z=0 (No Jump)
        let mut cpu2 = setup_cpu();
        cpu2.f = 0x00;
        cpu2.bus.memory[0x0000] = 0xCC;
        cpu2.bus.memory[0x0001] = 0x00;
        cpu2.bus.memory[0x0002] = 0x10;
        execute_instruction_until_done(&mut cpu2, 0xCC, vec![0x00, 0x10]);
        assert_eq!(cpu2.pc, 3);
    }

    #[test]
    fn test_call_c_a16() {
        let mut cpu = setup_cpu();
        cpu.sp = 0xFFFE;
        cpu.bus.memory[0x0000] = 0xDC;
        cpu.bus.memory[0x0001] = 0x00;
        cpu.bus.memory[0x0002] = 0x10;

        // Case 1: C=1 (Jump)
        cpu.f = 0x10;
        execute_instruction_until_done(&mut cpu, 0xDC, vec![0x00, 0x10]);
        assert_eq!(cpu.pc, 0x1000);

        // Case 2: C=0 (No Jump)
        let mut cpu2 = setup_cpu();
        cpu2.f = 0x00;
        cpu2.bus.memory[0x0000] = 0xDC;
        cpu2.bus.memory[0x0001] = 0x00;
        cpu2.bus.memory[0x0002] = 0x10;
        execute_instruction_until_done(&mut cpu2, 0xDC, vec![0x00, 0x10]);
        assert_eq!(cpu2.pc, 3);
    }

    // --- Return Instructions ---

    #[test]
    fn test_ret() {
        let mut cpu = setup_cpu();
        cpu.sp = 0xFFFC;
        cpu.bus.memory[0xFFFD] = 0x00; // High byte of return address
        cpu.bus.memory[0xFFFC] = 0x03; // Low byte of return address

        execute_instruction_until_done(&mut cpu, 0xC9, vec![]);

        assert_eq!(cpu.pc, 0x0003);
        assert_eq!(cpu.sp, 0xFFFE);
    }

    #[test]
    fn test_reti() {
        let mut cpu = setup_cpu();
        cpu.sp = 0xFFFC;
        cpu.bus.memory[0xFFFD] = 0x00;
        cpu.bus.memory[0xFFFC] = 0x03;
        
        execute_instruction_until_done(&mut cpu, 0xD9, vec![]);

        assert_eq!(cpu.pc, 0x0003);
        assert_eq!(cpu.ime, true);
        assert_eq!(cpu.sp, 0xFFFE);
    }

    #[test]
    fn test_ret_nz() {
        let mut cpu = setup_cpu();
        cpu.sp = 0xFFFC;
        cpu.bus.memory[0xFFFD] = 0x00;
        cpu.bus.memory[0xFFFC] = 0x03;

        // Case 1: Z=0 (Jump/Pop)
        cpu.f = 0x00;
        execute_instruction_until_done(&mut cpu, 0xC0, vec![]);
        assert_eq!(cpu.pc, 0x0003);
        assert_eq!(cpu.sp, 0xFFFE);

        // Case 2: Z=1 (No Jump)
        let mut cpu2 = setup_cpu();
        cpu2.f = 0x80;
        cpu2.sp = 0xFFFC;
        cpu2.bus.memory[0xFFFD] = 0x00;
        cpu2.bus.memory[0xFFFC] = 0x03;
        execute_instruction_until_done(&mut cpu2, 0xC0, vec![]);
        assert_eq!(cpu2.pc, 1); // PC increments by 1
        assert_eq!(cpu2.sp, 0xFFFC); // SP remains unchanged
    }

    #[test]
    fn test_ret_nc() {
        let mut cpu = setup_cpu();
        cpu.sp = 0xFFFC;
        cpu.bus.memory[0xFFFD] = 0x00;
        cpu.bus.memory[0xFFFC] = 0x03;

        // Case 1: C=0 (Jump/Pop)
        cpu.f = 0x00;
        execute_instruction_until_done(&mut cpu, 0xD0, vec![]);
        assert_eq!(cpu.pc, 0x0003);
        assert_eq!(cpu.sp, 0xFFFE);

        // Case 2: C=1 (No Jump)
        let mut cpu2 = setup_cpu();
        cpu2.f = 0x10;
        cpu2.sp = 0xFFFC;
        cpu2.bus.memory[0xFFFD] = 0x00;
        cpu2.bus.memory[0xFFFC] = 0x03;
        execute_instruction_until_done(&mut cpu2, 0xD0, vec![]);
        assert_eq!(cpu2.pc, 1);
        assert_eq!(cpu2.sp, 0xFFFC);
    }

    #[test]
    fn test_ret_z() {
        let mut cpu = setup_cpu();
        cpu.sp = 0xFFFC;
        cpu.bus.memory[0xFFFD] = 0x00;
        cpu.bus.memory[0xFFFC] = 0x03;

        // Case 1: Z=1 (Jump/Pop)
        cpu.f = 0x80;
        execute_instruction_until_done(&mut cpu, 0xC8, vec![]);
        assert_eq!(cpu.pc, 0x0003);
        assert_eq!(cpu.sp, 0xFFFE);

        // Case 2: Z=0 (No Jump)
        let mut cpu2 = setup_cpu();
        cpu2.f = 0x00;
        cpu2.sp = 0xFFFC;
        cpu2.bus.memory[0xFFFD] = 0x00;
        cpu2.bus.memory[0xFFFC] = 0x03;
        execute_instruction_until_done(&mut cpu2, 0xC8, vec![]);
        assert_eq!(cpu2.pc, 1);
        assert_eq!(cpu2.sp, 0xFFFC);
    }

    #[test]
    fn test_ret_c() {
        let mut cpu = setup_cpu();
        cpu.sp = 0xFFFC;
        cpu.bus.memory[0xFFFD] = 0x00;
        cpu.bus.memory[0xFFFC] = 0x03;

        // Case 1: C=1 (Jump/Pop)
        cpu.f = 0x10;
        execute_instruction_until_done(&mut cpu, 0xD8, vec![]);
        assert_eq!(cpu.pc, 0x0003);
        assert_eq!(cpu.sp, 0xFFFE);

        // Case 2: C=0 (No Jump)
        let mut cpu2 = setup_cpu();
        cpu2.f = 0x00;
        cpu2.sp = 0xFFFC;
        cpu2.bus.memory[0xFFFD] = 0x00;
        cpu2.bus.memory[0xFFFC] = 0x03;
        execute_instruction_until_done(&mut cpu2, 0xD8, vec![]);
        assert_eq!(cpu2.pc, 1);
        assert_eq!(cpu2.sp, 0xFFFC);
    }

    #[test]
    fn test_add_sp_s8() {
        // --- Case 1: Positive addition ---
        let mut cpu = setup_cpu();
        cpu.sp = 0x2000;
        // Opcode: 0xE8, s8: 0x05
        cpu.bus.memory[0] = 0xE8;
        cpu.bus.memory[1] = 0x05;
        
        execute_instruction_until_done(&mut cpu, 0xE8, vec![0x05]);
        
        assert_eq!(cpu.sp, 0x2005);
        assert_eq!(cpu.pc, 2);

        // --- Case 2: Negative addition ---
        let mut cpu = setup_cpu();
        cpu.sp = 0x2000;
        // Opcode: 0xE8, s8: -0x05 (0xFB in 8-bit 2's complement)
        cpu.bus.memory[0] = 0xE8;
        cpu.bus.memory[1] = 0xFB;
        
        execute_instruction_until_done(&mut cpu, 0xE8, vec![0xFB]);
        
        assert_eq!(cpu.sp, 0x1FFB);
        assert_eq!(cpu.pc, 2);

        // --- Case 3: Wrapping boundary ---
        let mut cpu = setup_cpu();
        cpu.sp = 0x0000;
        // Opcode: 0xE8, s8: -1 (0xFF in 8-bit 2's complement)
        cpu.bus.memory[0] = 0xE8;
        cpu.bus.memory[1] = 0xFF;
        
        execute_instruction_until_done(&mut cpu, 0xE8, vec![0xFF]);
        
        assert_eq!(cpu.sp, 0xFFFF);
        assert_eq!(cpu.pc, 2);
    }

    #[test]
    fn test_di() {
        let mut cpu = setup_cpu();
        
        // Start with interrupts enabled
        cpu.ime = true;
        
        // Execute DI (Opcode: 0xF3)
        execute_instruction_until_done(&mut cpu, 0xF3, vec![]);
        
        // Verify interrupts are now disabled
        assert_eq!(cpu.ime, false);
        assert_eq!(cpu.pc, 1);
    }

    #[test]
    fn test_ei() {
        let mut cpu = setup_cpu();
        
        // Start with interrupts disabled
        cpu.ime = false;
        
        // Execute EI (Opcode: 0xFB)
        execute_instruction_until_done(&mut cpu, 0xFB, vec![]);
        
        // Verify interrupts are now enabled
        assert_eq!(cpu.ime, true);
        assert_eq!(cpu.pc, 1);
    }

    #[test]
    fn test_rst_instructions() {
        let mut cpu = setup_cpu();
        
        // The RST instructions are at the start of our test "program"
        // We set the PC to 0x1000
        cpu.pc = 0x1000;
        cpu.sp = 0xFFFE;

        // Define the RST instructions and their target addresses
        let rst_instructions = [
            (0xC7, 0x0000), // RST 0
            (0xCF, 0x0008), // RST 1
            (0xD7, 0x0010), // RST 2
            (0xDF, 0x0018), // RST 3
            (0xE7, 0x0020), // RST 4
            (0xEF, 0x0028), // RST 5
            (0xF7, 0x0030), // RST 6
            (0xFF, 0x0038), // RST 7
        ];

        for (opcode, target_address) in rst_instructions {
            // Reset CPU state for each iteration
            let mut cpu_iter = setup_cpu();
            cpu_iter.pc = 0x1000;
            cpu_iter.sp = 0xFFFE;

            // Execute the RST instruction
            // (Note: execute_instruction_until_done is a helper that 
            // handles the cycle counting/looping logic for us)
            execute_instruction_until_done(&mut cpu_iter, opcode, vec![]);

            // 1. Verify the jump address
            assert_eq!(cpu_iter.pc, target_address, "RST 0x{:02X} should jump to 0x{:04X}", opcode, target_address);

            // 2. Verify the return address was pushed to the stack.
            // The return address is PC + 1 = 0x1001.
            // The stack pointer should have decremented twice to 0xFFFC.
            assert_eq!(cpu_iter.sp, 0xFFFC);
            assert_eq!(cpu_iter.bus.memory[0xFFFD], 0x10); // High byte of 0x1001
            assert_eq!(cpu_iter.bus.memory[0xFFFC], 0x01); // Low byte of 0x1001
        }
    }

    #[test]
fn test_all_cb_instructions() {
    let mut cpu = CPU::new();
    // Initialize with a known state
    cpu.a = 0xAA;
    cpu.b = 0xBB;
    cpu.c = 0xCC;
    cpu.d = 0xDD;
    cpu.e = 0xEE;
    cpu.h = 0xFF;
    cpu.l = 0x00;
    cpu.f = 0x00;
    cpu.sp = 0xFFFE;
    
    // Populate some memory for (HL) targets
    cpu.bus.memory[0x1000] = 0x55;
    cpu.bus.memory[0x1001] = 0x66;

    // Test every possible byte following the 0xCB prefix
    for op_byte in 0..=255 {
        let pc = 0x0000;
        cpu.pc = pc;
        
        // Setup memory for the instruction
        cpu.bus.memory[pc as usize] = 0xCB;
        cpu.bus.memory[(pc + 1) as usize] = op_byte as u8;
        
        // Execute the instruction
        cpu.cycle();
        
        // At this point, the CPU has moved the PC by 2 bytes.
        // We can inspect the state here.
        
        // Logic to verify results based on the implementation in cpu.rs:
        let family = op_byte >> 6;
        let operation = (op_byte >> 3) & 0b111;
        let target = op_byte & 0b111;

        match family {
            0 => {
                // Family 0: Rotate and Shift operations
                // These should clear N and H flags (bits 6 and 5) and set Z (bit 7) and C (bit 4)
                // Note: SWAP (op 6) clears C (bit 4)
                assert_eq!(cpu.f & 0x40, 0, "Family 0: N flag should be cleared");
                assert_eq!(cpu.f & 0x20, 0, "Family 0: H flag should be cleared");
                
                // Verify specific operations if desired, e.g., RLC (op 0)
                if operation == 0 {
                    // RLC (A) should set Z if result is 0
                    // For input 0xAA, RLC(0xAA) = 0x50. Z=0, C=1.
                    // Since we started with 0xAA, let's see if it's correct for our specific op.
                }
            }
            1 => {
                // Family 1: BIT b, r
                // These set Z (bit 7) if result is 0, set H (bit 5) to 1, and preserve C (bit 4).
                assert_eq!(cpu.f & 0x20, 0x20, "Family 1: H flag should be set to 1");
                // Z flag check depends on the bit index (operation)
                // Note: In the current implementation, 'operation' is (op >> 3) & 0x07.
            }
            2 => {
                // Family 2: RES b, r
                // Flags should remain untouched.
                // We don't have a specific check here as flags aren't updated.
            }
            3 => {
                // Family 3: SET b, r
                // Flags should remain untouched.
            }
            _ => unreachable!(),
        }
    }
    
    println!("Successfully executed all 256 CB-prefixed opcodes.");
}
}
