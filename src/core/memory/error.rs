use std::fmt::{Debug, Display, Formatter, Result as FmtResult};

/// Represents errors that can occur during memory operations
#[derive(Debug, Clone, Copy)]
pub enum MemoryError {
    /// Attempted to access an address outside the valid range
    AddressOutOfBounds,
    /// Attempted to write to read-only memory
    ReadOnlyMemory,
    /// Attempted to attach a device to an address range that overlaps with an existing device
    DeviceAlreadyAttached,
    /// Attempted to remove a device that doesn't exist
    DeviceNotFound,
    /// Attempted to attach a device to an invalid address range
    InvalidAddressRange,
    /// For more specific access control errors
    DeviceAccessViolation,
    /// For general bus-related issues
    BusError,
}

impl Display for MemoryError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "Memory error: {:?}", self)
    }
}
