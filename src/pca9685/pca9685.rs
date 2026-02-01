use embedded_hal::{i2c::SevenBitAddress, delay::DelayNs};

pub struct Pca9685<I2C, D>{
    address: SevenBitAddress,
    i2c: I2C,
    delay: D
}

impl <I2c, D>Pca9685<I2c, D> where I2c: embedded_hal::i2c::I2c, D: DelayNs{

}