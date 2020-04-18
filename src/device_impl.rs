use crate::{ic, Command, Config, Error, Max1704x, Register};
use core::marker::PhantomData;
use embedded_hal::blocking::i2c;

impl<I2C> Max1704x<I2C, ic::Max17043_4> {
    /// Create new instance of a MAX17043 or MAX17044 device.
    pub fn new_max17043_4(i2c: I2C) -> Self {
        Max1704x {
            i2c,
            config: Config { bits: 0x971C },
            _ic: PhantomData,
        }
    }
}
impl<I2C, IC> Max1704x<I2C, IC> {
    /// Destroy driver instance, return I2C bus.
    pub fn destroy(self) -> I2C {
        self.i2c
    }
}

impl<I2C, E, IC> Max1704x<I2C, IC>
where
    I2C: i2c::WriteRead<Error = E> + i2c::Write<Error = E>,
{
    /// Get IC version
    pub fn version(&mut self) -> Result<u16, Error<E>> {
        self.read_register(Register::VERSION)
    }

    /// Software reset
    pub fn reset(&mut self) -> Result<(), Error<E>> {
        self.write_register(Register::COMMAND, Command::POR)
    }
}

impl Config {
    fn with_high(self, mask: u16) -> Self {
        Config {
            bits: self.bits | mask,
        }
    }
    fn with_low(self, mask: u16) -> Self {
        Config {
            bits: self.bits & !mask,
        }
    }
}
