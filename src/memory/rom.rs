use crate::memory::{
    error::{MemoryError, MemoryResult},
    traits::Memory,
};

pub struct ROM<const SIZE: usize> {
    data: [u8; SIZE],
}

impl<const SIZE: usize> ROM<SIZE> {
    pub fn new(data: [u8; SIZE]) -> ROM<SIZE> {
        ROM { data }
    }
}

impl<const SIZE: usize> Memory for ROM<SIZE> {
    fn read_u8(&self, address: u16) -> MemoryResult<u8> {
        if address >= SIZE as u16 {
            return Err(MemoryError::OutOfBounds);
        }
        Ok(self.data[address as usize])
    }

    fn write_u8(&mut self, address: u16, _value: u8) -> MemoryResult<()> {
        if address >= SIZE as u16 {
            return Err(MemoryError::OutOfBounds);
        }
        Err(MemoryError::ReadOnly)
    }

    fn get_size(&self) -> usize {
        SIZE
    }
}
