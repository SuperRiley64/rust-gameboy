mod gameboy;
use gameboy::cpu::CPU;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inc_bc() {
        let mut cpu = CPU::new();
        cpu.b = 0x01;
        cpu.c = 0x00;
        cpu.pc = 0x00;
        cpu.memory[0x00] = 0x03; // INC BC

        cpu.cycle();

        assert_eq!(cpu.b, 0x01);
        assert_eq!(cpu.c, 0x01);
        assert_eq!(cpu.f, 0x00); // No Zero flag
    }

    #[test]
    fn test_dec_bc() {
        let mut cpu = CPU::new();
        cpu.b = 0x00;
        cpu.c = 0x01;
        cpu.pc = 0x00;
        cpu.memory[0x00] = 0x0B; // DEC BC

        cpu.cycle();

        assert_eq!(cpu.b, 0x00);
        assert_eq!(cpu.c, 0x00);
        assert_eq!(cpu.f, 0xC0); // Subtract and Zero flags
    }

    #[test]
    fn test_inc_de() {
        let mut cpu = CPU::new();
        cpu.d = 0x02;
        cpu.e = 0x00;
        cpu.pc = 0x00;
        cpu.memory[0x00] = 0x13; // INC DE

        cpu.cycle();

        assert_eq!(cpu.d, 0x02);
        assert_eq!(cpu.e, 0x01);
        assert_eq!(cpu.f, 0x00); // No Zero flag
    }

    #[test]
    fn test_dec_de() {
        let mut cpu = CPU::new();
        cpu.d = 0x00;
        cpu.e = 0x01;
        cpu.pc = 0x00;
        cpu.memory[0x00] = 0x1B; // DEC DE

        cpu.cycle();

        assert_eq!(cpu.d, 0x00);
        assert_eq!(cpu.e, 0x00);
        assert_eq!(cpu.f, 0xC0); // Subtract and Zero flags
    }

    #[test]
    fn test_inc_hl() {
        let mut cpu = CPU::new();
        cpu.h = 0x03;
        cpu.l = 0x00;
        cpu.pc = 0x00;
        cpu.memory[0x00] = 0x23; // INC HL

        cpu.cycle();

        assert_eq!(cpu.h, 0x03);
        assert_eq!(cpu.l, 0x01);
        assert_eq!(cpu.f, 0x00); // No Zero flag
    }

    #[test]
    fn test_dec_hl() {
        let mut cpu = CPU::new();
        cpu.h = 0x00;
        cpu.l = 0x01;
        cpu.pc = 0x00;
        cpu.memory[0x00] = 0x2B; // DEC HL

        cpu.cycle();

        assert_eq!(cpu.h, 0x00);
        assert_eq!(cpu.l, 0x00);
        assert_eq!(cpu.f, 0xC0); // Subtract and Zero flags
    }

    #[test]
    fn test_inc_sp() {
        let mut cpu = CPU::new();
        cpu.sp = 0x1234;
        cpu.pc = 0x00;
        cpu.memory[0x00] = 0x33; // INC SP

        cpu.cycle();

        assert_eq!(cpu.sp, 0x1235);
        assert_eq!(cpu.f, 0x00); // No Zero flag
    }

    #[test]
    fn test_dec_sp() {
        let mut cpu = CPU::new();
        cpu.sp = 0x1235;
        cpu.pc = 0x00;
        cpu.memory[0x00] = 0x3B; // DEC SP

        cpu.cycle();

        assert_eq!(cpu.sp, 0x1234);
        assert_eq!(cpu.f, 0x40); // Subtract flag (no Zero since SP != 0)
    }
}