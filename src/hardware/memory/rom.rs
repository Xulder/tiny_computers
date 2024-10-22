use crate::hardware::memory::error::{MemoryError, MemoryResult};

pub struct ROM<const SIZE: usize> {
    data: [u8; SIZE],
}

impl<const SIZE: usize> ROM<SIZE> {
    pub fn new(data: [u8; SIZE]) -> ROM<SIZE> {
        ROM { data }
    }

    pub fn get_size(&self) -> usize {
        SIZE
    }

    pub fn read_u8(&self, address: u16) -> MemoryResult<u8> {
        if address >= SIZE as u16 {
            return Err(MemoryError::OutOfBounds);
        }
        Ok(self.data[address as usize])
    }
}
