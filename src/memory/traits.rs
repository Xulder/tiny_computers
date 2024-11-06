use crate::memory::error::MemoryResult;

pub trait Memory {
    fn read_u8(&self, address: u16) -> MemoryResult<u8>;
    fn write_u8(&mut self, address: u16, value: u8) -> MemoryResult<()>;
    fn get_size(&self) -> usize;
}