pub mod chips;
pub mod error;

use crate::hardware::core::devices::memory::error::MemoryError;

pub trait MemoryDevice {
    fn read_u8(&self, address: u16) -> Result<u8, MemoryError>;
    fn write_u8(&mut self, address: u16, value: u8) -> Result<(), MemoryError>;

    /// Big Endian
    fn read_u16(&self, address: u16) -> Result<u16, MemoryError> {
        let high = self.read_u8(address)?;
        let low = self.read_u8(address.wrapping_add(1))?;
        Ok(u16::from_be_bytes([high, low]))
    }

    /// Big Endian
    fn write_u16(&mut self, address: u16, value: u16) -> Result<(), MemoryError> {
        let bytes = value.to_be_bytes();
        self.write_u8(address, bytes[0])?;
        self.write_u8(address + 1, bytes[1])?;
        Ok(())
    }
}


/// A memory device that can be read and written to
pub trait Memory<M: MemoryDevice + ?Sized> {
    fn get_size(&self) -> usize;
    fn validate_address(&self, address: u16) -> Result<(), MemoryError> {
        if address >= self.get_size() as u16 || address < 0 {
            Err(MemoryError::OutOfBounds)
        } else {
            Ok(())
        }
    }
}