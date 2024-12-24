use super::{error::RegisterError, flags::FlagsRegister};

/// Represents a CPU's register file
pub trait RegisterFile {
    type Index: Copy;
    type Word: Copy;
    type Address: Copy;
    type Flags: FlagsRegister;
    type Error: From<RegisterError>;

    /// Returns the value of a general-purpose register
    ///
    /// # Arguments
    /// * `index` - The register index to read from
    ///
    /// # Panics
    /// Panics if the index is out of bounds
    fn get(&self, index: Self::Index) -> Result<Self::Word, Self::Error>;

    /// Sets the value of a general-purpose register
    ///
    /// # Arguments
    /// * `index` - The register index to write to
    /// * `value` - The value to write
    ///
    /// # Panics
    /// Panics if the index is out of bounds
    fn set(&mut self, index: Self::Index, value: Self::Word) -> Result<(), Self::Error>;

    /// Returns a slice of all general-purpose registers
    fn registers(&self) -> &[Self::Word];

    /// Returns a mutable slice of all general-purpose registers
    fn registers_mut(&mut self) -> &mut [Self::Word];

    /// Returns the number of general-purpose registers
    fn register_count(&self) -> usize;

    /// Returns the current program counter value
    fn program_counter(&self) -> Self::Address;

    /// Sets the program counter to a new value
    fn set_program_counter(&mut self, value: Self::Address);

    /// Returns the current stack pointer value
    fn stack_pointer(&self) -> Self::Address;

    /// Sets the stack pointer to a new value
    fn set_stack_pointer(&mut self, value: Self::Address);

    /// Gets the current value of the flags register
    fn flags(&self) -> Self::Flags;

    /// Updates flags using a mask and new values
    ///
    /// # Arguments
    /// * `mask` - Bits to modify
    /// * `value` - New values for the masked bits
    fn update_flags(&mut self, mask: Self::Flags, value: Self::Flags);

    /// Tests if specific flags are set
    ///
    /// # Arguments
    /// * `mask` - The flags to test
    fn test_flags(&self, mask: Self::Flags) -> bool {
        (self.flags() & mask) == mask
    }

    /// Resets all registers to their initial state
    fn reset(&mut self);
}
