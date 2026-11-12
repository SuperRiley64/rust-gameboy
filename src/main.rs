mod gameboy;
use gameboy::cpu::CPU;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stop() {
        let mut cpu = CPU::new();
        cpu.memory[0x00] = 0x10; // STOP
        cpu.pc = 0x00;
        cpu.cycle();
        assert_eq!(cpu.pc, 0x02);
    }

    #[test]
    fn test_halt() {
        let mut cpu = CPU::new();
        cpu.memory[0x00] = 0x76; // HALT
        cpu.pc = 0x00;
        cpu.cycle();
        assert_eq!(cpu.pc, 0x01);
    }

    #[test]
    fn test_ld_memory_at_bc_from_a() {
        let mut cpu = CPU::new();
        cpu.memory[0x00] = 0x02; // LD (BC), A
        cpu.memory[0x01] = 0x00;
        cpu.memory[0x02] = 0x00;
        cpu.a = 0x56;
        cpu.b = 0x12;
        cpu.c = 0x34;
        cpu.pc = 0x00;
        cpu.cycle();
        assert_eq!(cpu.memory[0x1234 as usize], 0x56);
        assert_eq!(cpu.pc, 0x01);
    }

    #[test]
    fn test_ld_memory_at_de_from_a() {
        let mut cpu = CPU::new();
        cpu.memory[0x00] = 0x12; // LD (DE), A
        cpu.memory[0x01] = 0x00;
        cpu.memory[0x02] = 0x00;
        cpu.a = 0x56;
        cpu.d = 0x12;
        cpu.e = 0x34;
        cpu.pc = 0x00;
        cpu.cycle();
        assert_eq!(cpu.memory[0x1234 as usize], 0x56);
        assert_eq!(cpu.pc, 0x01);
    }

    #[test]
    fn test_ld_memory_at_hl_minus_from_a() {
        let mut cpu = CPU::new();
        cpu.memory[0x00] = 0x32; // LD (HL-), A
        cpu.memory[0x01] = 0x00;
        cpu.memory[0x02] = 0x00;
        cpu.a = 0x56;
        cpu.h = 0x12;
        cpu.l = 0x34;
        cpu.pc = 0x00;
        cpu.cycle();
        assert_eq!(cpu.memory[0x1233 as usize], 0x56);
        assert_eq!(cpu.h, 0x12);
        assert_eq!(cpu.l, 0x33);
        assert_eq!(cpu.pc, 0x01);
    }

    #[test]
    fn test_ld_memory_at_c_from_a() {
        let mut cpu = CPU::new();
        cpu.memory[0x00] = 0xE2; // LD (C), A
        cpu.memory[0x01] = 0x00;
        cpu.memory[0x02] = 0x00;
        cpu.a = 0x56;
        cpu.c = 0x12;
        cpu.pc = 0x00;
        cpu.cycle();
        assert_eq!(cpu.memory[0x12 as usize], 0x56);
        assert_eq!(cpu.pc, 0x01);
    }

    #[test]
    fn test_ld_memory_at_hl_plus_from_a() {
        let mut cpu = CPU::new();
        cpu.memory[0x00] = 0x22; // LD (HL+), A
        cpu.memory[0x01] = 0x00;
        cpu.memory[0x02] = 0x00;
        cpu.a = 0x56;
        cpu.h = 0x12;
        cpu.l = 0x34;
        cpu.pc = 0x00;
        cpu.cycle();
        assert_eq!(cpu.memory[0x1235 as usize], 0x56);
        assert_eq!(cpu.h, 0x12);
        assert_eq!(cpu.l, 0x35);
        assert_eq!(cpu.pc, 0x01);
    }

    #[test]
    fn test_ld_memory_at_hl_b() {
        let mut cpu = CPU::new();
        cpu.memory[0x00] = 0x70; // LD (HL), B
        cpu.memory[0x01] = 0x00;
        cpu.memory[0x02] = 0x00;
        cpu.h = 0x12;
        cpu.l = 0x34;
        cpu.b = 0x56;
        cpu.pc = 0x00;
        cpu.cycle();
        assert_eq!(cpu.memory[0x1234 as usize], 0x56);
        assert_eq!(cpu.pc, 0x01);
    }

    #[test]
    fn test_ld_memory_at_hl_c() {
        let mut cpu = CPU::new();
        cpu.memory[0x00] = 0x71; // LD (HL), C
        cpu.memory[0x01] = 0x00;
        cpu.memory[0x02] = 0x00;
        cpu.h = 0x12;
        cpu.l = 0x34;
        cpu.c = 0x56;
        cpu.pc = 0x00;
        cpu.cycle();
        assert_eq!(cpu.memory[0x1234 as usize], 0x56);
        assert_eq!(cpu.pc, 0x01);
    }

    #[test]
    fn test_ld_memory_at_hl_d() {
        let mut cpu = CPU::new();
        cpu.memory[0x00] = 0x72; // LD (HL), D
        cpu.memory[0x01] = 0x00;
        cpu.memory[0x02] = 0x00;
        cpu.h = 0x12;
        cpu.l = 0x34;
        cpu.d = 0x56;
        cpu.pc = 0x00;
        cpu.cycle();
        assert_eq!(cpu.memory[0x1234 as usize], 0x56);
        assert_eq!(cpu.pc, 0x01);
    }

    #[test]
    fn test_ld_memory_at_hl_e() {
        let mut cpu = CPU::new();
        cpu.memory[0x00] = 0x73; // LD (HL), E
        cpu.memory[0x01] = 0x00;
        cpu.memory[0x02] = 0x00;
        cpu.h = 0x12;
        cpu.l = 0x34;
        cpu.e = 0x56;
        cpu.pc = 0x00;
        cpu.cycle();
        assert_eq!(cpu.memory[0x1234 as usize], 0x56);
        assert_eq!(cpu.pc, 0x01);
    }

    #[test]
    fn test_ld_memory_at_hl_h() {
        let mut cpu = CPU::new();
        cpu.memory[0x00] = 0x74; // LD (HL), H
        cpu.memory[0x01] = 0x00;
        cpu.memory[0x02] = 0x00;
        cpu.h = 0x12;
        cpu.l = 0x34;
        cpu.h = 0x56;
        cpu.pc = 0x00;
        cpu.cycle();
        assert_eq!(cpu.memory[0x1234 as usize], 0x56);
        assert_eq!(cpu.pc, 0x01);
    }

    #[test]
    fn test_ld_memory_at_hl_l() {
        let mut cpu = CPU::new();
        cpu.memory[0x00] = 0x75; // LD (HL), L
        cpu.memory[0x01] = 0x00;
        cpu.memory[0x02] = 0x00;
        cpu.h = 0x12;
        cpu.l = 0x34;
        cpu.l = 0x56;
        cpu.pc = 0x00;
        cpu.cycle();
        assert_eq!(cpu.memory[0x1234 as usize], 0x56);
        assert_eq!(cpu.pc, 0x01);
    }

    #[test]
    fn test_ld_a_from_hl() {
        let mut cpu = CPU::new();
        cpu.memory[0x00] = 0x7E; // LD A, (HL)
        cpu.memory[0x01] = 0x00;
        cpu.memory[0x02] = 0x00;
        cpu.h = 0x12;
        cpu.l = 0x34;
        cpu.memory[0x1234 as usize] = 0x56;
        cpu.pc = 0x00;
        cpu.cycle();
        assert_eq!(cpu.a, 0x56);
        assert_eq!(cpu.pc, 0x01);
    }

    #[test]
    fn test_ld_b_from_hl() {
        let mut cpu = CPU::new();
        cpu.memory[0x00] = 0x46; // LD B, (HL)
        cpu.memory[0x01] = 0x00;
        cpu.memory[0x02] = 0x00;
        cpu.h = 0x12;
        cpu.l = 0x34;
        cpu.memory[0x1234 as usize] = 0x56;
        cpu.pc = 0x00;
        cpu.cycle();
        assert_eq!(cpu.b, 0x56);
        assert_eq!(cpu.pc, 0x01);
    }

    #[test]
    fn test_ld_d_from_hl() {
        let mut cpu = CPU::new();
        cpu.memory[0x00] = 0x56; // LD D, (HL)
        cpu.memory[0x01] = 0x00;
        cpu.memory[0x02] = 0x00;
        cpu.h = 0x12;
        cpu.l = 0x34;
        cpu.memory[0x1234 as usize] = 0x56;
        cpu.pc = 0x00;
        cpu.cycle();
        assert_eq!(cpu.d, 0x56);
        assert_eq!(cpu.pc, 0x01);
    }

    #[test]
    fn test_ld_e_from_hl() {
        let mut cpu = CPU::new();
        cpu.memory[0x00] = 0x5E; // LD E, (HL)
        cpu.memory[0x01] = 0x00;
        cpu.memory[0x02] = 0x00;
        cpu.h = 0x12;
        cpu.l = 0x34;
        cpu.memory[0x1234 as usize] = 0x56;
        cpu.pc = 0x00;
        cpu.cycle();
        assert_eq!(cpu.e, 0x56);
        assert_eq!(cpu.pc, 0x01);
    }

    #[test]
    fn test_ld_h_from_hl() {
        let mut cpu = CPU::new();
        cpu.memory[0x00] = 0x66; // LD H, (HL)
        cpu.memory[0x01] = 0x00;
        cpu.memory[0x02] = 0x00;
        cpu.h = 0x12;
        cpu.l = 0x34;
        cpu.memory[0x1234 as usize] = 0x56;
        cpu.pc = 0x00;
        cpu.cycle();
        assert_eq!(cpu.h, 0x56);
        assert_eq!(cpu.pc, 0x01);
    }

    #[test]
    fn test_ld_l_from_hl() {
        let mut cpu = CPU::new();
        cpu.memory[0x00] = 0x6E; // LD L, (HL)
        cpu.memory[0x01] = 0x00;
        cpu.memory[0x02] = 0x00;
        cpu.h = 0x12;
        cpu.l = 0x34;
        cpu.memory[0x1234 as usize] = 0x56;
        cpu.pc = 0x00;
        cpu.cycle();
        assert_eq!(cpu.l, 0x56);
        assert_eq!(cpu.pc, 0x01);
    }

    #[test]
    fn test_ld_a_a() {
        let mut cpu = CPU::new();
        cpu.memory[0x00] = 0x7F; // LD A, A
        cpu.a = 0x56;
        cpu.pc = 0x00;
        cpu.cycle();
        assert_eq!(cpu.a, 0x56);
        assert_eq!(cpu.pc, 0x01);
    }

    #[test]
    fn test_ld_d_n() {
        let mut cpu = CPU::new();
        cpu.memory[0x00] = 0x16; // LD D, n
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
    fn test_ld_a_from_c() {
        let mut cpu = CPU::new();
        cpu.memory[0x00] = 0xF2; // LD A, (C)
        cpu.memory[0x01] = 0x00;
        cpu.memory[0x02] = 0x00;
        cpu.c = 0x12;
        cpu.memory[0xFF12 as usize] = 0x34;
        cpu.pc = 0x00;
        cpu.cycle();
        assert_eq!(cpu.a, 0x34);
        assert_eq!(cpu.pc, 0x02);
    }

    #[test]
    fn test_ld_a_from_bc() {
        let mut cpu = CPU::new();
        cpu.memory[0x00] = 0x0A; // LD A, (BC)
        cpu.memory[0x01] = 0x00;
        cpu.memory[0x02] = 0x00;
        cpu.b = 0x12;
        cpu.c = 0x34;
        cpu.memory[0x1234 as usize] = 0x56;
        cpu.pc = 0x00;
        cpu.cycle();
        assert_eq!(cpu.a, 0x56);
        assert_eq!(cpu.pc, 0x02);
    }

    #[test]
    fn test_ld_a_from_de() {
        let mut cpu = CPU::new();
        cpu.memory[0x00] = 0x1A; // LD A, (DE)
        cpu.memory[0x01] = 0x00;
        cpu.memory[0x02] = 0x00;
        cpu.d = 0x12;
        cpu.e = 0x34;
        cpu.memory[0x1234 as usize] = 0x56;
        cpu.pc = 0x00;
        cpu.cycle();
        assert_eq!(cpu.a, 0x56);
        assert_eq!(cpu.pc, 0x03);
    }

    #[test]
    fn test_ld_a_from_hl_minus() {
        let mut cpu = CPU::new();
        cpu.memory[0x00] = 0x3A; // LD A, (HL-)
        cpu.memory[0x01] = 0x00;
        cpu.memory[0x02] = 0x00;
        cpu.h = 0x12;
        cpu.l = 0x34;
        cpu.memory[0x1233 as usize] = 0x56;
        cpu.pc = 0x00;
        cpu.cycle();
        assert_eq!(cpu.a, 0x56);
        assert_eq!(cpu.h, 0x12);
        assert_eq!(cpu.l, 0x33);
        assert_eq!(cpu.pc, 0x03);
    }

    #[test]
    fn test_ld_a_from_hl_plus() {
        let mut cpu = CPU::new();
        cpu.memory[0x00] = 0x2A; // LD A, (HL+)
        cpu.memory[0x01] = 0x00;
        cpu.memory[0x02] = 0x00;
        cpu.h = 0x12;
        cpu.l = 0x34;
        cpu.memory[0x1235 as usize] = 0x56;
        cpu.pc = 0x00;
        cpu.cycle();
        assert_eq!(cpu.a, 0x56);
        assert_eq!(cpu.h, 0x12);
        assert_eq!(cpu.l, 0x35);
        assert_eq!(cpu.pc, 0x03);
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
        assert_eq!(cpu.f, 0x60);
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
        assert_eq!(cpu.f, 0x60);
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
        assert_eq!(cpu.f, 0x60);
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
        assert_eq!(cpu.f, 0x60);
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
        assert_eq!(cpu.f, 0x60);
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
        assert_eq!(cpu.f, 0x60);
    }

    #[test]
    fn test_inc_a() {
        let mut cpu = CPU::new();
        cpu.a = 0x0F;
        cpu.memory[0x00] = 0x3C; // INC A
        cpu.pc = 0x00;
        cpu.cycle();
        assert_eq!(cpu.a, 0x10);
        assert_eq!(cpu.f, 0x20);
    }

    #[test]
    fn test_dec_a() {
        let mut cpu = CPU::new();
        cpu.a = 0x10;
        cpu.memory[0x00] = 0x3D; // DEC A
        cpu.pc = 0x00;
        cpu.cycle();
        assert_eq!(cpu.a, 0x0F);
        assert_eq!(cpu.f, 0x60);
    }

    #[test]
    fn test_inc_bc() {
        let mut cpu = CPU::new();
        cpu.b = 0x01;
        cpu.c = 0x00;
        cpu.f = 0x00;
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
        cpu.f = 0x00;
        cpu.memory[0x00] = 0x0B; // DEC BC
        cpu.pc = 0x00;
        cpu.cycle();
        assert_eq!(cpu.b, 0x01);
        assert_eq!(cpu.c, 0x00);
        assert_eq!(cpu.f, 0x00);
    }

    #[test]
    fn test_inc_de() {
        let mut cpu = CPU::new();
        cpu.d = 0x01;
        cpu.e = 0x00;
        cpu.f = 0x00;
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
        cpu.f = 0x00;
        cpu.memory[0x00] = 0x1B; // DEC DE
        cpu.pc = 0x00;
        cpu.cycle();
        assert_eq!(cpu.d, 0x01);
        assert_eq!(cpu.e, 0x00);
        assert_eq!(cpu.f, 0x00);
    }

    #[test]
    fn test_inc_hl() {
        let mut cpu = CPU::new();
        cpu.h = 0x01;
        cpu.l = 0x00;
        cpu.f = 0x00;
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
        cpu.f = 0x00;
        cpu.memory[0x00] = 0x2B; // DEC HL
        cpu.pc = 0x00;
        cpu.cycle();
        assert_eq!(cpu.h, 0x01);
        assert_eq!(cpu.l, 0x00);
        assert_eq!(cpu.f, 0x00);
    }

    #[test]
    fn test_inc_sp() {
        let mut cpu = CPU::new();
        cpu.sp = 0x1234;
        cpu.f = 0x00;
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
        cpu.f = 0x00;
        cpu.memory[0x00] = 0x3B; // DEC SP
        cpu.pc = 0x00;
        cpu.cycle();
        assert_eq!(cpu.sp, 0x1234);
        assert_eq!(cpu.f, 0x00);
    }

    #[test]
    fn test_unimplemented_opcode() {
        let mut cpu = CPU::new();
        cpu.memory[0x00] = 0xFF; // Unimplemented opcode
        cpu.pc = 0x00;
        cpu.cycle();
        assert_eq!(cpu.pc, 0x01);
    }

    #[test]
    fn test_ld_a_from_a16() {
        let mut cpu = CPU::new();
        cpu.memory[0x00] = 0xFA; // LD A, (a16)
        cpu.memory[0x01] = 0x12;
        cpu.memory[0x02] = 0x34;
        cpu.memory[0x3412 as usize] = 0x56;
        cpu.pc = 0x00;
        cpu.cycle();
        assert_eq!(cpu.a, 0x56);
        assert_eq!(cpu.pc, 0x03);
    }

    #[test]
    fn test_ld_a_from_a8() {
        let mut cpu = CPU::new();
        cpu.memory[0x00] = 0xF0; // LD A, (a8)
        cpu.memory[0x01] = 0x78;
        cpu.memory[0x78 as usize] = 0x9A;
        cpu.pc = 0x00;
        cpu.cycle();
        assert_eq!(cpu.a, 0x9A);
        assert_eq!(cpu.pc, 0x02);
    }

    #[test]
    fn test_ld_memory_at_a16_from_a() {
        let mut cpu = CPU::new();
        cpu.memory[0x00] = 0xEA; // LD (a16), A
        cpu.memory[0x01] = 0x12;
        cpu.memory[0x02] = 0x34;
        cpu.a = 0x56;
        cpu.pc = 0x00;
        cpu.cycle();
        assert_eq!(cpu.memory[0x3412 as usize], 0x56);
        assert_eq!(cpu.pc, 0x03);
    }

    #[test]
    fn test_ld_memory_at_a8_from_a() {
        let mut cpu = CPU::new();
        cpu.memory[0x00] = 0xE0; // LD (a8), A
        cpu.memory[0x01] = 0x78;
        cpu.a = 0x9A;
        cpu.pc = 0x00;
        cpu.cycle();
        assert_eq!(cpu.memory[0x78 as usize], 0x9A);
        assert_eq!(cpu.pc, 0x02);
    }

    #[test]
    fn test_ld_memory_at_hl_from_d8() {
        let mut cpu = CPU::new();
        cpu.memory[0x00] = 0x36; // LD (HL), d8
        cpu.memory[0x01] = 0x56;
        cpu.h = 0x12;
        cpu.l = 0x34;
        cpu.pc = 0x00;
        cpu.cycle();
        assert_eq!(cpu.memory[0x1234 as usize], 0x56);
        assert_eq!(cpu.pc, 0x02);
    }

    #[test]
    fn test_ld_a_d() {
        let mut cpu = CPU::new();
        cpu.d = 0x56;
        cpu.memory[0x00] = 0x7A; // LD A, D
        cpu.pc = 0x00;
        cpu.cycle();
        assert_eq!(cpu.a, 0x56);
        assert_eq!(cpu.pc, 0x01);
    }

    #[test]
    fn test_ld_a_e() {
        let mut cpu = CPU::new();
        cpu.e = 0x78;
        cpu.memory[0x00] = 0x7B; // LD A, E
        cpu.pc = 0x00;
        cpu.cycle();
        assert_eq!(cpu.a, 0x78);
        assert_eq!(cpu.pc, 0x01);
    }

    #[test]
    fn test_ld_a_h() {
        let mut cpu = CPU::new();
        cpu.h = 0x9A;
        cpu.memory[0x00] = 0x7C; // LD A, H
        cpu.pc = 0x00;
        cpu.cycle();
        assert_eq!(cpu.a, 0x9A);
        assert_eq!(cpu.pc, 0x01);
    }

    #[test]
    fn test_ld_a_l() {
        let mut cpu = CPU::new();
        cpu.l = 0xBC;
        cpu.memory[0x00] = 0x7D; // LD A, L
        cpu.pc = 0x00;
        cpu.cycle();
        assert_eq!(cpu.a, 0xBC);
        assert_eq!(cpu.pc, 0x01);
    }

    #[test]
    fn test_ld_b_a() {
        let mut cpu = CPU::new();
        cpu.a = 0xDE;
        cpu.memory[0x00] = 0x47; // LD B, A
        cpu.pc = 0x00;
        cpu.cycle();
        assert_eq!(cpu.b, 0xDE);
        assert_eq!(cpu.pc, 0x01);
    }

    #[test]
    fn test_ld_c_a() {
        let mut cpu = CPU::new();
        cpu.a = 0xF0;
        cpu.memory[0x00] = 0x4F; // LD C, A
        cpu.pc = 0x00;
        cpu.cycle();
        assert_eq!(cpu.c, 0xF0);
        assert_eq!(cpu.pc, 0x01);
    }

    #[test]
    fn test_ld_b_c() {
        let mut cpu = CPU::new();
        cpu.c = 0x12;
        cpu.memory[0x00] = 0x48; // LD B, C
        cpu.pc = 0x00;
        cpu.cycle();
        assert_eq!(cpu.b, 0x12);
        assert_eq!(cpu.pc, 0x01);
    }

    #[test]
    fn test_ld_c_b() {
        let mut cpu = CPU::new();
        cpu.b = 0x34;
        cpu.memory[0x00] = 0x41; // LD C, B
        cpu.pc = 0x00;
        cpu.cycle();
        assert_eq!(cpu.c, 0x34);
        assert_eq!(cpu.pc, 0x01);
    }

    #[test]
    fn test_ld_d_c() {
        let mut cpu = CPU::new();
        cpu.c = 0x56;
        cpu.memory[0x00] = 0x4A; // LD D, C
        cpu.pc = 0x00;
        cpu.cycle();
        assert_eq!(cpu.d, 0x56);
        assert_eq!(cpu.pc, 0x01);
    }

    #[test]
    fn test_ld_e_c() {
        let mut cpu = CPU::new();
        cpu.c = 0x78;
        cpu.memory[0x00] = 0x4B; // LD E, C
        cpu.pc = 0x00;
        cpu.cycle();
        assert_eq!(cpu.e, 0x78);
        assert_eq!(cpu.pc, 0x01);
    }

    #[test]
    fn test_ld_h_c() {
        let mut cpu = CPU::new();
        cpu.c = 0x9A;
        cpu.memory[0x00] = 0x4C; // LD H, C
        cpu.pc = 0x00;
        cpu.cycle();
        assert_eq!(cpu.h, 0x9A);
        assert_eq!(cpu.pc, 0x01);
    }

    #[test]
    fn test_ld_d_a() {
        let mut cpu = CPU::new();
        cpu.a = 0x66;
        cpu.memory[0x00] = 0x79; // LD D, A
        cpu.pc = 0x00;
        cpu.cycle();
        assert_eq!(cpu.d, 0x66);
        assert_eq!(cpu.pc, 0x01);
    }

    #[test]
    fn test_ld_e_a() {
        let mut cpu = CPU::new();
        cpu.a = 0x88;
        cpu.memory[0x00] = 0x78; // LD E, A
        cpu.pc = 0x00;
        cpu.cycle();
        assert_eq!(cpu.e, 0x88);
        assert_eq!(cpu.pc, 0x01);
    }
}