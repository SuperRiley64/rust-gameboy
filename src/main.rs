mod gameboy;
use gameboy::cpu::CPU;

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_cpu() -> CPU {
        CPU::new()
    }

    fn execute_instruction_until_done(cpu: &mut CPU, opcode: u8, operands: Vec<u8>) {
        cpu.memory[cpu.pc as usize] = opcode;
        for (i, &op) in operands.iter().enumerate() {
            cpu.memory[(cpu.pc as usize + 1 + i)] = op;
        }

        cpu.cycle(); 
        while cpu.cycles > 0 {
            cpu.cycle();
        }
    }

    #[test]
    fn test_nop() {
        let mut cpu = setup_cpu();
        cpu.memory[0] = 0x00;
        cpu.cycle(); 
        assert_eq!(cpu.pc, 1);
        assert_eq!(cpu.cycles, 0);
    }

    #[test]
    fn test_stop() {
        let mut cpu = setup_cpu();
        cpu.memory[0] = 0x10;
        cpu.cycle(); 
        assert_eq!(cpu.pc, 2);
        assert_eq!(cpu.cycles, 0);
    }

    #[test]
    fn test_halt() {
        let mut cpu = setup_cpu();
        cpu.memory[0] = 0x76;
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
        cpu.memory[0] = 0xFA;
        cpu.memory[1] = 0x00;
        cpu.memory[2] = 0x10;
        cpu.memory[0x1000] = 0xBB;
        execute_instruction_until_done(&mut cpu, 0xFA, vec![0x00, 0x10]);
        assert_eq!(cpu.a, 0xBB);
        assert_eq!(cpu.pc, 3);
    }

    #[test]
    fn test_ld_a_a8() {
        let mut cpu = setup_cpu();
        cpu.memory[0] = 0xF0;
        cpu.memory[1] = 0x20;
        cpu.memory[0x20] = 0xCC;
        execute_instruction_until_done(&mut cpu, 0xF0, vec![0x20]);
        assert_eq!(cpu.a, 0xCC);
        assert_eq!(cpu.pc, 2);
    }

    #[test]
    fn test_ld_a_c() {
        let mut cpu = setup_cpu();
        cpu.c = 0x30;
        cpu.memory[0] = 0xF2;
        cpu.memory[0xFF30] = 0xDD;
        execute_instruction_until_done(&mut cpu, 0xF2, vec![]);
        assert_eq!(cpu.a, 0xDD);
        assert_eq!(cpu.pc, 1);
    }

    #[test]
    fn test_ld_a_bc() {
        let mut cpu = setup_cpu();
        cpu.b = 0x10;
        cpu.c = 0x20;
        cpu.memory[0] = 0x0A;
        cpu.memory[0x1020] = 0xEE;
        execute_instruction_until_done(&mut cpu, 0x0A, vec![]);
        assert_eq!(cpu.a, 0xEE);
        assert_eq!(cpu.pc, 1);
    }

    #[test]
    fn test_ld_a_de() {
        let mut cpu = setup_cpu();
        cpu.d = 0x30;
        cpu.e = 0x40;
        cpu.memory[0] = 0x1A;
        cpu.memory[0x3040] = 0xFF;
        execute_instruction_until_done(&mut cpu, 0x1A, vec![]);
        assert_eq!(cpu.a, 0xFF);
        assert_eq!(cpu.pc, 1);
    }

    #[test]
    fn test_ld_a_hl_plus() {
        let mut cpu = setup_cpu();
        cpu.h = 0x10;
        cpu.l = 0x20;
        cpu.memory[0] = 0x2A;
        cpu.memory[0x1020] = 0x11;
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
        cpu.memory[0] = 0x3A;
        cpu.memory[0x1020] = 0x22;
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
        cpu.memory[0] = 0x02;
        execute_instruction_until_done(&mut cpu, 0x02, vec![]);
        assert_eq!(cpu.memory[0x1200], 0x11);
        assert_eq!(cpu.pc, 1);
    }

    #[test]
    fn test_ld_de_a() {
        let mut cpu = setup_cpu();
        cpu.a = 0x22;
        cpu.d = 0x34;
        cpu.e = 0x00;
        cpu.memory[0] = 0x12;
        execute_instruction_until_done(&mut cpu, 0x12, vec![]);
        assert_eq!(cpu.memory[0x3400], 0x22);
        assert_eq!(cpu.pc, 1);
    }

    #[test]
    fn test_ld_hl_minus_a() {
        let mut cpu = setup_cpu();
        cpu.a = 0x33;
        cpu.h = 0x10;
        cpu.l = 0x20;
        cpu.memory[0] = 0x32;
        cpu.memory[0x1020] = 0x33;
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
        cpu.memory[0] = 0x22;
        cpu.memory[0x1020] = 0x44;
        execute_instruction_until_done(&mut cpu, 0x22, vec![]);
        assert_eq!(cpu.h, 0x10);
        assert_eq!(cpu.l, 0x21);
        assert_eq!(cpu.pc, 1);
    }

    #[test]
    fn test_ld_a16_a() {
        let mut cpu = setup_cpu();
        cpu.a = 0x55;
        cpu.memory[0] = 0xEA;
        cpu.memory[1] = 0x00;
        cpu.memory[2] = 0x30;
        cpu.memory[0x3000] = 0x55;
        execute_instruction_until_done(&mut cpu, 0xEA, vec![0x00, 0x30]);
        assert_eq!(cpu.memory[0x3000], 0x55);
        assert_eq!(cpu.pc, 3);
    }

    #[test]
    fn test_ld_c_a() {
        let mut cpu = setup_cpu();
        cpu.a = 0x66;
        cpu.c = 0x60;
        cpu.memory[0] = 0xE2;
        cpu.memory[0x60] = 0x66;
        execute_instruction_until_done(&mut cpu, 0xE2, vec![]);
        assert_eq!(cpu.memory[0x60], 0x66);
        assert_eq!(cpu.pc, 1);
    }

    #[test]
    fn test_ld_a8_a() {
        let mut cpu = setup_cpu();
        cpu.a = 0x77;
        cpu.memory[0] = 0xE0;
        cpu.memory[1] = 0x80;
        cpu.memory[0xFF80] = 0x77;
        execute_instruction_until_done(&mut cpu, 0xE0, vec![0x80]);
        assert_eq!(cpu.memory[0xFF80], 0x77);
        assert_eq!(cpu.pc, 2);
    }

    #[test]
    fn test_ld_hl_d8() {
        let mut cpu = setup_cpu();
        cpu.h = 0x10;
        cpu.l = 0x20;
        cpu.memory[0] = 0x36;
        cpu.memory[1] = 0x88;
        execute_instruction_until_done(&mut cpu, 0x36, vec![0x88]);
        assert_eq!(cpu.memory[0x1020], 0x88);
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
        assert_eq!(cpu.memory[0x1220], 0x99);
        assert_eq!(cpu.pc, 1);

        // LD (HL), C
        cpu.c = 0xAA;
        execute_instruction_until_done(&mut cpu, 0x71, vec![]);
        assert_eq!(cpu.memory[0x1220], 0xAA);
        assert_eq!(cpu.pc, 2);

        // LD (HL), D
        cpu.d = 0xBB;
        execute_instruction_until_done(&mut cpu, 0x72, vec![]);
        assert_eq!(cpu.memory[0x1220], 0xBB);
        assert_eq!(cpu.pc, 3);

        // LD (HL), E
        cpu.e = 0xCC;
        execute_instruction_until_done(&mut cpu, 0x73, vec![]);
        assert_eq!(cpu.memory[0x1220], 0xCC);
        assert_eq!(cpu.pc, 4);

        // LD (HL), H
        cpu.h = 0xDD;
        execute_instruction_until_done(&mut cpu, 0x74, vec![]);
        assert_eq!(cpu.memory[0xDD20], 0xDD);
        assert_eq!(cpu.pc, 5);

        // LD (HL), L
        cpu.l = 0xEE;
        execute_instruction_until_done(&mut cpu, 0x75, vec![]);
        assert_eq!(cpu.memory[0xDDEE], 0xEE);
        assert_eq!(cpu.pc, 6);
    }

    // --- LD Reg from (HL) ---

    #[test]
    fn test_ld_reg_from_hl() {
        let mut cpu = setup_cpu();
        cpu.h = 0x10;
        cpu.l = 0x20;
        cpu.memory[0x1020] = 0x33;
        cpu.memory[0x3320] = 0x44;

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
    }

    // --- 16-bit Load Immediates ---

    #[test]
    fn test_ld_16bit_immediates() {
        let mut cpu = setup_cpu();
        // LD BC, nn
        cpu.memory[0] = 0x01;
        cpu.memory[1] = 0x10;
        cpu.memory[2] = 0x20;
        execute_instruction_until_done(&mut cpu, 0x01, vec![0x10, 0x20]);
        assert_eq!(cpu.b, 0x20);
        assert_eq!(cpu.c, 0x10);
        assert_eq!(cpu.pc, 3);

        // LD DE, nn
        cpu.memory[3] = 0x11;
        cpu.memory[4] = 0x30;
        cpu.memory[5] = 0x40;
        execute_instruction_until_done(&mut cpu, 0x11, vec![0x30, 0x40]);
        assert_eq!(cpu.d, 0x40);
        assert_eq!(cpu.e, 0x30);
        assert_eq!(cpu.pc, 6);

        // LD HL, nn
        cpu.memory[6] = 0x21;
        cpu.memory[7] = 0x50;
        cpu.memory[8] = 0x60;
        execute_instruction_until_done(&mut cpu, 0x21, vec![0x50, 0x60]);
        assert_eq!(cpu.h, 0x60);
        assert_eq!(cpu.l, 0x50);
        assert_eq!(cpu.pc, 9);

        // LD SP, nn
        cpu.memory[9] = 0x31;
        cpu.memory[10] = 0x70;
        cpu.memory[11] = 0x80;
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
    fn test_arithmetic_instructions() {
        let mut cpu = setup_cpu();
        // ADD A, A
        cpu.a = 0x05;
        execute_instruction_until_done(&mut cpu, 0x87, vec![]);
        assert_eq!(cpu.a, 0x0A);
        assert_eq!(cpu.f & 0x80, 0x00);
        assert_eq!(cpu.f & 0x10, 0x00);
        assert_eq!(cpu.f & 0x20, 0x00);

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
    fn test_jump_instructions() {
        let mut cpu = setup_cpu();
        // JP nn
        cpu.memory[0] = 0xC3;
        cpu.memory[1] = 0x20;
        cpu.memory[2] = 0x10;
        execute_instruction_until_done(&mut cpu, 0xC3, vec![0x20, 0x10]);
        assert_eq!(cpu.pc, 0x1020);
    }
}
