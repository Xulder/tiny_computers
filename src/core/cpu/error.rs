use crate::core::{isa::InstructionError, memory::MemoryError};
use std::error::Error;
use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Debug, Clone, PartialEq)]
pub enum RegisterError {
    InvalidIndex(u8),
    InvalidValue { register: u8, value: u8 },
    InvalidFlag(u8),
    ReadOnlyRegister(u8),
    AccessError(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum CpuStateError {
    Register(RegisterError),
    Memory(MemoryError),
    Instruction(InstructionError),
    InvalidState(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum CpuError {
    State(CpuStateError),
    Halted,
    InvalidInterrupt(u8),
    StackOverflow,
    StackUnderflow,
    Other(String),
}

// Implement std::error::Error for all error types
impl Error for RegisterError {}
impl Error for CpuStateError {}
impl Error for CpuError {}

// Implement Display for better error messages
impl Display for RegisterError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::InvalidIndex(idx) => write!(f, "invalid register index: {}", idx),
            Self::InvalidValue { register, value } => {
                write!(f, "invalid value {} for register {}", value, register)
            }
            Self::InvalidFlag(flag) => write!(f, "invalid flag: {:#08b}", flag),
            Self::ReadOnlyRegister(reg) => {
                write!(f, "attempted to write to read-only register {}", reg)
            }
            Self::AccessError(msg) => write!(f, "register access error: {}", msg),
        }
    }
}

impl Display for CpuStateError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::Register(e) => write!(f, "register error: {}", e),
            Self::Memory(e) => write!(f, "memory error: {}", e),
            Self::Instruction(e) => write!(f, "instruction error: {}", e),
            Self::InvalidState(msg) => write!(f, "invalid CPU state: {}", msg),
        }
    }
}

impl Display for CpuError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::State(e) => write!(f, "CPU state error: {}", e),
            Self::Halted => write!(f, "CPU halted"),
            Self::InvalidInterrupt(vec) => write!(f, "invalid interrupt vector: {:#04x}", vec),
            Self::StackOverflow => write!(f, "stack overflow"),
            Self::StackUnderflow => write!(f, "stack underflow"),
            Self::Other(msg) => write!(f, "CPU error: {}", msg),
        }
    }
}

// Implement From for convenient error conversion
impl From<RegisterError> for CpuStateError {
    fn from(err: RegisterError) -> Self {
        Self::Register(err)
    }
}

impl From<MemoryError> for CpuStateError {
    fn from(err: MemoryError) -> Self {
        Self::Memory(err)
    }
}

impl From<InstructionError> for CpuStateError {
    fn from(err: InstructionError) -> Self {
        Self::Instruction(err)
    }
}

impl From<CpuStateError> for CpuError {
    fn from(err: CpuStateError) -> Self {
        Self::State(err)
    }
}
