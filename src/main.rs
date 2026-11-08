mod gameboy;
use gameboy::cpu::CPU;

fn main() {
    let mut cpu = CPU::new();

    // Test: LD A, 0x42 → ADD A, A → JP 0x1000
    cpu.memory[0] = 0x3E; // LD A, n
    cpu.memory[1] = 0x42;
    cpu.memory[2] = 0x87; // ADD A, A
    cpu.memory[3] = 0xC3; // JP nn
    cpu.memory[4] = 0x00; // nn = 0x1000
    cpu.memory[5] = 0x10;

    // Run 1 cycle (LD A, 0x42)
    cpu.cycle();
    assert_eq!(cpu.a, 0x42);

    // Run 2 cycles (ADD A, A → A=0x84)
    cpu.cycle();
    assert_eq!(cpu.a, 0x84);
    // Test flags after ADD A, A
    assert_eq!(cpu.f & 0x80, 0); // Zero Flag not set
    assert_eq!(cpu.f & 0x20, 0); // Half Carry Flag not set
    assert_eq!(cpu.f & 0x10, 0); // Carry Flag not set

    // Run 3 cycles (JP 0x1000)
    cpu.cycle();
    assert_eq!(cpu.pc, 0x1000); // Jumped to 0x1000

    // Test: NOP
    cpu.pc = 0;
    cpu.memory[0] = 0x00; // NOP
    cpu.cycle();
    assert_eq!(cpu.a, 0x84); // A should retain value from previous ADD
    assert_eq!(cpu.b, 0);   // No changes to registers
    assert_eq!(cpu.c, 0);
    println!("Test NOP: Passed");

    // Test: LD B, 0x12
    cpu.pc = 0;
    cpu.memory[0] = 0x08; // LD B, n
    cpu.memory[1] = 0x12;
    cpu.cycle();
    assert_eq!(cpu.b, 0x12); // B register should be 0x12
    assert_eq!(cpu.pc, 2);   // PC should increment by 2
    println!("Test LD B, 0x12: Passed");

    // Test: LD C, 0x34
    cpu.pc = 0;
    cpu.memory[0] = 0x09; // LD C, n
    cpu.memory[1] = 0x34;
    cpu.cycle();
    assert_eq!(cpu.c, 0x34); // C register should be 0x34
    assert_eq!(cpu.pc, 2);   // PC should increment by 2
    println!("Test LD C, 0x34: Passed");

    // Test: LD D, 0x56
    cpu.pc = 0;
    cpu.memory[0] = 0x12; // LD D, n
    cpu.memory[1] = 0x56;
    cpu.cycle();
    assert_eq!(cpu.d, 0x56); // D register should be 0x56
    assert_eq!(cpu.pc, 2);   // PC should increment by 2
    println!("Test LD D, 0x56: Passed");

    // Test: LD E, 0x78
    cpu.pc = 0;
    cpu.memory[0] = 0x13; // LD E, n
    cpu.memory[1] = 0x78;
    cpu.cycle();
    assert_eq!(cpu.e, 0x78); // E register should be 0x78
    assert_eq!(cpu.pc, 2);   // PC should increment by 2
    println!("Test LD E, 0x78: Passed");

    // Test: LD H, 0x9A
    cpu.pc = 0;
    cpu.memory[0] = 0x26; // LD H, n
    cpu.memory[1] = 0x9A;
    cpu.cycle();
    assert_eq!(cpu.h, 0x9A); // H register should be 0x9A
    assert_eq!(cpu.pc, 2);   // PC should increment by 2
    println!("Test LD H, 0x9A: Passed");

    // Test: LD L, 0xBC
    cpu.pc = 0;
    cpu.memory[0] = 0x2E; // LD L, n
    cpu.memory[1] = 0xBC;
    cpu.cycle();
    assert_eq!(cpu.l, 0xBC); // L register should be 0xBC
    assert_eq!(cpu.pc, 2);   // PC should increment by 2
    println!("Test LD L, 0xBC: Passed");

    // Test: LD A, B (B = 0x12)
    cpu.pc = 0;
    cpu.memory[0] = 0x78; // LD A, B
    cpu.b = 0x12;
    cpu.cycle();
    assert_eq!(cpu.a, 0x12); // A should be equal to B
    assert_eq!(cpu.pc, 1);   // PC should increment by 1
    println!("Test LD A, B: Passed");

    // Test: LD A, C (C = 0x34)
    cpu.pc = 0;
    cpu.memory[0] = 0x79; // LD A, C
    cpu.c = 0x34;
    cpu.cycle();
    assert_eq!(cpu.a, 0x34); // A should be equal to C
    assert_eq!(cpu.pc, 1);   // PC should increment by 1
    println!("Test LD A, C: Passed");

    // Test: LD A, D (D = 0x56)
    cpu.pc = 0;
    cpu.memory[0] = 0x7A; // LD A, D
    cpu.d = 0x56;
    cpu.cycle();
    assert_eq!(cpu.a, 0x56); // A should be equal to D
    assert_eq!(cpu.pc, 1);   // PC should increment by 1
    println!("Test LD A, D: Passed");

    // Test: LD A, E (E = 0x78)
    cpu.pc = 0;
    cpu.memory[0] = 0x7B; // LD A, E
    cpu.e = 0x78;
    cpu.cycle();
    assert_eq!(cpu.a, 0x78); // A should be equal to E
    assert_eq!(cpu.pc, 1);   // PC should increment by 1
    println!("Test LD A, E: Passed");

    // Test: LD A, H (H = 0x9A)
    cpu.pc = 0;
    cpu.memory[0] = 0x7C; // LD A, H
    cpu.h = 0x9A;
    cpu.cycle();
    assert_eq!(cpu.a, 0x9A); // A should be equal to H
    assert_eq!(cpu.pc, 1);   // PC should increment by 1
    println!("Test LD A, H: Passed");

    // Test: LD A, L (L = 0xBC)
    cpu.pc = 0;
    cpu.memory[0] = 0x7D; // LD A, L
    cpu.l = 0xBC;
    cpu.cycle();
    assert_eq!(cpu.a, 0xBC); // A should be equal to L
    assert_eq!(cpu.pc, 1);   // PC should increment by 1
    println!("Test LD A, L: Passed");

    // Test: LD BC, 0x1234
    cpu.pc = 0;
    cpu.memory[0] = 0x01; // LD BC, nn
    cpu.memory[1] = 0x12; // high byte
    cpu.memory[2] = 0x34; // low byte
    cpu.cycle();
    assert_eq!(cpu.b, 0x12); // B register should be 0x12
    assert_eq!(cpu.c, 0x34); // C register should be 0x34
    assert_eq!(cpu.pc, 3);   // PC should increment by 2
    println!("Test LD BC, 0x1234: Passed");

    // Test: LD DE, 0x5678
    cpu.pc = 0;
    cpu.memory[0] = 0x11; // LD DE, nn
    cpu.memory[1] = 0x56; // high byte
    cpu.memory[2] = 0x78; // low byte
    cpu.cycle();
    assert_eq!(cpu.d, 0x56); // D register should be 0x56
    assert_eq!(cpu.e, 0x78); // E register should be 0x78
    assert_eq!(cpu.pc, 3);   // PC should increment by 2
    println!("Test LD DE, 0x5678: Passed");

    // Test: LD HL, 0x9ABC
    cpu.pc = 0;
    cpu.memory[0] = 0x21; // LD HL, nn
    cpu.memory[1] = 0x9A; // high byte
    cpu.memory[2] = 0xBC; // low byte
    cpu.cycle();
    assert_eq!(cpu.h, 0x9A); // H register should be 0x9A
    assert_eq!(cpu.l, 0xBC); // L register should be 0xBC
    assert_eq!(cpu.pc, 3);   // PC should increment by 2
    println!("Test LD HL, 0x9ABC: Passed");

    // Test: LD SP, 0x3456
    cpu.pc = 0;
    cpu.memory[0] = 0x31; // LD SP, nn
    cpu.memory[1] = 0x34; // high byte
    cpu.memory[2] = 0x56; // low byte
    cpu.cycle();
    assert_eq!(cpu.sp, 0x3456); // SP should be 0x3456
    assert_eq!(cpu.pc, 3);      // PC should increment by 2
    println!("Test LD SP, 0x3456: Passed");

    println!("All tests passed!");
}