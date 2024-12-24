//! Error types for instruction-related operations
//!
//! Defines the various error conditions that can occur during instruction
//! execution, decoding, and address resolution.

use std::fmt::{Display, Formatter, Result as FmtResult};

/// Represents errors that can occur during instruction operations
#[derive(Debug, Clone, PartialEq)]
pub enum InstructionError {
    /// The opcode is not valid for this instruction set
    InvalidOpcode,
    /// The value is not valid for this operation
    InvalidValue,
    /// An addressing error occurred
    AddressingError(AddressingError),
}

/// Represents errors that can occur during address resolution
#[derive(Debug, Clone, PartialEq)]
pub enum AddressingError {
    /// The addressing mode is not valid for this instruction
    InvalidAddressingMode,
    /// The register specified is not valid
    InvalidRegister,
}

impl Display for InstructionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::InvalidOpcode => write!(f, "Invalid opcode"),
            Self::InvalidValue => write!(f, "Invalid value"),
            Self::AddressingError(e) => write!(f, "Addressing error: {}", e),
        }
    }
}

impl Display for AddressingError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::InvalidAddressingMode => write!(f, "Invalid addressing mode"),
            Self::InvalidRegister => write!(f, "Invalid register"),
        }
    }
}

impl From<AddressingError> for InstructionError {
    fn from(error: AddressingError) -> Self {
        Self::AddressingError(error)
    }
}

impl From<InstructionError> for AddressingError {
    fn from(error: InstructionError) -> Self {
        match error {
            InstructionError::AddressingError(e) => e,
            _ => Self::InvalidAddressingMode,
        }
    }
}
