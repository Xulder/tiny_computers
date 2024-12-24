//! CPU core functionality
//!
//! This module provides traits and types for implementing CPU emulation.

mod error;
mod flags;
mod registers;
mod state;

pub use error::{CpuError, CpuStateError, RegisterError};
pub use flags::FlagsRegister;
pub use registers::RegisterFile;
pub use state::CpuState;

use crate::core::isa::InstructionSet;

/// Represents a complete CPU implementation
pub trait Cpu {
    type Error: From<CpuError> + Into<CpuError>;
    type ISA: InstructionSet<Error = Self::Error>;
    type State: CpuState<Error = Self::Error>;

    fn instruction_set(&self) -> &Self::ISA;
    fn state(&self) -> &Self::State;
    fn state_mut(&mut self) -> &mut Self::State;
    fn reset(&mut self) -> Result<(), Self::Error>;
    fn step(&mut self) -> Result<u8, Self::Error>;
}
