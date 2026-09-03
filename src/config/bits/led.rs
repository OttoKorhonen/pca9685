pub struct LedRegisters;

impl LedRegisters {
    pub const BASE_ADDRESS: u8 = 0x06;

    pub fn on_l(led: u8) -> u8 {
        Self::BASE_ADDRESS + led * 4
    }

    pub fn on_h(led: u8) -> u8 {
        Self::BASE_ADDRESS + led * 4 + 1
    }

    pub fn off_l(led: u8) -> u8 {
        Self::BASE_ADDRESS + led * 4 + 2
    }

    pub fn off_h(led: u8) -> u8 {
        Self::BASE_ADDRESS + led * 4 + 3
    }
}
