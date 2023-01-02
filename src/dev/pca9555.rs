//! Support for the `PCA9555` "16-bit I2C-bus and SMBus I/O port with interrupt"
use crate::I2cExt;

/// `PCA9555` "16-bit I2C-bus and SMBus I/O port with interrupt"
pub struct Pca9555<M>(pub M, pub u8);

impl<I2C> Pca9555<shared_bus::NullMutex<Driver<I2C>>>
where
    I2C: crate::I2cBus,
{
    pub fn new(i2c: I2C, a0: bool, a1: bool, a2: bool) -> Self {
        Self::with_mutex(i2c, a0, a1, a2)
    }

    pub fn new_m(i2c: I2C) -> Self {
        Self::with_mutex_m(i2c)
    }
}

impl<I2C, M> Pca9555<M>
where
    I2C: crate::I2cBus,
    M: shared_bus::BusMutex<Bus = Driver<I2C>>,
{
    pub fn with_mutex(i2c: I2C, a0: bool, a1: bool, a2: bool) -> Self {
        Self(
            shared_bus::BusMutex::create(Driver::new(i2c, a0, a1, a2)),
            1,
        )
    }

    pub fn with_mutex_m(i2c: I2C) -> Self {
        Self(shared_bus::BusMutex::create(Driver::new_m(i2c)), 8)
    }

    /*
    fn read_u16(&mut self, index: u8) -> u16
    where
        <M as crate::PortDriver>::Error: core::fmt::Debug,
    {
        assert!(index < self.1);
        self.0.get(index, 0xFFFF, 0).unwrap() as u16
    }

    fn write_u16(&mut self, index: u8, data: u16)
    where
        <M as crate::PortDriver>::Error: core::fmt::Debug,
    {
        assert!(index < self.1);
        self.0.set(index, data as u32, !data as u32).unwrap();
    }
    */

    pub fn split<'a>(&'a mut self) -> Parts<'a, I2C, M> {
        assert!(self.1 == 1);
        Parts {
            io0a_0: crate::Pin::new(0, 0, &self.0),
            io0a_1: crate::Pin::new(0, 1, &self.0),
            io0a_2: crate::Pin::new(0, 2, &self.0),
            io0a_3: crate::Pin::new(0, 3, &self.0),
            io0a_4: crate::Pin::new(0, 4, &self.0),
            io0a_5: crate::Pin::new(0, 5, &self.0),
            io0a_6: crate::Pin::new(0, 6, &self.0),
            io0a_7: crate::Pin::new(0, 7, &self.0),
            io0b_0: crate::Pin::new(0, 8, &self.0),
            io0b_1: crate::Pin::new(0, 9, &self.0),
            io0b_2: crate::Pin::new(0, 10, &self.0),
            io0b_3: crate::Pin::new(0, 11, &self.0),
            io0b_4: crate::Pin::new(0, 12, &self.0),
            io0b_5: crate::Pin::new(0, 13, &self.0),
            io0b_6: crate::Pin::new(0, 14, &self.0),
            io0b_7: crate::Pin::new(0, 15, &self.0),
        }
    }

    pub fn split_m<'a>(&'a mut self) -> PartsM<'a, I2C, M> {
        assert!(self.1 == 8);
        PartsM {
            io0a_0: crate::Pin::new(0, 0, &self.0),
            io0a_1: crate::Pin::new(0, 1, &self.0),
            io0a_2: crate::Pin::new(0, 2, &self.0),
            io0a_3: crate::Pin::new(0, 3, &self.0),
            io0a_4: crate::Pin::new(0, 4, &self.0),
            io0a_5: crate::Pin::new(0, 5, &self.0),
            io0a_6: crate::Pin::new(0, 6, &self.0),
            io0a_7: crate::Pin::new(0, 7, &self.0),
            io0b_0: crate::Pin::new(0, 8, &self.0),
            io0b_1: crate::Pin::new(0, 9, &self.0),
            io0b_2: crate::Pin::new(0, 10, &self.0),
            io0b_3: crate::Pin::new(0, 11, &self.0),
            io0b_4: crate::Pin::new(0, 12, &self.0),
            io0b_5: crate::Pin::new(0, 13, &self.0),
            io0b_6: crate::Pin::new(0, 14, &self.0),
            io0b_7: crate::Pin::new(0, 15, &self.0),

            io1a_0: crate::Pin::new(1, 0, &self.0),
            io1a_1: crate::Pin::new(1, 1, &self.0),
            io1a_2: crate::Pin::new(1, 2, &self.0),
            io1a_3: crate::Pin::new(1, 3, &self.0),
            io1a_4: crate::Pin::new(1, 4, &self.0),
            io1a_5: crate::Pin::new(1, 5, &self.0),
            io1a_6: crate::Pin::new(1, 6, &self.0),
            io1a_7: crate::Pin::new(1, 7, &self.0),
            io1b_0: crate::Pin::new(1, 8, &self.0),
            io1b_1: crate::Pin::new(1, 9, &self.0),
            io1b_2: crate::Pin::new(1, 10, &self.0),
            io1b_3: crate::Pin::new(1, 11, &self.0),
            io1b_4: crate::Pin::new(1, 12, &self.0),
            io1b_5: crate::Pin::new(1, 13, &self.0),
            io1b_6: crate::Pin::new(1, 14, &self.0),
            io1b_7: crate::Pin::new(1, 15, &self.0),

            io2a_0: crate::Pin::new(2, 0, &self.0),
            io2a_1: crate::Pin::new(2, 1, &self.0),
            io2a_2: crate::Pin::new(2, 2, &self.0),
            io2a_3: crate::Pin::new(2, 3, &self.0),
            io2a_4: crate::Pin::new(2, 4, &self.0),
            io2a_5: crate::Pin::new(2, 5, &self.0),
            io2a_6: crate::Pin::new(2, 6, &self.0),
            io2a_7: crate::Pin::new(2, 7, &self.0),
            io2b_0: crate::Pin::new(2, 8, &self.0),
            io2b_1: crate::Pin::new(2, 9, &self.0),
            io2b_2: crate::Pin::new(2, 10, &self.0),
            io2b_3: crate::Pin::new(2, 11, &self.0),
            io2b_4: crate::Pin::new(2, 12, &self.0),
            io2b_5: crate::Pin::new(2, 13, &self.0),
            io2b_6: crate::Pin::new(2, 14, &self.0),
            io2b_7: crate::Pin::new(2, 15, &self.0),

            io3a_0: crate::Pin::new(3, 0, &self.0),
            io3a_1: crate::Pin::new(3, 1, &self.0),
            io3a_2: crate::Pin::new(3, 2, &self.0),
            io3a_3: crate::Pin::new(3, 3, &self.0),
            io3a_4: crate::Pin::new(3, 4, &self.0),
            io3a_5: crate::Pin::new(3, 5, &self.0),
            io3a_6: crate::Pin::new(3, 6, &self.0),
            io3a_7: crate::Pin::new(3, 7, &self.0),
            io3b_0: crate::Pin::new(3, 8, &self.0),
            io3b_1: crate::Pin::new(3, 9, &self.0),
            io3b_2: crate::Pin::new(3, 10, &self.0),
            io3b_3: crate::Pin::new(3, 11, &self.0),
            io3b_4: crate::Pin::new(3, 12, &self.0),
            io3b_5: crate::Pin::new(3, 13, &self.0),
            io3b_6: crate::Pin::new(3, 14, &self.0),
            io3b_7: crate::Pin::new(3, 15, &self.0),

            io4a_0: crate::Pin::new(4, 0, &self.0),
            io4a_1: crate::Pin::new(4, 1, &self.0),
            io4a_2: crate::Pin::new(4, 2, &self.0),
            io4a_3: crate::Pin::new(4, 3, &self.0),
            io4a_4: crate::Pin::new(4, 4, &self.0),
            io4a_5: crate::Pin::new(4, 5, &self.0),
            io4a_6: crate::Pin::new(4, 6, &self.0),
            io4a_7: crate::Pin::new(4, 7, &self.0),
            io4b_0: crate::Pin::new(4, 8, &self.0),
            io4b_1: crate::Pin::new(4, 9, &self.0),
            io4b_2: crate::Pin::new(4, 10, &self.0),
            io4b_3: crate::Pin::new(4, 11, &self.0),
            io4b_4: crate::Pin::new(4, 12, &self.0),
            io4b_5: crate::Pin::new(4, 13, &self.0),
            io4b_6: crate::Pin::new(4, 14, &self.0),
            io4b_7: crate::Pin::new(4, 15, &self.0),

            io5a_0: crate::Pin::new(5, 0, &self.0),
            io5a_1: crate::Pin::new(5, 1, &self.0),
            io5a_2: crate::Pin::new(5, 2, &self.0),
            io5a_3: crate::Pin::new(5, 3, &self.0),
            io5a_4: crate::Pin::new(5, 4, &self.0),
            io5a_5: crate::Pin::new(5, 5, &self.0),
            io5a_6: crate::Pin::new(5, 6, &self.0),
            io5a_7: crate::Pin::new(5, 7, &self.0),
            io5b_0: crate::Pin::new(5, 8, &self.0),
            io5b_1: crate::Pin::new(5, 9, &self.0),
            io5b_2: crate::Pin::new(5, 10, &self.0),
            io5b_3: crate::Pin::new(5, 11, &self.0),
            io5b_4: crate::Pin::new(5, 12, &self.0),
            io5b_5: crate::Pin::new(5, 13, &self.0),
            io5b_6: crate::Pin::new(5, 14, &self.0),
            io5b_7: crate::Pin::new(5, 15, &self.0),

            io6a_0: crate::Pin::new(6, 0, &self.0),
            io6a_1: crate::Pin::new(6, 1, &self.0),
            io6a_2: crate::Pin::new(6, 2, &self.0),
            io6a_3: crate::Pin::new(6, 3, &self.0),
            io6a_4: crate::Pin::new(6, 4, &self.0),
            io6a_5: crate::Pin::new(6, 5, &self.0),
            io6a_6: crate::Pin::new(6, 6, &self.0),
            io6a_7: crate::Pin::new(6, 7, &self.0),
            io6b_0: crate::Pin::new(6, 8, &self.0),
            io6b_1: crate::Pin::new(6, 9, &self.0),
            io6b_2: crate::Pin::new(6, 10, &self.0),
            io6b_3: crate::Pin::new(6, 11, &self.0),
            io6b_4: crate::Pin::new(6, 12, &self.0),
            io6b_5: crate::Pin::new(6, 13, &self.0),
            io6b_6: crate::Pin::new(6, 14, &self.0),
            io6b_7: crate::Pin::new(6, 15, &self.0),

            io7a_0: crate::Pin::new(7, 0, &self.0),
            io7a_1: crate::Pin::new(7, 1, &self.0),
            io7a_2: crate::Pin::new(7, 2, &self.0),
            io7a_3: crate::Pin::new(7, 3, &self.0),
            io7a_4: crate::Pin::new(7, 4, &self.0),
            io7a_5: crate::Pin::new(7, 5, &self.0),
            io7a_6: crate::Pin::new(7, 6, &self.0),
            io7a_7: crate::Pin::new(7, 7, &self.0),
            io7b_0: crate::Pin::new(7, 8, &self.0),
            io7b_1: crate::Pin::new(7, 9, &self.0),
            io7b_2: crate::Pin::new(7, 10, &self.0),
            io7b_3: crate::Pin::new(7, 11, &self.0),
            io7b_4: crate::Pin::new(7, 12, &self.0),
            io7b_5: crate::Pin::new(7, 13, &self.0),
            io7b_6: crate::Pin::new(7, 14, &self.0),
            io7b_7: crate::Pin::new(7, 15, &self.0),
        }
    }
}

pub struct Parts<'a, I2C, M = shared_bus::NullMutex<Driver<I2C>>>
where
    I2C: crate::I2cBus,
    M: shared_bus::BusMutex<Bus = Driver<I2C>>,
{
    pub io0a_0: crate::Pin<'a, crate::mode::Input, M>,
    pub io0a_1: crate::Pin<'a, crate::mode::Input, M>,
    pub io0a_2: crate::Pin<'a, crate::mode::Input, M>,
    pub io0a_3: crate::Pin<'a, crate::mode::Input, M>,
    pub io0a_4: crate::Pin<'a, crate::mode::Input, M>,
    pub io0a_5: crate::Pin<'a, crate::mode::Input, M>,
    pub io0a_6: crate::Pin<'a, crate::mode::Input, M>,
    pub io0a_7: crate::Pin<'a, crate::mode::Input, M>,

    pub io0b_0: crate::Pin<'a, crate::mode::Input, M>,
    pub io0b_1: crate::Pin<'a, crate::mode::Input, M>,
    pub io0b_2: crate::Pin<'a, crate::mode::Input, M>,
    pub io0b_3: crate::Pin<'a, crate::mode::Input, M>,
    pub io0b_4: crate::Pin<'a, crate::mode::Input, M>,
    pub io0b_5: crate::Pin<'a, crate::mode::Input, M>,
    pub io0b_6: crate::Pin<'a, crate::mode::Input, M>,
    pub io0b_7: crate::Pin<'a, crate::mode::Input, M>,
}

pub struct PartsM<'a, I2C, M = shared_bus::NullMutex<Driver<I2C>>>
where
    I2C: crate::I2cBus,
    M: shared_bus::BusMutex<Bus = Driver<I2C>>,
{
    pub io0a_0: crate::Pin<'a, crate::mode::Input, M>,
    pub io0a_1: crate::Pin<'a, crate::mode::Input, M>,
    pub io0a_2: crate::Pin<'a, crate::mode::Input, M>,
    pub io0a_3: crate::Pin<'a, crate::mode::Input, M>,
    pub io0a_4: crate::Pin<'a, crate::mode::Input, M>,
    pub io0a_5: crate::Pin<'a, crate::mode::Input, M>,
    pub io0a_6: crate::Pin<'a, crate::mode::Input, M>,
    pub io0a_7: crate::Pin<'a, crate::mode::Input, M>,

    pub io0b_0: crate::Pin<'a, crate::mode::Input, M>,
    pub io0b_1: crate::Pin<'a, crate::mode::Input, M>,
    pub io0b_2: crate::Pin<'a, crate::mode::Input, M>,
    pub io0b_3: crate::Pin<'a, crate::mode::Input, M>,
    pub io0b_4: crate::Pin<'a, crate::mode::Input, M>,
    pub io0b_5: crate::Pin<'a, crate::mode::Input, M>,
    pub io0b_6: crate::Pin<'a, crate::mode::Input, M>,
    pub io0b_7: crate::Pin<'a, crate::mode::Input, M>,

    pub io1a_0: crate::Pin<'a, crate::mode::Input, M>,
    pub io1a_1: crate::Pin<'a, crate::mode::Input, M>,
    pub io1a_2: crate::Pin<'a, crate::mode::Input, M>,
    pub io1a_3: crate::Pin<'a, crate::mode::Input, M>,
    pub io1a_4: crate::Pin<'a, crate::mode::Input, M>,
    pub io1a_5: crate::Pin<'a, crate::mode::Input, M>,
    pub io1a_6: crate::Pin<'a, crate::mode::Input, M>,
    pub io1a_7: crate::Pin<'a, crate::mode::Input, M>,

    pub io1b_0: crate::Pin<'a, crate::mode::Input, M>,
    pub io1b_1: crate::Pin<'a, crate::mode::Input, M>,
    pub io1b_2: crate::Pin<'a, crate::mode::Input, M>,
    pub io1b_3: crate::Pin<'a, crate::mode::Input, M>,
    pub io1b_4: crate::Pin<'a, crate::mode::Input, M>,
    pub io1b_5: crate::Pin<'a, crate::mode::Input, M>,
    pub io1b_6: crate::Pin<'a, crate::mode::Input, M>,
    pub io1b_7: crate::Pin<'a, crate::mode::Input, M>,

    pub io2a_0: crate::Pin<'a, crate::mode::Input, M>,
    pub io2a_1: crate::Pin<'a, crate::mode::Input, M>,
    pub io2a_2: crate::Pin<'a, crate::mode::Input, M>,
    pub io2a_3: crate::Pin<'a, crate::mode::Input, M>,
    pub io2a_4: crate::Pin<'a, crate::mode::Input, M>,
    pub io2a_5: crate::Pin<'a, crate::mode::Input, M>,
    pub io2a_6: crate::Pin<'a, crate::mode::Input, M>,
    pub io2a_7: crate::Pin<'a, crate::mode::Input, M>,

    pub io2b_0: crate::Pin<'a, crate::mode::Input, M>,
    pub io2b_1: crate::Pin<'a, crate::mode::Input, M>,
    pub io2b_2: crate::Pin<'a, crate::mode::Input, M>,
    pub io2b_3: crate::Pin<'a, crate::mode::Input, M>,
    pub io2b_4: crate::Pin<'a, crate::mode::Input, M>,
    pub io2b_5: crate::Pin<'a, crate::mode::Input, M>,
    pub io2b_6: crate::Pin<'a, crate::mode::Input, M>,
    pub io2b_7: crate::Pin<'a, crate::mode::Input, M>,

    pub io3a_0: crate::Pin<'a, crate::mode::Input, M>,
    pub io3a_1: crate::Pin<'a, crate::mode::Input, M>,
    pub io3a_2: crate::Pin<'a, crate::mode::Input, M>,
    pub io3a_3: crate::Pin<'a, crate::mode::Input, M>,
    pub io3a_4: crate::Pin<'a, crate::mode::Input, M>,
    pub io3a_5: crate::Pin<'a, crate::mode::Input, M>,
    pub io3a_6: crate::Pin<'a, crate::mode::Input, M>,
    pub io3a_7: crate::Pin<'a, crate::mode::Input, M>,

    pub io3b_0: crate::Pin<'a, crate::mode::Input, M>,
    pub io3b_1: crate::Pin<'a, crate::mode::Input, M>,
    pub io3b_2: crate::Pin<'a, crate::mode::Input, M>,
    pub io3b_3: crate::Pin<'a, crate::mode::Input, M>,
    pub io3b_4: crate::Pin<'a, crate::mode::Input, M>,
    pub io3b_5: crate::Pin<'a, crate::mode::Input, M>,
    pub io3b_6: crate::Pin<'a, crate::mode::Input, M>,
    pub io3b_7: crate::Pin<'a, crate::mode::Input, M>,

    pub io4a_0: crate::Pin<'a, crate::mode::Input, M>,
    pub io4a_1: crate::Pin<'a, crate::mode::Input, M>,
    pub io4a_2: crate::Pin<'a, crate::mode::Input, M>,
    pub io4a_3: crate::Pin<'a, crate::mode::Input, M>,
    pub io4a_4: crate::Pin<'a, crate::mode::Input, M>,
    pub io4a_5: crate::Pin<'a, crate::mode::Input, M>,
    pub io4a_6: crate::Pin<'a, crate::mode::Input, M>,
    pub io4a_7: crate::Pin<'a, crate::mode::Input, M>,

    pub io4b_0: crate::Pin<'a, crate::mode::Input, M>,
    pub io4b_1: crate::Pin<'a, crate::mode::Input, M>,
    pub io4b_2: crate::Pin<'a, crate::mode::Input, M>,
    pub io4b_3: crate::Pin<'a, crate::mode::Input, M>,
    pub io4b_4: crate::Pin<'a, crate::mode::Input, M>,
    pub io4b_5: crate::Pin<'a, crate::mode::Input, M>,
    pub io4b_6: crate::Pin<'a, crate::mode::Input, M>,
    pub io4b_7: crate::Pin<'a, crate::mode::Input, M>,

    pub io5a_0: crate::Pin<'a, crate::mode::Input, M>,
    pub io5a_1: crate::Pin<'a, crate::mode::Input, M>,
    pub io5a_2: crate::Pin<'a, crate::mode::Input, M>,
    pub io5a_3: crate::Pin<'a, crate::mode::Input, M>,
    pub io5a_4: crate::Pin<'a, crate::mode::Input, M>,
    pub io5a_5: crate::Pin<'a, crate::mode::Input, M>,
    pub io5a_6: crate::Pin<'a, crate::mode::Input, M>,
    pub io5a_7: crate::Pin<'a, crate::mode::Input, M>,

    pub io5b_0: crate::Pin<'a, crate::mode::Input, M>,
    pub io5b_1: crate::Pin<'a, crate::mode::Input, M>,
    pub io5b_2: crate::Pin<'a, crate::mode::Input, M>,
    pub io5b_3: crate::Pin<'a, crate::mode::Input, M>,
    pub io5b_4: crate::Pin<'a, crate::mode::Input, M>,
    pub io5b_5: crate::Pin<'a, crate::mode::Input, M>,
    pub io5b_6: crate::Pin<'a, crate::mode::Input, M>,
    pub io5b_7: crate::Pin<'a, crate::mode::Input, M>,

    pub io6a_0: crate::Pin<'a, crate::mode::Input, M>,
    pub io6a_1: crate::Pin<'a, crate::mode::Input, M>,
    pub io6a_2: crate::Pin<'a, crate::mode::Input, M>,
    pub io6a_3: crate::Pin<'a, crate::mode::Input, M>,
    pub io6a_4: crate::Pin<'a, crate::mode::Input, M>,
    pub io6a_5: crate::Pin<'a, crate::mode::Input, M>,
    pub io6a_6: crate::Pin<'a, crate::mode::Input, M>,
    pub io6a_7: crate::Pin<'a, crate::mode::Input, M>,

    pub io6b_0: crate::Pin<'a, crate::mode::Input, M>,
    pub io6b_1: crate::Pin<'a, crate::mode::Input, M>,
    pub io6b_2: crate::Pin<'a, crate::mode::Input, M>,
    pub io6b_3: crate::Pin<'a, crate::mode::Input, M>,
    pub io6b_4: crate::Pin<'a, crate::mode::Input, M>,
    pub io6b_5: crate::Pin<'a, crate::mode::Input, M>,
    pub io6b_6: crate::Pin<'a, crate::mode::Input, M>,
    pub io6b_7: crate::Pin<'a, crate::mode::Input, M>,

    pub io7a_0: crate::Pin<'a, crate::mode::Input, M>,
    pub io7a_1: crate::Pin<'a, crate::mode::Input, M>,
    pub io7a_2: crate::Pin<'a, crate::mode::Input, M>,
    pub io7a_3: crate::Pin<'a, crate::mode::Input, M>,
    pub io7a_4: crate::Pin<'a, crate::mode::Input, M>,
    pub io7a_5: crate::Pin<'a, crate::mode::Input, M>,
    pub io7a_6: crate::Pin<'a, crate::mode::Input, M>,
    pub io7a_7: crate::Pin<'a, crate::mode::Input, M>,

    pub io7b_0: crate::Pin<'a, crate::mode::Input, M>,
    pub io7b_1: crate::Pin<'a, crate::mode::Input, M>,
    pub io7b_2: crate::Pin<'a, crate::mode::Input, M>,
    pub io7b_3: crate::Pin<'a, crate::mode::Input, M>,
    pub io7b_4: crate::Pin<'a, crate::mode::Input, M>,
    pub io7b_5: crate::Pin<'a, crate::mode::Input, M>,
    pub io7b_6: crate::Pin<'a, crate::mode::Input, M>,
    pub io7b_7: crate::Pin<'a, crate::mode::Input, M>,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Regs {
    InputPort0 = 0x00,
    InputPort1 = 0x01,
    OutputPort0 = 0x02,
    OutputPort1 = 0x03,
    PolarityInversion0 = 0x04,
    PolarityInversion1 = 0x05,
    Configuration0 = 0x06,
    Configuration1 = 0x07,
}

impl From<Regs> for u8 {
    fn from(r: Regs) -> u8 {
        r as u8
    }
}

pub struct Driver<I2C> {
    i2c: I2C,
    out: [u16; 8],
    pub addr: u8,
    pub num: u8,
}

impl<I2C> Driver<I2C> {
    pub fn new(i2c: I2C, a0: bool, a1: bool, a2: bool) -> Self {
        let addr = 0x20 | ((a2 as u8) << 2) | ((a1 as u8) << 1) | (a0 as u8);
        Self {
            i2c,
            out: [0xffff; 8],
            addr,
            num: 1,
        }
    }

    pub fn new_m(i2c: I2C) -> Self {
        let addr = 0x20;
        Self {
            i2c,
            out: [0xffff; 8],
            addr,
            num: 8,
        }
    }
}

impl<I2C: crate::I2cBus> crate::PortDriver for Driver<I2C> {
    type Error = I2C::BusError;

    fn set(&mut self, index: u8, mask_high: u32, mask_low: u32) -> Result<(), Self::Error> {
        assert!(index < self.num);
        self.out[index as usize] |= mask_high as u16;
        self.out[index as usize] &= !mask_low as u16;
        if (mask_high | mask_low) & 0x00FF != 0 {
            self.i2c.write_reg(
                self.addr + index,
                Regs::OutputPort0,
                (self.out[index as usize] & 0xFF) as u8,
            )?;
        }
        if (mask_high | mask_low) & 0xFF00 != 0 {
            self.i2c.write_reg(
                self.addr + index,
                Regs::OutputPort1,
                (self.out[index as usize] >> 8) as u8,
            )?;
        }
        Ok(())
    }

    fn is_set(&mut self, index: u8, mask_high: u32, mask_low: u32) -> Result<u32, Self::Error> {
        assert!(index < self.num);
        Ok(((self.out[index as usize] as u32) & mask_high)
            | (!(self.out[index as usize] as u32) & mask_low))
    }

    fn get(&mut self, index: u8, mask_high: u32, mask_low: u32) -> Result<u32, Self::Error> {
        assert!(index < self.num);
        let io0 = if (mask_high | mask_low) & 0x00FF != 0 {
            self.i2c.read_reg(self.addr + index, Regs::InputPort0)?
        } else {
            0
        };
        let io1 = if (mask_high | mask_low) & 0xFF00 != 0 {
            self.i2c.read_reg(self.addr + index, Regs::InputPort1)?
        } else {
            0
        };
        let in_ = ((io1 as u32) << 8) | io0 as u32;
        Ok((in_ & mask_high) | (!in_ & mask_low))
    }

    fn read_u16(&mut self, index: u8) -> Result<u16, Self::Error> {
        assert!(index < self.num);
        Ok(self.get(index, 0xFFFF, 0)? as u16)
    }

    fn write_u16(&mut self, index: u8, data: u16) -> Result<(), Self::Error> {
        assert!(index < self.num);
        self.set(index, data as u32, !data as u32)
    }
}

impl<I2C: crate::I2cBus> crate::PortDriverTotemPole for Driver<I2C> {
    fn set_direction(
        &mut self,
        index: u8,
        mask: u32,
        dir: crate::Direction,
        state: bool,
    ) -> Result<(), Self::Error> {
        assert!(index < self.num);
        // set state before switching direction to prevent glitch
        if dir == crate::Direction::Output {
            use crate::PortDriver;
            if state {
                self.set(index, mask, 0)?;
            } else {
                self.set(index, 0, mask)?;
            }
        }

        let (mask_set, mask_clear) = match dir {
            crate::Direction::Input => (mask as u16, 0),
            crate::Direction::Output => (0, mask as u16),
        };
        if mask & 0x00FF != 0 {
            self.i2c.update_reg(
                self.addr + index,
                Regs::Configuration0,
                (mask_set & 0xFF) as u8,
                (mask_clear & 0xFF) as u8,
            )?;
        }
        if mask & 0xFF00 != 0 {
            self.i2c.update_reg(
                self.addr + index,
                Regs::Configuration1,
                (mask_set >> 8) as u8,
                (mask_clear >> 8) as u8,
            )?;
        }
        Ok(())
    }
}

impl<I2C: crate::I2cBus> crate::PortDriverPolarity for Driver<I2C> {
    fn set_polarity(&mut self, index: u8, mask: u32, inverted: bool) -> Result<(), Self::Error> {
        assert!(index < self.num);
        let (mask_set, mask_clear) = match inverted {
            false => (0, mask as u16),
            true => (mask as u16, 0),
        };

        if mask & 0x00FF != 0 {
            self.i2c.update_reg(
                self.addr + index,
                Regs::PolarityInversion0,
                (mask_set & 0xFF) as u8,
                (mask_clear & 0xFF) as u8,
            )?;
        }
        if mask & 0xFF00 != 0 {
            self.i2c.update_reg(
                self.addr + index,
                Regs::PolarityInversion1,
                (mask_set >> 8) as u8,
                (mask_clear >> 8) as u8,
            )?;
        }
        Ok(())
    }
}

// EOF
