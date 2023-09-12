#![deny(unsafe_code)]
#![feature(async_fn_in_trait)]
#![no_std]

use embedded_hal::digital::OutputPin;
use embedded_hal::spi::SpiDevice;
#[cfg(feature = "async")]
use embedded_hal_async::spi::SpiDevice as AsyncSpiDevice;

/// All possible errors in this crate
#[derive(Debug)]
pub enum Error<E> {
    /// SPI communication error
    Spi(E),
    /// Invalid argument provided
    InvalidArgument,
}

/// Channel list for Adc128s022
#[derive(Clone, Copy)]
#[allow(missing_docs)]
pub enum Channels {
    Ch0,
    Ch1,
    Ch2,
    Ch3,
    Ch4,
    Ch5,
    Ch6,
    Ch7,
}

/// Adc128s022 driver
#[derive(Debug)]
pub struct Adc128s022<DEV, CS> {
    spi: DEV,
    cs: CS,
}

trait Driver {
    type Dev;
    type Cs;
    type Error;

    fn new(spi: Self::Dev, cs: Self::Cs) -> Adc128s022<Self::Dev, Self::Cs>;
    fn read_channel(&mut self, ch: Channels) -> Result<u16, Self::Error>;
    /// Destroy driver instance, return SPI bus instance and CS output pin.
    fn destroy(self) -> (Self::Dev, Self::Cs);
}

trait AsyncDriver {
    type Dev;
    type Cs;
    type Error;

    fn new(spi: Self::Dev, cs: Self::Cs) -> Adc128s022<Self::Dev, Self::Cs>;
    async fn read_channel(&mut self, ch: Channels) -> Result<u16, Self::Error>;
    fn destroy(self) -> (Self::Dev, Self::Cs);
}

impl<DEV, CS, E> Driver for Adc128s022<DEV, CS>
where
    DEV: SpiDevice<Error = E>,
    CS: OutputPin,
{
    type Dev = DEV;
    type Cs = CS;
    type Error = Error<E>;

    fn new(spi: DEV, cs: CS) -> Self {
        Adc128s022 { spi, cs }
    }

    fn read_channel(&mut self, ch: Channels) -> Result<u16, Error<E>> {
        self.cs.set_low();

        let mut read = [0u8; 2];

        self.spi
            .transfer(&mut read, &mut [(ch as u8) << 3])
            .map_err(|e| Error::Spi(e))?;

        self.cs.set_high();

        let r = (((read[0] as u16) << 8) | (read[1] as u16)) & 0xFFF;

        Ok(r)
    }

    fn destroy(self) -> (DEV, CS) {
        (self.spi, self.cs)
    }
}

#[cfg(feature = "async")]
impl<DEV, CS, E> AsyncDriver for Adc128s022<DEV, CS>
where
    DEV: AsyncSpiDevice<Error = E>,
    CS: OutputPin,
{
    type Dev = DEV;
    type Cs = CS;
    type Error = Error<E>;

    fn new(spi: DEV, cs: CS) -> Self {
        Adc128s022 { spi, cs }
    }

    async fn read_channel(&mut self, ch: Channels) -> Result<u16, Error<E>> {
        self.cs.set_low();

        let mut read = [0u8; 2];

        self.spi
            .transfer(&mut read, &mut [(ch as u8) << 3])
            .await
            .map_err(|e| Error::Spi(e))?;

        self.cs.set_high();

        let r = (((read[0] as u16) << 8) | (read[1] as u16)) & 0xFFF;

        Ok(r)
    }

    /// Destroy driver instance, return SPI bus instance and CS output pin.
    fn destroy(self) -> (DEV, CS) {
        (self.spi, self.cs)
    }
}
