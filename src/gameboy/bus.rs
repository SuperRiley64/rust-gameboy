use crate::gameboy::cartridge::Cartridge;
use crate::gameboy::serial::Serial;
use crate::gameboy::timer::Timer;

pub struct Bus {
    pub memory: [u8; 0x10000],
    pub cartridge: Cartridge,
    pub serial: Serial,
    pub timer: Timer,
}

impl Bus {
    pub fn new() -> Self {
        Self {
            memory: [0; 0x10000],
            cartridge: Cartridge::new(),
            serial: Serial::new(),
            timer: Timer::new(),
        }
    }

    // Reads the correct data from the memory map
    pub fn read(&self, address: u16) -> u8 {
        // Cartridge Memory
        if address <= 0x3FFF { return self.cartridge.bank00[address as usize]; } // ROM bank 00 space
        if address > 0x3FFF && address <= 0x7FFF { return self.cartridge.banknn[(address - 0x4000) as usize]; } // ROM bank 00 space

        // Serial Registers
        if address == 0xFF01 { return self.serial.SB } // Serial transfer data byte
        if address == 0xFF02 { return self.serial.SC } // Serial transfer control byte

        // Timer Registers
        if address == 0xFF04 { return self.timer.div_reg; } // Timer Divider Register - Any write resets the timer
        if address == 0xFF05 { return self.timer.counter_reg; } // Timer Counter Register
        if address == 0xFF06 { return self.timer.modulo_reg; } // Timer Modulo Register
        if address == 0xFF07 { return self.timer.control_reg; } // Timer Control Register

        if address == 0xFF0F { 
            let mut val :u8 = 0;
            
            if self.timer.irq { val |= 0b00000100; }

            return val;
        }

        // Normal memory space
        else { return self.memory[address as usize]; }
    }

    pub fn write(&mut self, address: u16, value: u8) {
        // Writes to cartridge 0 address space are allowed here, but ignored by read.

        // Serial Registers
        if address == 0xFF01 { self.serial.SB = value; return; } // Serial transfer data byte
        if address == 0xFF02 { self.serial.SC = value; return; } // Serial transfer control byte

        // Timer Registers
        if address == 0xFF04 { self.timer.reset_div(); } // Timer Divider Register - Any write resets the timer
        if address == 0xFF05 { self.timer.counter_reg = value; } // Timer Counter Register
        if address == 0xFF06 { self.timer.modulo_reg = value; } // Timer Modulo Register
        if address == 0xFF07 { self.timer.control_reg = value; } // Timer Control Register

        // Interrupt Request Register
        if address == 0xFF0F { 
            self.timer.irq = value & 0b00000100 != 0;
        }


        self.memory[address as usize] = value;
    }

    pub fn irq_cycle(&mut self){
        if self.timer.irq {
            self.memory[0xFF0F] |= 0x04; // Set the IRQ flag for timer
        }
    }
}