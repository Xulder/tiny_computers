use crate::hardware::memory::{MemoryDevice, MemoryError, MemoryResult};

pub struct ROM<const SIZE: usize> {
    data: [u8; SIZE],
}

impl<const SIZE: usize> ROM<SIZE> {
    pub fn new(data: [u8; SIZE]) -> ROM<SIZE> {
        ROM { data }
    }
}

impl<const SIZE: usize> MemoryDevice for ROM<SIZE> {
    fn get_size(&self) -> usize {
        SIZE
    }

    fn read_u8(&self, address: u16) -> MemoryResult<u8> {
        if address >= SIZE as u16 {
            return Err(MemoryError::OutOfBounds);
        }
        Ok(self.data[address as usize])
    }

    fn write_u8(&mut self, _address: u16, _value: u8) -> MemoryResult<()> {
        Err(MemoryError::ReadOnly)
    }

    fn write_u16(&mut self, _address: u16, _value: u16) -> MemoryResult<()> {
        Err(MemoryError::ReadOnly)
    }
}
