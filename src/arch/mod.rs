pub mod chip_8;
pub mod error;

use crate::core::{
    cpu::{Cpu, CpuState},
    isa::InstructionSet,
    memory::MemoryDevice,
};

use error::ArchError;
use std::fmt::Debug;

pub use chip_8::Chip8;

/// Represents a CPU architecture implementation
pub trait Architecture: Debug {
    /// Architecture-specific error type
    type Error: From<ArchError>;
    /// CPU implementation for this architecture
    type CPU: Cpu<Error = Self::Error>;
    /// Instruction set for this architecture
    type ISA: InstructionSet<Error = Self::Error>;
    /// CPU state for this architecture
    type State: CpuState<Error = Self::Error>;
    /// Memory implementation for this architecture
    type Memory: MemoryDevice<Error = Self::Error>;

    /// Returns the name of this architecture
    fn name(&self) -> &str;

    /// Returns the CPU implementation
    fn cpu(&self) -> &Self::CPU;

    /// Returns a mutable reference to the CPU
    fn cpu_mut(&mut self) -> &mut Self::CPU;

    /// Returns the memory implementation
    fn memory(&self) -> &Self::Memory;

    /// Returns a mutable reference to memory
    fn memory_mut(&mut self) -> &mut Self::Memory;

    /// Resets the architecture to its initial state
    fn reset(&mut self) -> Result<(), Self::Error>;

    /// Executes one instruction
    /// Returns the number of cycles taken
    fn step(&mut self) -> Result<u8, Self::Error>;

    /// Returns the current state
    fn state(&self) -> &Self::State;

    /// Returns the instruction set
    fn instruction_set(&self) -> &Self::ISA;
}
