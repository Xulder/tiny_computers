use super::MemoryError;
use std::fmt::Debug;

/// A trait that defines the interface for memory-like objects.
/// This is versatile and is intended to be used for ROM, RAM, and composite memory systems.
pub trait MemoryDevice: Debug {
    /// The type used for memory addresses
    type Address;
    /// The type used for memory values/data
    type Word: Copy;
    /// The error type returned by memory operations
    type Error;

    /// Reads a value from the specified memory address
    ///
    /// # Arguments
    /// * `address` - The memory address to read from
    ///
    /// # Returns
    /// * `Ok(value)` - The value read from memory
    /// * `Err(error)` - If the read operation failed
    fn read(&self, address: Self::Address) -> Result<Self::Word, Self::Error>;

    /// Writes a value to the specified memory address
    ///
    /// # Arguments
    /// * `address` - The memory address to write to
    /// * `value` - The value to write
    ///
    /// # Returns
    /// * `Ok(())` - If the write was successful
    /// * `Err(error)` - If the write operation failed
    fn write(&mut self, address: Self::Address, value: Self::Word) -> Result<(), Self::Error>;

    /// Resets the memory device to its initial state
    fn reset(&mut self);

    /// Returns the total size of the memory device in bytes
    fn size(&self) -> usize;
}

/// Represents a mapped memory device with its address range and access properties
#[derive(Debug)]
pub struct MappedDevice<A, W, E>
where
    A: Copy + Ord + Debug,
    W: Debug,
    E: From<MemoryError> + Debug,
{
    /// Starting address of the device's memory range
    start_addr: A,
    /// Ending address of the device's memory range (inclusive)
    end_addr: A,
    /// If true, the device is read-only
    write_only: bool,
    /// The actual memory device
    device: Box<dyn MemoryDevice<Address = A, Word = W, Error = E>>,
}

impl<A, W, E> MappedDevice<A, W, E>
where
    A: Copy + Ord + Debug,
    W: Debug,
    E: From<MemoryError> + Debug,
{
    pub fn new(
        start_addr: A,
        end_addr: A,
        write_only: bool,
        device: Box<dyn MemoryDevice<Address = A, Word = W, Error = E>>,
    ) -> Self {
        Self {
            start_addr,
            end_addr,
            write_only,
            device,
        }
    }

    pub fn start_addr(&self) -> A {
        self.start_addr
    }

    pub fn end_addr(&self) -> A {
        self.end_addr
    }

    pub fn write_only(&self) -> bool {
        self.write_only
    }

    /// Consumes the MappedDevice and returns the inner device
    pub fn into_device(self) -> Box<dyn MemoryDevice<Address = A, Word = W, Error = E>> {
        self.device
    }
}

impl<A, W, E> MemoryDevice for MappedDevice<A, W, E>
where
    A: Copy + Ord + Debug,
    W: Copy + Debug,
    E: From<MemoryError> + Debug,
{
    type Address = A;
    type Word = W;
    type Error = E;

    fn read(&self, address: Self::Address) -> Result<Self::Word, Self::Error> {
        if address < self.start_addr || address > self.end_addr {
            return Err(MemoryError::AddressOutOfBounds.into());
        }
        self.device.read(address)
    }

    fn write(&mut self, address: Self::Address, value: Self::Word) -> Result<(), Self::Error> {
        if address < self.start_addr || address > self.end_addr {
            return Err(MemoryError::AddressOutOfBounds.into());
        }
        if self.write_only {
            return Err(MemoryError::ReadOnlyMemory.into());
        }
        self.device.write(address, value)
    }

    fn reset(&mut self) {
        self.device.reset();
    }

    fn size(&self) -> usize {
        self.device.size()
    }
}
