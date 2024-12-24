use super::{MappedDevice, MemoryBus, MemoryDevice, MemoryError};
use std::fmt::Debug;

/// A memory mapper that manages multiple devices in different address ranges.
/// It implements both MemoryDevice and MemoryBus traits to provide a complete
/// memory management system.
#[derive(Debug)]
pub struct MemoryMapper<A, V, E>
where
    A: Copy + Ord + Debug,
    V: Debug,
    E: From<MemoryError> + Debug,
{
    /// Vector of mapped devices managed by this mapper
    devices: Vec<MappedDevice<A, V, E>>,
}

/// Implementation of the MemoryDevice trait for MemoryMapper.
/// This allows the mapper to act as a memory device itself, delegating operations
/// to the appropriate mapped device based on the address.
impl<A, W, E> MemoryDevice for MemoryMapper<A, W, E>
where
    A: Copy + Ord + Debug,
    W: Copy + Debug,
    E: From<MemoryError> + Debug,
{
    type Address = A;
    type Word = W;
    type Error = E;

    /// Reads a value from the appropriate mapped device based on the address.
    ///
    /// # Arguments
    /// * `address` - The memory address to read from
    ///
    /// # Returns
    /// * `Ok(value)` - The value read from the mapped device
    /// * `Err(error)` - If no device is mapped to the address
    fn read(&self, address: Self::Address) -> Result<Self::Word, Self::Error> {
        for device in self.devices.iter() {
            if address >= device.start_addr() && address <= device.end_addr() {
                return device.read(address);
            }
        }
        Err(MemoryError::AddressOutOfBounds.into())
    }

    /// Writes a value to the appropriate mapped device based on the address.
    ///
    /// # Arguments
    /// * `address` - The memory address to write to
    /// * `value` - The value to write
    ///
    /// # Returns
    /// * `Ok(())` - If the write was successful
    /// * `Err(error)` - If no device is mapped to the address
    fn write(&mut self, address: Self::Address, value: Self::Word) -> Result<(), Self::Error> {
        for device in self.devices.iter_mut() {
            if address >= device.start_addr() && address <= device.end_addr() {
                return device.write(address, value);
            }
        }
        Err(MemoryError::AddressOutOfBounds.into())
    }

    /// Resets all mapped devices to their initial state
    fn reset(&mut self) {
        for device in self.devices.iter_mut() {
            device.reset();
        }
    }

    /// Returns the total size of all mapped devices in bytes
    fn size(&self) -> usize {
        self.devices.iter().map(|device| device.size()).sum()
    }
}

/// Implementation of the MemoryBus trait for MemoryMapper.
/// This allows devices to be attached to and removed from specific address ranges.
impl<A, W, E> MemoryBus for MemoryMapper<A, W, E>
where
    A: Copy + Ord + Debug,
    W: Copy + Debug,
    E: From<MemoryError> + Debug,
{
    /// Attaches a memory device to the specified address range.
    ///
    /// # Arguments
    /// * `start_addr` - The starting address of the range
    /// * `end_addr` - The ending address of the range
    /// * `write_only` - Whether the device is write-only
    /// * `device` - The memory device to attach
    ///
    /// # Returns
    /// * `Ok(())` - If the device was successfully attached
    /// * `Err(error)` - If the address range overlaps with an existing device
    fn attach_device(
        &mut self,
        start_addr: Self::Address,
        end_addr: Self::Address,
        write_only: bool,
        device: Box<
            dyn MemoryDevice<Address = Self::Address, Word = Self::Word, Error = Self::Error>,
        >,
    ) -> Result<(), Self::Error> {
        if end_addr < start_addr {
            return Err(MemoryError::InvalidAddressRange.into());
        }
        if self.devices.iter().any(|d| {
            (start_addr >= d.start_addr() && start_addr <= d.end_addr())
                || (end_addr >= d.start_addr() && end_addr <= d.end_addr())
        }) {
            return Err(MemoryError::DeviceAlreadyAttached.into());
        }
        self.devices
            .push(MappedDevice::new(start_addr, end_addr, write_only, device));
        Ok(())
    }

    /// Removes and returns the device at the specified starting address.
    ///
    /// # Arguments
    /// * `start_addr` - The starting address of the device to remove
    ///
    /// # Returns
    /// * `Some(device)` - The removed device if found
    /// * `None` - If no device was found at the address
    fn remove_device(
        &mut self,
        start_addr: Self::Address,
    ) -> Option<
        Box<dyn MemoryDevice<Address = Self::Address, Word = Self::Word, Error = Self::Error>>,
    > {
        if let Some(index) = self
            .devices
            .iter()
            .position(|device| device.start_addr() == start_addr)
        {
            Some(self.devices.remove(index).into_device())
        } else {
            None
        }
    }
}

/// Implementation of the Default trait for MemoryMapper.
impl<A, V, E> Default for MemoryMapper<A, V, E>
where
    A: Copy + Ord + Debug,
    V: Debug,
    E: From<MemoryError> + Debug,
{
    /// Creates a new empty MemoryMapper
    fn default() -> Self {
        Self {
            devices: Vec::new(),
        }
    }
}

/// Additional utility methods for MemoryMapper
impl<A, V, E> MemoryMapper<A, V, E>
where
    A: Copy + Ord + Debug,
    V: Debug,
    E: From<MemoryError> + Debug,
{
    /// Creates a new empty MemoryMapper
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the number of attached devices
    pub fn device_count(&self) -> usize {
        self.devices.len()
    }

    /// Returns true if there are no devices attached
    pub fn is_empty(&self) -> bool {
        self.devices.is_empty()
    }

    /// Clears all devices from the mapper
    pub fn clear(&mut self) {
        self.devices.clear();
    }
}
