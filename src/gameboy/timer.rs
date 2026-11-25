
pub struct Timer {
    pub div_reg: u8, // DIV: Divider Register
    pub counter_reg: u8, // TIMA: Timer Counter
    pub modulo_reg: u8, // TMA: Timer Modulo
    pub control_reg: u8, // TAC: Timer Control
    pub irq: bool,
}

impl Timer {
    pub fn new() -> Self {
        Self {
            div_reg: 0,
            counter_reg: 0,
            modulo_reg: 0,
            control_reg: 0,
            irq: false,
        }
    }

    pub fn reset_div(&mut self){
        self.div_reg = 0;
    }

    //TODO: Implement DMG timer-tick behavior on timer disable https://gbdev.io/pandocs/Timer_Obscure_Behaviour.html#relation-between-timer-and-divider-register
    pub fn cycle(&mut self, m_cycle :u64) {
        let mut should_tick = false;
        let enabled = self.control_reg & 0x04 != 0;
        let clock_select = self.control_reg & 0x03;

        if m_cycle % 64 == 0 { // 16384Hz Divider Register increment
            self.div_reg = self.div_reg.wrapping_add(1);
        }

        //TODO: Update these to run from the div_reg
        if enabled { // If Enable bit is set...
            match clock_select {
                0x00 => { should_tick = self.div_reg == 0; }
                0x01 =>  { should_tick = self.div_reg % 64 == 0; }
                0x02 =>  { should_tick = self.div_reg % 16 == 0; }
                0x03 =>  { should_tick = self.div_reg % 4 == 0; }
                _ => { should_tick = false }
            }
        }

        if should_tick {
            let (result, overflow) = self.counter_reg.overflowing_add(1);
            self.counter_reg = result;

            if overflow { // If counter register overflowed...
                self.counter_reg = self.modulo_reg;
                self.irq = true;
            }
        }
    }
}