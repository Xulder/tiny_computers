use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum MemoryError {
    #[error("Out of bounds")]
    OutOfBounds,
    #[error("ReadOnly")]
    ReadOnly,
    #[error("WriteOnly")]
    WriteOnly,
    #[error("InvalidAddress {0}")]
    InvalidAddress(u16),
    #[error("InvalidValue {0}")]
    InvalidValue(u8),
    #[error("Unexpected error {0}")]
    Unexpected(u16),
}

impl From<MemoryError> for u8 {
    fn from(error: MemoryError) -> Self {
        match error {
            MemoryError::OutOfBounds => 0,
            MemoryError::ReadOnly => 1,
            MemoryError::WriteOnly => 2,
            MemoryError::InvalidAddress(_) => 3,
            MemoryError::InvalidValue(_) => 4,
            MemoryError::Unexpected(_) => 5,
        }
    }
}

impl From<u8> for MemoryError {
    fn from(value: u8) -> Self {
        match value {
            0 => MemoryError::OutOfBounds,
            1 => MemoryError::ReadOnly,
            2 => MemoryError::WriteOnly,
            3 => MemoryError::InvalidAddress(0),
            4 => MemoryError::InvalidValue(0),
            _ => MemoryError::Unexpected(0),
        }
    }
}