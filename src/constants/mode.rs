///1 = default
///1 = PCA9685 responds to LED All Call I2C-bus address.
///0 = PCA9685 does not respond to LED All Call I2C-bus address.
pub const ALL_CALL_1: u8 = 0b1;
pub const ALL_CALL_0: u8 = 0b0;

///0 = default
///1 = PCA9685 responds to I2C-bus subaddress 3.
///0 = PCA9685 does not respond to I2C-bus subaddress 3.
pub const SUB3_1: u8 = 0b1 << 1;
pub const SUB3_0: u8 = 0b0 << 1;

///0 = default
///1 = PCA9685 responds to I2C-bus subaddress 2.
///0 PCA9685 does not respond to I2C-bus subaddress 2.
pub const SUB2_1: u8 = 0b1 << 2;
pub const SUB2_0: u8 = 0b0 << 2;

///0 = default
///1 = PCA9685 responds to I2C-bus subaddress 1.
///0 = PCA9685 does not respond to I2C-bus subaddress 1.
pub const SUB1_1: u8 = 0b1 << 3;
pub const SUB1_0: u8 = 0b0 << 3;

///1 = default
///1 = Low power mode. Oscillator off.
///0 = Normal mode[
pub const SLEEP_1: u8 = 0b1 << 4;
pub const SLEEP_0: u8 = 0b0 << 4;

///0 = default
///1 = Register Auto-Increment enabled.
///0 = Register Auto-Increment disabled
pub const AI_1: u8 = 0b1 << 5;
pub const AI_0: u8 = 0b0 << 5;

///To use the EXTCLK pin, this bit must be set by the following sequence:
/// 1. Set the SLEEP bit in MODE1. This turns off the internal oscillator.
/// 2. Write logic 1s to both the SLEEP and EXTCLK bits in MODE1. The switch is
/// now made. The external clock can be active during the switch because the
/// SLEEP bit is set.
/// This bit is a ‘sticky bit’, that is, it cannot be cleared by writing a logic 0 to it. The
/// EXTCLK bit can only be cleared by a power cycle or software reset.
/// EXTCLK range is DC to 50 MHz.
///                                      EXTCLK
/// refresh_rate = -------------------------------------------------------
///                              4096 × ( prescale + 1 )
/// 0 = default
pub const EXTCLK_1: u8 = 0b1 << 6;
pub const EXTCLK_0: u8 = 0b0 << 6; //default

///0 = default
///1 = Restart enabled.
/// 0 = Restart disabled
pub const RESTART_1: u8 = 0b1 << 7;
pub const RESTART_0: u8 = 0b0 << 7;