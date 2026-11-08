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

    // Run 3 cycles (JP 0x1000)
    cpu.cycle();
    assert_eq!(cpu.pc, 0x1000); // Jumped to 0x1000

    println!("All tests passed!");
}
