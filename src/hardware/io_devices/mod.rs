pub mod error;
use crate::hardware::io_devices::error::IOResult;

pub trait IODevice: Sized + Copy {
    fn validate_address(&self, address: u16) -> IOResult<()>;
    fn read_u8(&self, address: u16) -> IOResult<u8>;
    fn write_u8(&mut self, address: u16, value: u8) -> IOResult<()>;
}