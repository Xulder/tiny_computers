use thiserror::Error;

use crate::memory::error::MemoryError;

#[derive(Debug, Error)]
pub enum CPUError {
    #[error("Invalid Register")]
    InvalidRegister,
    #[error("Memory Error: {0}")]
    MemoryError(MemoryError),
    #[error("Invalid Instruction {0:?}")]
    InvalidInstruction(u8),
    #[error("Unexpected")]
    Unexpected,
}

pub type CPUResult<T> = Result<T, CPUError>;