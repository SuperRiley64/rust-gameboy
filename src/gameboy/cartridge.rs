use std::fs::File;
use std::io::Read;

pub struct Cartridge {
    pub bank00: [u8; 0x4000], // 16KiB
    pub banknn: [u8; 0x4000], // 16KiB
}

impl Cartridge {
    pub fn new() -> Self {
        Self {
            bank00: [0; 0x4000],
            banknn: [0; 0x4000],
        }
    }

    pub fn load_rom(&mut self, path: &str) -> std::io::Result<()> {
        let mut file = File::open(path)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;

        let bank_size = 0x4000; // 16KiB

        // 1. Handle bank00 (First 16KiB)
        if buffer.len() >= bank_size {
            // We take a "slice" of the buffer from 0 to 16KiB
            let chunk1 = &buffer[0..bank_size];
            self.bank00.copy_from_slice(chunk1);
        } else {
            println!("Warning: ROM file is smaller than 16KiB!");
        }

        // 2. Handle banknn (Next 16KiB)
        // We check if there is enough data for the second bank
        if buffer.len() >= (bank_size * 2) {
            // We take a slice from 16KiB to 32KiB
            let chunk2 = &buffer[bank_size..(bank_size * 2)];
            self.banknn.copy_from_slice(chunk2);
        } else if buffer.len() > bank_size {
            println!("Warning: ROM file is between 16KiB and 32KiB. banknn will be partially empty.");
        } else {
            println!("Warning: ROM file is too small to fill banknn.");
        }

        Ok(())
    }
}