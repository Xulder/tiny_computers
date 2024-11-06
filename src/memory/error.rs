use thiserror::Error;

pub type MemoryResult<T> = Result<T, MemoryError>;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum MemoryError {
    #[error("Out of bounds")]
    OutOfBounds,
    #[error("ReadOnly")]
    ReadOnly,
    #[error("Unexpected")]
    Unexpected,
}

impl From<MemoryError> for u8 {
    fn from(error: MemoryError) -> Self {
        match error {
            MemoryError::OutOfBounds => 0,
            MemoryError::ReadOnly => 1,
            MemoryError::Unexpected => 2,
        }
    }
}

impl From<u8> for MemoryError {
    fn from(value: u8) -> Self {
        match value {
            0 => MemoryError::OutOfBounds,
            1 => MemoryError::ReadOnly,
            _ => MemoryError::Unexpected,
        }
    }
}