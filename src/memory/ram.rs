use crate::memory::{
    error::{MemoryError, MemoryResult},
    traits::Memory,
};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct RAM {
    data: Box<[u8]>, // in bytes
}

impl RAM {
    pub fn new(size: usize) -> RAM {
        let data = vec![0; size].into_boxed_slice();
        RAM { data }
    }

    pub fn new_with_data(data: Vec<u8>) -> RAM {
        RAM {
            data: data.into_boxed_slice(),
        }
    }
}

impl Memory for RAM {
    fn read_u8(&self, address: u16) -> MemoryResult<u8> {
        self.data
            .get(address as usize)
            .cloned()
            .ok_or(MemoryError::OutOfBounds)
    }

    fn write_u8(&mut self, address: u16, value: u8) -> MemoryResult<()> {
        self.data
            .get_mut(address as usize)
            .map(|v| *v = value)
            .ok_or(MemoryError::OutOfBounds)
    }

    fn get_size(&self) -> usize {
        self.data.len()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ram() {
        let mut ram = RAM::new(16);

        assert_eq!(ram.read_u8(0).unwrap(), 0);
        ram.write_u8(0, 1).unwrap();

        assert_eq!(ram.read_u8(0).unwrap(), 1);
    }

    #[test]
    fn test_ram_size() {
        assert_eq!(RAM::new(16).get_size(), 16);
    }

    #[test]
    fn test_ram_out_of_bounds() {
        let ram = RAM::new(16);

        assert_eq!(ram.read_u8(16).unwrap_err(), MemoryError::OutOfBounds);
    }
}
