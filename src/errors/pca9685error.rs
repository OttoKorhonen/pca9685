
pub enum Pca9685Error<E>{
    DeviceNotFound,
    I2CError(E)
}