use crate::gameboy::cartridge::Cartridge;
use crate::gameboy::serial::Serial;

pub struct Bus {
    pub memory: [u8; 0x10000],
    pub cartridge: Cartridge,
    pub serial: Serial,
}

impl Bus {
    pub fn new() -> Self {
        Self {
            memory: [0; 0x10000],
            cartridge: Cartridge::new(),
            serial: Serial::new(),
        }
    }

    // Reads the correct data from the memory map
    pub fn read(&self, address: u16) -> u8 {
        if address <= 0x3FFF { return self.cartridge.bank00[address as usize]; } // ROM bank 00 space
        if address > 0x3FFF && address <= 0x7FFF { return self.cartridge.banknn[(address - 0x4000) as usize]; } // ROM bank 00 space
        if address == 0xFF01 { return self.serial.SB } // Serial transfer data byte
        if address == 0xFF02 { return self.serial.SC } // Serial transfer control byte
        else { return self.memory[address as usize]; } // Normal memory space
    }

    pub fn write(&mut self, address: u16, value: u8) {
        // Writes to cartridge 0 address space are allowed here, but ignored by read.
        if address == 0xFF01 { self.serial.SB = value; return; } // Serial transfer data byte
        if address == 0xFF02 { self.serial.SC = value; return; } // Serial transfer control byte
        self.memory[address as usize] = value;
    }
}