# rust-gameboy
Game Boy emulator project in Rust, using gemma4:12b locally as a tool.

## usage
run `cargo test` to test the emulated CPU.

## current state of development
All opcodes have been implemented in cpu.rs. They may be buggy. Without a bus or a screen set up yet, I can't run test roms.

## next step:
Interrupt handling for the cpu.rs.

## main.rs 
Currently runs tests on the Game Boy CPU.

## gameboy module
Contains cpu.rs, where the registers, opcodes, and memory space is set up.
