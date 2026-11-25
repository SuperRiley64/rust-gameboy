mod gameboy;
mod tests;
use gameboy::cpu::CPU;
use std::thread;
use std::time::{Duration, Instant};

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
    cpu.bus.cartridge.load_rom("gb-test-roms-master/cpu_instrs/individual/02-interrupts.gb")
    .expect("Failed to load the ROM file. Check the path!");
    println!("Test ROM loaded");

    //
    // Run the CPU
    //
    
    let mut t_cycles :u64 = 0;
    let mut m_cycles :u64 = 0;

    let mut next_tick_time = Instant::now();
    // 1 master clock tick is 1 / 4.194304 MHz = ~238.4 ns
    let target = Duration::from_nanos(238);

    println!("Running CPU forever\n\n");

    while true {
        t_cycles += 1;

        // Process interrupts
        cpu.bus.irq_cycle();

        // CPU - 1.048576 MHz
        if t_cycles % 4 == 0 {

            if !cpu.stop {
                cpu.bus.timer.cycle(m_cycles);
            }

            cpu.cycle();
            m_cycles += 1;
        }

        // Serial 8192Hz
        if t_cycles % 512 == 0 {
            if !cpu.stop {
                cpu.bus.serial.cycle();
            }
        }

        // Timing calculation and spin loop
        let now = Instant::now();
        if now >= next_tick_time {
            next_tick_time += target;
            
            // If we are significantly behind, jump to now to avoid huge lag
            if now > next_tick_time {
                next_tick_time = now + target;
            }
            
            if now < next_tick_time {
                let remaining = next_tick_time - now;
                // Only sleep if we have a significant amount of time to spare
                if remaining > Duration::from_millis(1) {
                    thread::sleep(remaining);
                } else {
                    // Spin-loop for high-precision timing in the short term
                    while Instant::now() < next_tick_time {
                        std::hint::spin_loop();
                    }
                }
            }
        }
    }
}