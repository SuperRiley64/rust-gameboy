mod gameboy;
mod tests;
use gameboy::cpu::CPU;
use std::thread;
use std::time::Duration;

fn main () {
    println!("Initializing CPU");
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

    //
    // Load the test rom
    //
    println!("Loading test ROM");
    cpu.bus.cartridge.load_rom("gb-test-roms-master/cpu_instrs/individual/03-op sp,hl.gb")
    .expect("Failed to load the ROM file. Check the path!");
    println!("Test ROM loaded");

    //
    // Run the CPU
    //

    let mut cycle_num: i64 = 0;

    println!("Running CPU forever\n\n");

    while(true) {
        cpu.cycle();
        //thread::sleep(Duration::from_millis(1));
    }
}