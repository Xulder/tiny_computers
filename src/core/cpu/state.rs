use super::error::CpuStateError;
use crate::core::memory::MemoryDevice;

/// Represents a CPU's execution state
pub trait CpuState {
    type Register;
    type Address;
    type Word: Copy;
    type Error: From<CpuStateError>;
    type Memory: MemoryDevice<Address = Self::Address, Word = Self::Word, Error = Self::Error>;

    fn read_register(&self, reg: Self::Register) -> Result<Self::Word, Self::Error>;
    fn write_register(&mut self, reg: Self::Register, value: Self::Word)
        -> Result<(), Self::Error>;
    fn get_program_counter(&self) -> Self::Address;
    fn set_program_counter(&mut self, addr: Self::Address) -> Result<(), Self::Error>;
    fn get_stack_pointer(&self) -> Self::Address;
    fn set_stack_pointer(&mut self, addr: Self::Address) -> Result<(), Self::Error>;
    fn get_flags(&self) -> u8;
    fn set_flags(&mut self, flags: u8) -> Result<(), Self::Error>;
    fn test_flag(&self, flag: u8) -> Result<bool, Self::Error>;
    fn memory(&self) -> &Self::Memory;
    fn memory_mut(&mut self) -> &mut Self::Memory;
    fn cycles(&self) -> u64;
    fn add_cycles(&mut self, cycles: u8);
}
