use super::{BoxedMemoryDevice, MemoryDevice};

/// A trait that extends MemoryDevice to provide memory mapping capabilities.
/// This allows for attaching and removing devices at specific address ranges,
/// creating a complete memory bus system.
pub trait MemoryBus: MemoryDevice {
    /// Attaches a memory device to the bus at the specified address range
    ///
    /// # Arguments
    /// * `start_addr` - Starting address of the range
    /// * `end_addr` - Ending address of the range (inclusive)
    /// * `write_only` - If true, the device is read-only
    /// * `device` - The memory device to attach
    ///
    /// # Returns
    /// * `Ok(())` - If the device was successfully attached
    /// * `Err(error)` - If the device could not be attached
    fn attach_device(
        &mut self,
        start_addr: Self::Address,
        end_addr: Self::Address,
        write_only: bool,
        device: BoxedMemoryDevice<Self::Address, Self::Word, Self::Error>,
    ) -> Result<(), Self::Error>;

    /// Removes a device from the bus at the specified starting address
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
    ) -> Option<BoxedMemoryDevice<Self::Address, Self::Word, Self::Error>>;
}
