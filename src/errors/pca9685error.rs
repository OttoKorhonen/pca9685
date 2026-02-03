use core::{fmt, error::Error};

#[derive(Debug)]
pub enum Pca9685Error<E> {
    DeviceNotFound,
    I2CError(E),
    DelayTimeOutOfScope,
}

impl<E> fmt::Display for Pca9685Error<E>
where
    E: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::I2CError(e) => write!(f,"I2C error {:?}", e),
            Self::DeviceNotFound => write!(f, "Device not found!"),
            Self::DelayTimeOutOfScope => write!(f, "Delay time is out of scope")
        }
    }
}

impl <E> Error for Pca9685Error<E> where E: fmt::Debug{}

impl<E> From<E> for Pca9685Error<E> {
    fn from(e: E) -> Self {
        Self::I2CError(e)
    }
}