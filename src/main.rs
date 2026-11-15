mod gameboy;
mod tests;
use gameboy::cpu::CPU;

fn main () {
    println!("CPU under construction.");
    let mut cpu = CPU::new();
}