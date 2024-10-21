use crate::hardware::core::devices::memory::{error::MemoryError, MemoryDevice};

pub struct RAM<const SIZE: usize> {
    data: [u8; SIZE],
}

impl<const SIZE: usize> RAM<SIZE> {
    pub fn new() -> RAM<SIZE> {
        RAM { data: [0; SIZE] }
    }
}

impl<const SIZE: usize> MemoryDevice for RAM<SIZE> {
    fn get_size(&self) -> usize {
        SIZE
    }

    fn read_u8(&self, address: u16) -> Result<u8, MemoryError> {
        if address >= SIZE as u16 {
            return Err(MemoryError::OutOfBounds);
        }
        Ok(self.data[address as usize])
    }

    fn write_u8(&mut self, address: u16, value: u8) -> Result<(), MemoryError> {
        self.validate_address(address)?;
        self.data[address as usize] = value;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ram() {
        let mut ram = RAM::<16>::new();

        assert_eq!(ram.read_u8(0).unwrap(), 0);
        ram.write_u8(0, 1).unwrap();

        assert_eq!(ram.read_u8(0).unwrap(), 1);
    }

    #[test]
    fn test_ram_size() {
        assert_eq!(RAM::<16>::new().get_size(), 16);
    }

    #[test]
    fn test_ram_out_of_bounds() {
        let ram = RAM::<16>::new();

        assert_eq!(ram.read_u8(16).unwrap_err(), MemoryError::OutOfBounds);
    }
}