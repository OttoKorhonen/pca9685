use embedded_hal::{delay::DelayNs, i2c::SevenBitAddress};
use crate::errors::pca9685error::Pca9685Error;

pub struct Pca9685<I2C, D> {
    i2c: I2C,
    address: SevenBitAddress,
    delay: D,
}

impl<I2c, D> Pca9685<I2c, D>
where
    I2c: embedded_hal::i2c::I2c,
    D: DelayNs,
{
    pub fn new(i2c: I2c, address: u8, delay: D) -> Self {
        Self { i2c, address, delay }
    }

    pub fn configure() {
        todo!()
    }

    pub fn write() {
        todo!()
    }

    fn count_duty_cycle(delay_time: u8, pwm_duty_cycle: u8) -> Result<(u16, u16), Pca9685Error<I2c::Error>> {
        match delay_time {
            0..=100 => {
                let delay_time_as_percent = delay_time as f32 / 100.0;
                let duty_cycle = pwm_duty_cycle as f32 / 100.0;
                let period = 4096.0;

                let led_on = (delay_time_as_percent * period + 0.5) as u16;
                let duration = (duty_cycle * period + 0.5) as u16;
                let led_off = (led_on + duration) % 4096;

                Ok((led_on, led_off))
            }
            _ => { Err(Pca9685Error::DelayTimeOutOfScope) }

        }
    }
}
