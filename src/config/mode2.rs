
const MASK_MODE_2: u8 = 0b0000_0001;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Mode2 {
    bits: u8,
}

impl Mode2 {
    pub const fn new() -> Self {
        Self { bits: 0 }
    }
    pub const fn get_value(self) -> u8 {
        self.bits
    }
}

/// 0 = default
/// 00 = When OE = 1 (output drivers not enabled), LEDn = 0.
/// 01 = When OE = 1 (output drivers not enabled):
// LEDn = 1 when OUTDRV = 1
// LEDn = high-impedance when OUTDRV = 0 (same as OUTNE[1:0] = 10)
pub const OUTNE_1: u8 = 0b01;
pub const OUTNE_0: u8 = 0b00;

/// 1 = default
/// 1 = The 16 LEDn outputs are configured with a totem pole structure.
/// 0 = The 16 LEDn outputs are configured with an open-drain structure.
pub const OUTDRV_1: u8 = 0b1 << 2;
pub const OUTDRV_0: u8 = 0b0 << 2;

///0 = default
/// 1 = Outputs change on ACK
/// 0 = Outputs change on STOP
pub const OCH_1: u8 = 0b1 << 3;
pub const OCH_0: u8 = 0b0 << 3;

///default = 0
/// 1 = Output logic state inverted. Value to use when no external driver used.
/// Applicable when OE = 0.
///Output logic state not inverted. Value to use when external driver used.
/// Applicable when OE = 0.
pub const INVRT_1: u8 = 0b1 << 4;
pub const INVRT_0: u8 = 0b0 << 4;

pub const RESERVED: u8 = 0b00 << 5;
