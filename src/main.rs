mod gameboy;
use gameboy::cpu::CPU;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nop() {
        let mut cpu = CPU::new();
        cpu.memory[0x00] = 0x00; // NOP
        cpu.pc = 0x00;
        cpu.cycle();
        assert_eq!(cpu.pc, 0x01);
    }

    #[test]
    fn test_ld_b_n() {
        let mut cpu = CPU::new();
        cpu.memory[0x00] = 0x08; // LD B, n
        cpu.memory[0x01] = 0x12;
        cpu.pc = 0x00;
        cpu.cycle();
        assert_eq!(cpu.b, 0x12);
        assert_eq!(cpu.pc, 0x02);
    }

    #[test]
    fn test_ld_c_n() {
        let mut cpu = CPU::new();
        cpu.memory[0x00] = 0x09; // LD C, n
        cpu.memory[0x01] = 0x34;
        cpu.pc = 0x00;
        cpu.cycle();
        assert_eq!(cpu.c, 0x34);
        assert_eq!(cpu.pc, 0x02);
    }

    #[test]
    fn test_ld_d_n() {
        let mut cpu = CPU::new();
        cpu.memory[0x00] = 0x12; // LD D, n
        cpu.memory[0x01] = 0x56;
        cpu.pc = 0x00;
        cpu.cycle();
        assert_eq!(cpu.d, 0x56);
        assert_eq!(cpu.pc, 0x02);
    }

    #[test]
    fn test_ld_e_n() {
        let mut cpu = CPU::new();
        cpu.memory[0x00] = 0x1E; // LD E, n
        cpu.memory[0x01] = 0x78;
        cpu.pc = 0x00;
        cpu.cycle();
        assert_eq!(cpu.e, 0x78);
        assert_eq!(cpu.pc, 0x02);
    }

    #[test]
    fn test_ld_h_n() {
        let mut cpu = CPU::new();
        cpu.memory[0x00] = 0x26; // LD H, n
        cpu.memory[0x01] = 0x9A;
        cpu.pc = 0x00;
        cpu.cycle();
        assert_eq!(cpu.h, 0x9A);
        assert_eq!(cpu.pc, 0x02);
    }

    #[test]
    fn test_ld_l_n() {
        let mut cpu = CPU::new();
        cpu.memory[0x00] = 0x2E; // LD L, n
        cpu.memory[0x01] = 0xBC;
        cpu.pc = 0x00;
        cpu.cycle();
        assert_eq!(cpu.l, 0xBC);
        assert_eq!(cpu.pc, 0x02);
    }

    #[test]
    fn test_ld_a_n() {
        let mut cpu = CPU::new();
        cpu.memory[0x00] = 0x3E; // LD A, n
        cpu.memory[0x01] = 0xDE;
        cpu.pc = 0x00;
        cpu.cycle();
        assert_eq!(cpu.a, 0xDE);
        assert_eq!(cpu.pc, 0x02);
    }

    #[test]
    fn test_ld_a_b() {
        let mut cpu = CPU::new();
        cpu.b = 0x12;
        cpu.memory[0x00] = 0x78; // LD A, B
        cpu.pc = 0x00;
        cpu.cycle();
        assert_eq!(cpu.a, 0x12);
        assert_eq!(cpu.pc, 0x01);
    }

    #[test]
    fn test_ld_a_c() {
        let mut cpu = CPU::new();
        cpu.c = 0x34;
        cpu.memory[0x00] = 0x79; // LD A, C
        cpu.pc = 0x00;
        cpu.cycle();
        assert_eq!(cpu.a, 0x34);
        assert_eq!(cpu.pc, 0x01);
    }

    #[test]
    fn test_add_a_a() {
        let mut cpu = CPU::new();
        cpu.a = 0x01;
        cpu.memory[0x00] = 0x87; // ADD A, A
        cpu.pc = 0x00;
        cpu.cycle();
        assert_eq!(cpu.a, 0x02);
        assert_eq!(cpu.f, 0x00); // no carries or <0
    }

    #[test]
    fn test_jp_nn() {
        let mut cpu = CPU::new();
        cpu.memory[0x00] = 0xC3; // JP nn
        cpu.memory[0x01] = 0x12;
        cpu.memory[0x02] = 0x34;
        cpu.pc = 0x00;
        cpu.cycle();
        assert_eq!(cpu.pc, 0x3412);
    }

    #[test]
    fn test_ld_bc_nn() {
        let mut cpu = CPU::new();
        cpu.memory[0x00] = 0x01; // LD BC, nn
        cpu.memory[0x01] = 0x56;
        cpu.memory[0x02] = 0x78;
        cpu.pc = 0x00;
        cpu.cycle();
        assert_eq!(cpu.b, 0x78);
        assert_eq!(cpu.c, 0x56);
        assert_eq!(cpu.pc, 0x03);
    }

    #[test]
    fn test_ld_de_nn() {
        let mut cpu = CPU::new();
        cpu.memory[0x00] = 0x11; // LD DE, nn
        cpu.memory[0x01] = 0x9A;
        cpu.memory[0x02] = 0xBC;
        cpu.pc = 0x00;
        cpu.cycle();
        assert_eq!(cpu.d, 0xBC);
        assert_eq!(cpu.e, 0x9A);
        assert_eq!(cpu.pc, 0x03);
    }

    #[test]
    fn test_ld_hl_nn() {
        let mut cpu = CPU::new();
        cpu.memory[0x00] = 0x21; // LD HL, nn
        cpu.memory[0x01] = 0xDE;
        cpu.memory[0x02] = 0xAD;
        cpu.pc = 0x00;
        cpu.cycle();
        assert_eq!(cpu.h, 0xAD);
        assert_eq!(cpu.l, 0xDE);
        assert_eq!(cpu.pc, 0x03);
    }

    #[test]
    fn test_ld_sp_nn() {
        let mut cpu = CPU::new();
        cpu.memory[0x00] = 0x31; // LD SP, nn
        cpu.memory[0x01] = 0x12;
        cpu.memory[0x02] = 0x34;
        cpu.pc = 0x00;
        cpu.cycle();
        assert_eq!(cpu.sp, 0x3412);
        assert_eq!(cpu.pc, 0x03);
    }

    #[test]
    fn test_inc_b() {
        let mut cpu = CPU::new();
        cpu.b = 0x0F;
        cpu.memory[0x00] = 0x04; // INC B
        cpu.pc = 0x00;
        cpu.cycle();
        assert_eq!(cpu.b, 0x10);
        assert_eq!(cpu.f, 0x20);
    }

    #[test]
    fn test_dec_b() {
        let mut cpu = CPU::new();
        cpu.b = 0x10;
        cpu.memory[0x00] = 0x05; // DEC B
        cpu.pc = 0x00;
        cpu.cycle();
        assert_eq!(cpu.b, 0x0F);
        assert_eq!(cpu.f, 0x40);
    }

    #[test]
    fn test_inc_c() {
        let mut cpu = CPU::new();
        cpu.c = 0x0F;
        cpu.memory[0x00] = 0x0C; // INC C
        cpu.pc = 0x00;
        cpu.cycle();
        assert_eq!(cpu.c, 0x10);
        assert_eq!(cpu.f, 0x20);
    }

    #[test]
    fn test_dec_c() {
        let mut cpu = CPU::new();
        cpu.c = 0x10;
        cpu.memory[0x00] = 0x0D; // DEC C
        cpu.pc = 0x00;
        cpu.cycle();
        assert_eq!(cpu.c, 0x0F);
        assert_eq!(cpu.f, 0x40);
    }

    #[test]
    fn test_inc_d() {
        let mut cpu = CPU::new();
        cpu.d = 0x0F;
        cpu.memory[0x00] = 0x14; // INC D
        cpu.pc = 0x00;
        cpu.cycle();
        assert_eq!(cpu.d, 0x10);
        assert_eq!(cpu.f, 0x20);
    }

    #[test]
    fn test_dec_d() {
        let mut cpu = CPU::new();
        cpu.d = 0x10;
        cpu.memory[0x00] = 0x15; // DEC D
        cpu.pc = 0x00;
        cpu.cycle();
        assert_eq!(cpu.d, 0x0F);
        assert_eq!(cpu.f, 0x40);
    }

    #[test]
    fn test_inc_e() {
        let mut cpu = CPU::new();
        cpu.e = 0x0F;
        cpu.memory[0x00] = 0x1C; // INC E
        cpu.pc = 0x00;
        cpu.cycle();
        assert_eq!(cpu.e, 0x10);
        assert_eq!(cpu.f, 0x20);
    }

    #[test]
    fn test_dec_e() {
        let mut cpu = CPU::new();
        cpu.e = 0x10;
        cpu.memory[0x00] = 0x1D; // DEC E
        cpu.pc = 0x00;
        cpu.cycle();
        assert_eq!(cpu.e, 0x0F);
        assert_eq!(cpu.f, 0x40);
    }

    #[test]
    fn test_inc_h() {
        let mut cpu = CPU::new();
        cpu.h = 0x0F;
        cpu.memory[0x00] = 0x24; // INC H
        cpu.pc = 0x00;
        cpu.cycle();
        assert_eq!(cpu.h, 0x10);
        assert_eq!(cpu.f, 0x20);
    }

    #[test]
    fn test_dec_h() {
        let mut cpu = CPU::new();
        cpu.h = 0x10;
        cpu.memory[0x00] = 0x25; // DEC H
        cpu.pc = 0x00;
        cpu.cycle();
        assert_eq!(cpu.h, 0x0F);
        assert_eq!(cpu.f, 0x40);
    }

    #[test]
    fn test_inc_l() {
        let mut cpu = CPU::new();
        cpu.l = 0x0F;
        cpu.memory[0x00] = 0x2C; // INC L
        cpu.pc = 0x00;
        cpu.cycle();
        assert_eq!(cpu.l, 0x10);
        assert_eq!(cpu.f, 0x20);
    }

    #[test]
    fn test_dec_l() {
        let mut cpu = CPU::new();
        cpu.l = 0x10;
        cpu.memory[0x00] = 0x2D; // DEC L
        cpu.pc = 0x00;
        cpu.cycle();
        assert_eq!(cpu.l, 0x0F);
        assert_eq!(cpu.f, 0x40);
    }

    #[test]
    fn test_inc_a() {
        let mut cpu = CPU::new();
        cpu.a = 0x0F;
        cpu.memory[0x00] = 0x34; // INC A
        cpu.pc = 0x00;
        cpu.cycle();
        assert_eq!(cpu.a, 0x10);
        assert_eq!(cpu.f, 0x20);
    }

    #[test]
    fn test_dec_a() {
        let mut cpu = CPU::new();
        cpu.a = 0x10;
        cpu.memory[0x00] = 0x35; // DEC A
        cpu.pc = 0x00;
        cpu.cycle();
        assert_eq!(cpu.a, 0x0F);
        assert_eq!(cpu.f, 0x40);
    }

    #[test]
    fn test_inc_bc() {
        let mut cpu = CPU::new();
        cpu.b = 0x01;
        cpu.c = 0x00;
        cpu.memory[0x00] = 0x03; // INC BC
        cpu.pc = 0x00;
        cpu.cycle();
        assert_eq!(cpu.b, 0x01);
        assert_eq!(cpu.c, 0x01);
        assert_eq!(cpu.f, 0x00);
    }

    #[test]
    fn test_dec_bc() {
        let mut cpu = CPU::new();
        cpu.b = 0x01;
        cpu.c = 0x01;
        cpu.memory[0x00] = 0x0B; // DEC BC
        cpu.pc = 0x00;
        cpu.cycle();
        assert_eq!(cpu.b, 0x01);
        assert_eq!(cpu.c, 0x00);
        assert_eq!(cpu.f, 0x40);
    }

    #[test]
    fn test_inc_de() {
        let mut cpu = CPU::new();
        cpu.d = 0x01;
        cpu.e = 0x00;
        cpu.memory[0x00] = 0x13; // INC DE
        cpu.pc = 0x00;
        cpu.cycle();
        assert_eq!(cpu.d, 0x01);
        assert_eq!(cpu.e, 0x01);
        assert_eq!(cpu.f, 0x00);
    }

    #[test]
    fn test_dec_de() {
        let mut cpu = CPU::new();
        cpu.d = 0x01;
        cpu.e = 0x01;
        cpu.memory[0x00] = 0x1B; // DEC DE
        cpu.pc = 0x00;
        cpu.cycle();
        assert_eq!(cpu.d, 0x01);
        assert_eq!(cpu.e, 0x00);
        assert_eq!(cpu.f, 0x40);
    }

    #[test]
    fn test_inc_hl() {
        let mut cpu = CPU::new();
        cpu.h = 0x01;
        cpu.l = 0x00;
        cpu.memory[0x00] = 0x23; // INC HL
        cpu.pc = 0x00;
        cpu.cycle();
        assert_eq!(cpu.h, 0x01);
        assert_eq!(cpu.l, 0x01);
        assert_eq!(cpu.f, 0x00);
    }

    #[test]
    fn test_dec_hl() {
        let mut cpu = CPU::new();
        cpu.h = 0x01;
        cpu.l = 0x01;
        cpu.memory[0x00] = 0x2B; // DEC HL
        cpu.pc = 0x00;
        cpu.cycle();
        assert_eq!(cpu.h, 0x01);
        assert_eq!(cpu.l, 0x00);
        assert_eq!(cpu.f, 0x40);
    }

    #[test]
    fn test_inc_sp() {
        let mut cpu = CPU::new();
        cpu.sp = 0x1234;
        cpu.memory[0x00] = 0x33; // INC SP
        cpu.pc = 0x00;
        cpu.cycle();
        assert_eq!(cpu.sp, 0x1235);
        assert_eq!(cpu.f, 0x00);
    }

    #[test]
    fn test_dec_sp() {
        let mut cpu = CPU::new();
        cpu.sp = 0x1235;
        cpu.memory[0x00] = 0x3B; // DEC SP
        cpu.pc = 0x00;
        cpu.cycle();
        assert_eq!(cpu.sp, 0x1234);
        assert_eq!(cpu.f, 0x40);
    }

    #[test]
    fn test_unimplemented_opcode() {
        let mut cpu = CPU::new();
        cpu.memory[0x00] = 0xFF; // Unimplemented opcode
        cpu.pc = 0x00;
        cpu.cycle();
        assert_eq!(cpu.pc, 0x01);
    }
}