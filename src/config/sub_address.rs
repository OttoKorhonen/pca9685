
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum SubAddress {
    SubAddr1 = 0x02,
    SubAddr2 = 0x03,
    SubAddr3 = 0x04
}

impl SubAddress {
    pub const fn get_value(self) -> u8 {self as u8}
}