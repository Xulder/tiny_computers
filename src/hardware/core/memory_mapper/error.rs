use thiserror::Error;

use crate::hardware::{io_devices::error::IODeviceError, memory::error::MemoryError};

pub type MemoryMapperResult<T> = Result<T, MemoryMapperError>;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum MemoryMapperError {
    #[error("Rom not loaded")]
    RomNotLoaded,
    #[error("Ram not loaded")]
    RamNotLoaded,
    #[error("Io not loaded")]
    IoNotLoaded,
    #[error("Memory error")]
    MemoryError(#[from] MemoryError),
    #[error("IO error")]
    IOError(#[from] IODeviceError),
}

/// Propagates a result from a memory device, mapping any errors to
/// `MemoryMapperError` using the given function.
pub fn propagate_result<T, E, F>(result: Result<T, E>, map_error: F) -> MemoryMapperResult<T>
where
    F: Fn(E) -> MemoryMapperError,
{
    match result {
        Ok(value) => Ok(value),
        Err(error) => Err(map_error(error)),
    }
}
