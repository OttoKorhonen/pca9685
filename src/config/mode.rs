pub const MASK_MODE1: u8 = 0b0000_0000;
pub const MASK_ALL_CALL: u8 = 0b0000_0001;
pub const MASK_SUB3: u8     = 0b0000_0010;
pub const MASK_SUB2: u8     = 0b0000_0100;
pub const MASK_SUB1: u8     = 0b0000_1000;
pub const MASK_SLEEP: u8    = 0b0001_0000;
pub const MASK_AI: u8       = 0b0010_0000;
pub const MASK_EXTCLK: u8   = 0b0100_0000;
pub const MASK_RESTART: u8  = 0b1000_0000;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Mode {
    bits: u8,
}

impl Mode {
    pub const fn new() -> Self {
        Self { bits: 0 }
    }


    pub const fn get_value(self) -> u8 {
        self.bits
    }
    ///PCA9685 responds to LED All Call I2C-bus address.
    pub const fn set_all_call(mut self, enable: bool) -> Self {
        if enable {
            self.bits |= MASK_ALL_CALL;
        } else {
            self.bits &= !MASK_ALL_CALL;
        }
        self
    }
    ///PCA9685 responds to I2C-bus subaddress 3
    pub const fn set_sub3(mut self, enable: bool) -> Self {
        if enable {
            self.bits |= MASK_SUB3;
        } else {
            self.bits &= !MASK_SUB3;
        }
        self
    }
    ///PCA9685 responds to I2C-bus subaddress 2.
    pub const fn set_sub2(mut self, enable: bool) -> Self {
        if enable {
            self.bits |= MASK_SUB2;
        } else {
            self.bits &= !MASK_SUB2;
        }
        self
    }
    ///PCA9685 responds to I2C-bus subaddress 1.
    pub const fn set_sub1(mut self, enable: bool) -> Self {
        if enable {
            self.bits |= MASK_SUB1;
        } else {
            self.bits &= !MASK_SUB1;
        }
        self
    }
    ///set sleep mode
    pub const fn set_sleep(mut self, enable: bool) -> Self {
        if enable {
            self.bits |= MASK_SLEEP;
        } else {
            self.bits &= !MASK_SLEEP;
        }
        self
    }
    ///Register Auto-Increment enabled
    pub const fn set_auto_increment(mut self, enable: bool) -> Self {
        if enable {
            self.bits |= MASK_AI;
        } else {
            self.bits &= !MASK_AI;
        }
        self
    }
    ///enable EXTCLK pin clock.
    pub const fn set_external_clock(mut self, enable: bool) -> Self {
        if enable {
            self.bits |= MASK_EXTCLK;
        } else {
            self.bits &= !MASK_EXTCLK;
        }
        self
    }
    ///Restart enabled.
    pub const fn set_restart(mut self, enable: bool) -> Self {
        if enable {
            self.bits |= MASK_RESTART;
        } else {
            self.bits &= !MASK_RESTART;
        }
        self
    }
}
 impl Default for Mode {
     fn default() -> Self {
         let default_bits = 0b0001_0001;
         Self{bits: default_bits}
     }
 }