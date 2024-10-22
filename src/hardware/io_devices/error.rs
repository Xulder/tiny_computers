use thiserror::Error;

pub type IOResult<T> = Result<T, IODeviceError>;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum IODeviceError {
    #[error("Out of bounds")]
    OutOfBounds,
    #[error("ReadOnly")]
    ReadOnly,
    #[error("Unexpected")]
    Unexpected,
    #[error("Device returned error code {0}")]
    DeviceError(u8),
}


impl From<IODeviceError> for u8 {
    fn from(error: IODeviceError) -> Self {
        match error {
            IODeviceError::OutOfBounds => 0,
            IODeviceError::ReadOnly => 1,
            IODeviceError::Unexpected => 2,
            IODeviceError::DeviceError(value) => value,
        }
    }
}

impl From<u8> for IODeviceError {
    fn from(value: u8) -> Self {
        match value {
            0 => IODeviceError::OutOfBounds,
            1 => IODeviceError::ReadOnly,
            2 => IODeviceError::Unexpected,
            _ => IODeviceError::DeviceError(value),
        }
    }
}