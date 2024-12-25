use crate::core::{cpu::CpuError, isa::InstructionError, memory::MemoryError};
use std::error::Error;
use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Debug, Clone, PartialEq)]
pub enum ArchError {
    /// CPU-related errors
    Cpu(CpuError),

    /// Memory-related errors
    Memory(MemoryError),

    /// Instruction-related errors
    Instruction(InstructionError),

    /// Architecture-specific errors
    InvalidState(String),
    UnsupportedFeature(String),
    TimingViolation,
}

impl Display for ArchError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::Cpu(err) => write!(f, "CPU error: {}", err),
            Self::Memory(err) => write!(f, "Memory error: {}", err),
            Self::Instruction(err) => write!(f, "Instruction error: {}", err),
            Self::InvalidState(msg) => write!(f, "Invalid state: {}", msg),
            Self::UnsupportedFeature(feature) => write!(f, "Unsupported feature: {}", feature),
            Self::TimingViolation => write!(f, "Timing violation"),
        }
    }
}

impl Error for ArchError {}

// Implement From for core error types
impl From<CpuError> for ArchError {
    fn from(err: CpuError) -> Self {
        Self::Cpu(err)
    }
}

impl From<MemoryError> for ArchError {
    fn from(err: MemoryError) -> Self {
        Self::Memory(err)
    }
}

impl From<InstructionError> for ArchError {
    fn from(err: InstructionError) -> Self {
        Self::Instruction(err)
    }
}
