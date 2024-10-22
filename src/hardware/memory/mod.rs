pub mod ram;
pub mod rom;
pub mod error;

use crate::hardware::memory::error::{MemoryResult, MemoryError};

/// A compile time sized memory device that can be read and written to
pub trait MemoryDevice: Sized {
    fn get_size(&self) -> usize;
    fn validate_address(&self, address: u16) -> error::MemoryResult<()>{
        if address >= self.get_size() as u16 {
            Err(MemoryError::OutOfBounds)
        } else {
            Ok(())
        }
    }
    fn read_u8(&self, address: u16) -> MemoryResult<u8>;
    fn write_u8(&mut self, address: u16, value: u8) -> Result<(), MemoryError>;

}