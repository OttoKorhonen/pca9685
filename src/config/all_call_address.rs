
pub enum AllCallAddr{
    Addr = 0x05
}

impl AllCallAddr {
    pub const fn get_value(self) -> u8 {self as u8}
}