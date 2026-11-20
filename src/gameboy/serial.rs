pub struct Serial {
    pub SB: u8, // Serial transfer data
    pub SC: u8, // Serial transfer control
}

impl Serial {
    pub fn new() -> Self {
        Self {
            SB: 0x00,
            SC: 0x00,
        }
    }

    pub fn cycle(&mut self) {
        let transer_enabled: bool = self.SC & 0b10000000 != 0; // Get transfer enabled bit
        let is_master: bool = self.SC & 0b00000001 != 0; // Get clock select bit

        if transer_enabled && is_master {
            // Just print SB to the console for now
            let s = char::from(self.SB).to_string();
            print!("{}", s);
            self.SC &= 0b01111111; // disable transfer
        }
    }
}