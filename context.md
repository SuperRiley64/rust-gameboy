# Game Boy Emulator Context Summary

## Architecture
- 8-bit CPU with 16-bit memory (0x0000-0xFFFF)
- Registers: A, F, B, C, D, E, H, L, SP, PC
- Flag register (F): Zero, Subtract, Half Carry

## Implemented Instructions
- 8-bit INC/DEC (B, C, D, E, H, L, A)
- 16-bit LD (BC, DE, HL, SP), INC/DEC (BC, DE, HL)
- LD (B, C, D, E, H, L, A, SP) from memory
- JP (nn)
- ADD A, A

## Completed Fixes
- Fixed 16-bit load instruction byte order in cpu.rs
- Corrected test cases in main.rs for LD BC, DE, HL, SP
- Verified flag behavior for 8-bit INC/DEC

## Test Coverage
- 8-bit INC/DEC (all registers)
- 16-bit LD (BC, DE, HL, SP)
- JP (nn)
- ADD A, A
- Remaining: ADC, SBC, AND, XOR, etc.

## Remaining Tasks
- Implement remaining opcodes (ADC, SBC, AND, XOR, etc.)
- Add more test cases for edge conditions
- Implement interrupt handling
- Add memory-mapped I/O support