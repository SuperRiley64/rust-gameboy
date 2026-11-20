mod gameboy;
mod tests;
use gameboy::cpu::CPU;

fn main () {
    println!("CPU under construction.");
    let mut cpu = CPU::new();

    //
    // Initialize the CPU to post-boot state (No boot rom required)
    //

    cpu.a = 0x01;
    cpu.f = 0xb0;
    cpu.b = 0x00;
    cpu.c = 0x13;
    cpu.d = 0x00;
    cpu.e = 0xD8;
    cpu.h = 0x01;
    cpu.l = 0x4D;
    cpu.sp = 0xFFFE;
    cpu.pc = 0x0100;
}