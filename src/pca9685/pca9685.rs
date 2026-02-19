use crate::errors::pca9685error::Pca9685Error;
use embedded_hal::{delay::DelayNs, i2c::SevenBitAddress};
use crate::config::{mode::Mode, mode2::Mode2};

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
        Self {
            i2c,
            address,
            delay,
        }
    }

    pub fn set_mode1(&mut self, mode: Mode) -> Result<(), Pca9685Error<I2c::Error>> {
        // MODE1 register is at 0x00
        self.i2c.write(self.address, &[0x00, mode.get_value()])?;
        Ok(())
    }

    pub fn set_mode2(&mut self, mode2: Mode2) -> Result<(), Pca9685Error<I2c::Error>> {
        // MODE1 register is at 0x01
        self.i2c.write(self.address, &[0x01, mode2.get_value()])?;
        Ok(())
    }

    pub fn read_register(&mut self, register: u8) -> Result<u8, Pca9685Error<I2c::Error>> {
        let mut buffer = [0];
        self.i2c.write_read(self.address, &[register], &mut buffer)?;
        Ok(buffer[0])
    }

    pub fn set_pwm_frequency(&mut self, freq_hz: f32) -> Result<(), Pca9685Error<I2c::Error>> {
        let mut prescale_val = 25_000_000.0 / (4096.0 * freq_hz);
        prescale_val -= 1.0;
        let prescale = (prescale_val + 0.5) as u8;

        let old_mode = self.read_register(0x00)?;
        let new_mode = (old_mode & 0x7F) | 0x10; // sleep
        self.i2c.write(self.address, &[0x00, new_mode])?;
        self.i2c.write(self.address, &[0xFE, prescale])?;
        self.i2c.write(self.address, &[0x00, old_mode])?;
        self.delay.delay_us(500);
        self.i2c.write(self.address, &[0x00, old_mode | 0x80])?;

        Ok(())
    }

    pub fn set_pwm(&mut self, channel: u8, delay_time: u16, pwm_duty_cycle: u16) -> Result<(), Pca9685Error<I2c::Error>> {
        if channel > 15 {
            return Err(Pca9685Error::InvalidChannel);
        }

        let (led_on, led_off) = Self::count_duty_cycle(delay_time, pwm_duty_cycle)?;

        let on_l = (led_on & 0xFF) as u8;
        let on_h = (led_on >> 8) as u8;
        let off_l = (led_off & 0xFF) as u8;
        let off_h = (led_off >> 8) as u8;

        let base_reg = 0x06 + 4 * channel;

        self.i2c.write(self.address, &[base_reg, on_l])?;
        self.i2c.write(self.address, &[base_reg + 1, on_h])?;
        self.i2c.write(self.address, &[base_reg + 2, off_l])?;
        self.i2c.write(self.address, &[base_reg + 3, off_h])?;

        Ok(())
    }

    pub fn set_pulse_us(&mut self, channel: u8, pulse_us: u16) -> Result<(), Pca9685Error<I2c::Error>> {
        let counts = (pulse_us as f32 * 4096.0 / 20000.0) as u16;

        self.set_pwm(channel, 0, counts)
    }
    pub fn set_servo_angle(&mut self, channel: u8, angle: f32) -> Result<(), Pca9685Error<I2c::Error>> {
        let clamped_angle = angle.clamp(0.0, 180.0);
        let pulse_us = 1000.0 + (clamped_angle / 180.0 * 1000.0);

        self.set_pulse_us(channel, pulse_us as u16)
    }
    
    pub fn set_throttle(&mut self, channel: u8, throttle: f32) -> Result<(), Pca9685Error<I2c::Error>> {
        let clamped = throttle.clamp(0.0, 1.0);
        let pulse_us = 1000.0 + (clamped * 1000.0);

        self.set_pulse_us(channel, pulse_us as u16)
    }

    fn count_duty_cycle(
        delay_time: u16,
        pwm_duty_cycle: u16,
    ) -> Result<(u16, u16), Pca9685Error<I2c::Error>> {
        match delay_time {
            0..=4095 => match pwm_duty_cycle {
                0 => Ok((0, 4096)), // Full OFF
                4096 => Ok((4096, 0)), // Full ON
                1..=4095 => {
                    let led_on = delay_time;
                    let led_off = (delay_time + pwm_duty_cycle) % 4096;

                    Ok((led_on, led_off))
                }
                _ => Err(Pca9685Error::PWMDutyCycleOutOfScope),
            },
            _ => Err(Pca9685Error::DelayTimeOutOfScope),
        }
    }
}
