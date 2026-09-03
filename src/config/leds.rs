use crate::config::bits::led::LedRegisters;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Led {
    Led0 = 0,
    Led1 = 1,
    Led2 = 2,
    Led3 = 3,
    Led4 = 4,
    Led5 = 5,
    Led6 = 6,
    Led7 = 7,
    Led8 = 8,
    Led9 = 9,
    Led10 = 10,
    Led11 = 11,
    Led12 = 12,
    Led13 = 13,
    Led14 = 14,
    Led15 = 15,
}

impl Led {
    pub fn on_l(&self) -> u8 {
        LedRegisters::on_l(*self as u8)
    }

    pub fn on_h(&self) -> u8 {
        LedRegisters::on_h(*self as u8)
    }

    pub fn off_l(&self) -> u8 {
        LedRegisters::off_l(*self as u8)
    }

    pub fn off_h(&self) -> u8 {
        LedRegisters::off_h(*self as u8)
    }
}