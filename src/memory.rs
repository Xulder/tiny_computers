use crate::result::{Result, ResultU16, ResultU8};

#[derive(Debug)]
pub struct Memory<const M: usize> {
    pub data: [u8; M],
}

impl<const M: usize> Memory<M> {
    /// Creates a new memory object with the given size.
    /// This will panic if the size is not a multiple of 2.
    /// The memory is initialized to all zeros.
    pub fn new() -> Memory<M> {
        if M == 0 {
            panic!("Memory size must be greater than 0.")
        }
        if M % 2 != 0 {
            panic!("Memory size must be a multiple of 2.")
        }
        Memory { data: [0; M] }
    }

    /// Used to validate that the given address is in bounds.
    fn validate_address(&self, address: u16) -> Result<()> {
        if address >= M as u16 {
            Err("Memory address out of bounds.".to_string())
        } else {
            Ok(())
        }
    }
    /// Writes a 8-bit value to memory at the given address.
    /// This does use unsafe code to write to memory. The caller must ensure that the address is valid.
    /// This will panic if the address is out of bounds.
    /// ```
    /// use tiny_computers::memory::Memory;
    /// 
    /// let mut mem = Memory::<256>::new();
    /// mem.write_mem_u8(0, 42);
    /// assert_eq!(mem.data[0], 42);
    /// ```
    pub fn write_mem_u8(&mut self, address: u16, value: u8) -> Result<()> {
        self.validate_address(address)?;
        self.data[address as usize] = value;
        Ok(())
    }

    /// Writes two 8-bit values to memory at the given address in big-endian order. (human order)
    /// Used by a few instructions that operate on 16-bit values.
    /// ```
    /// use tiny_computers::memory::Memory;
    /// let mut mem = Memory::<256>::new();
    /// mem.write_mem_u16(0, 0b0000_0001_0000_0010);
    /// assert_eq!(mem.data[0], 1);
    /// assert_eq!(mem.data[1], 2);
    /// ```
    /// 
    pub fn write_mem_u16(&mut self, address: u16, value: u16) -> Result<()> {
        self.validate_address(address)?;
        let high = (value >> 8) as u8;
        let low = value as u8;
        self.data[address as usize] = high;
        self.data[address as usize + 1] = low;
        Ok(())
    }



    /// Reads a 8-bit value from memory at the given address.
    /// ```
    /// use tiny_computers::memory::Memory;
    /// let mut mem = Memory::<256>::new();
    /// mem.data[0] = 42;
    /// assert_eq!(mem.read_mem_u8(0), 42);
    /// ```
    pub fn read_mem_u8(&self, address: u16) -> ResultU8 {
        self.validate_address(address)?;
        Ok(self.data[address as usize])
    }

    /// Reads two 8-bit values from memory at the given address in big-endian order. (human order)
    /// Used by a few instructions that operate on 16-bit values.
    /// ```
    /// use tiny_computers::memory::Memory;
    /// let mut mem = Memory::<256>::new();
    /// mem.data[0] = 1;
    /// mem.data[1] = 2;
    /// assert_eq!(mem.read_mem_u16(0), 258);
    /// ```
    pub fn read_mem_u16(&self, address: u16) -> ResultU16 {
        self.validate_address(address)?;
        let high_byte = self.data[address as usize] as u16;
        let low_byte = self.data[(address + 1) as usize] as u16;
        Ok((high_byte << 8) | low_byte)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_write_read_u8() {
        let mut mem = Memory::<256>::new();
        mem.write_mem_u8(0, 42).unwrap();
        assert_eq!(mem.read_mem_u8(0), Ok(42));
    }

    #[test]
    fn test_write_read_u16() {
        let mut mem = Memory::<256>::new();
        mem.write_mem_u8(0, 1).unwrap();
        mem.write_mem_u8(1, 2).unwrap();
        assert_eq!(mem.read_mem_u16(0), Ok(258));
    }
}
