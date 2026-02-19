const MASK_OUTNE: u8 = 0b0000_0001;
const MASK_OUTDRV: u8 = 0b0000_0100;
const MASK_OCH: u8 = 0b0000_1000;
const MASK_INVRT: u8 = 0b0001_0000;

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
    /// 0 = default
    /// 00 = When OE = 1 (output drivers not enabled), LEDn = 0.
    /// 01 = When OE = 1 (output drivers not enabled):
    pub const fn set_outne(mut self, enable: bool) -> Self {
        match enable {
            true => self.bits |= MASK_OUTNE,
            false => self.bits &= !MASK_OUTNE,
        }
        self
    }
    /// 1 = default
    /// 1 = The 16 LEDn outputs are configured with a totem pole structure.
    /// 0 = The 16 LEDn outputs are configured with an open-drain structure.
    pub const fn set_outdrv(mut self, enable: bool) -> Self {
        match enable {
            true => self.bits |= MASK_OUTDRV,
            false => self.bits &= !MASK_OUTDRV,
        }
        self
    }
    ///0 = default
    /// 1 = Outputs change on ACK
    /// 0 = Outputs change on STOP
    pub const fn set_och(mut self, enable: bool) -> Self {
        match enable {
            true => self.bits |= MASK_OCH,
            false => self.bits &= !MASK_OCH,
        }
        self
    }
    ///default = 0
    /// 1 = Output logic state inverted. Value to use when no external driver used.
    /// Applicable when OE = 0.
    ///Output logic state not inverted. Value to use when external driver used.
    /// Applicable when OE = 0.
    pub const fn set_invrt(mut self, enable: bool) -> Self {
        match enable {
            true => self.bits |= MASK_INVRT,
            false => self.bits &= !MASK_INVRT,
        }
        self
    }
}
