pub mod error;

use crate::hardware::{
    core::memory_mapper::error::{propagate_result, MemoryMapperError, MemoryMapperResult},
    io_devices::IODevice,
    memory::{error::MemoryError, MemoryDevice},
};

/// A memory mapper for mapping memory regions to memory devices.
pub struct MemoryMapper<M, I, const ROM_SIZE: usize, const RAM_SIZE: usize, const IO_SIZE: usize> {
    /// The ROM device.
    rom: M,
    /// The RAM device.
    ram: M,
    /// The IO devices.
    io: [Option<I>; IO_SIZE],
    /// Whether the ROM is enabled.
    ram_enabled: bool,
    /// Whether the RAM is enabled.
    rom_enabled: bool,
    /// Whether the IO is enabled.
    io_enabled: bool,
}

impl<M, I, const ROM_SIZE: usize, const RAM_SIZE: usize, const IO_SIZE: usize>
    MemoryMapper<M, I, ROM_SIZE, RAM_SIZE, IO_SIZE>
where
    M: MemoryDevice,
    I: IODevice,
{
    /// Creates a new `MemoryMapper` instance.
    pub fn new(
        rom: M,
        ram: M,
        io: [Option<I>; IO_SIZE],
    ) -> MemoryMapper<M, I, ROM_SIZE, RAM_SIZE, IO_SIZE> {
        MemoryMapper {
            rom,
            ram,
            io,
            ram_enabled: false,
            rom_enabled: false,
            io_enabled: false,
        }
    }

    /// Enables or disables the ROM region.
    #[inline(always)]
    pub fn set_rom_enabled(&mut self, enabled: bool) {
        self.rom_enabled = enabled;
    }

    /// Enables or disables the RAM region.
    #[inline(always)]
    pub fn set_ram_enabled(&mut self, enabled: bool) {
        self.ram_enabled = enabled;
    }

    /// Enables or disables the IO region.
    #[inline(always)]
    pub fn set_io_enabled(&mut self, enabled: bool) {
        self.io_enabled = enabled;
    }

    /// Validates that the given address is within the bounds of the memory
    /// region.
    ///
    /// Returns an error if the address is out of bounds or if the memory region
    /// is not enabled.
    fn validate_address(&self, address: u16) -> MemoryMapperResult<()> {
        // Check if the address is within the ROM region
        if address < ROM_SIZE as u16 {
            if !self.rom_enabled {
                return Err(MemoryMapperError::RomNotLoaded);
            }
        }
        // Check if the address is within the RAM region
        else if address < ROM_SIZE as u16 + RAM_SIZE as u16 {
            if !self.ram_enabled {
                return Err(MemoryMapperError::RamNotLoaded);
            }
        }
        // Check if the address is within the IO region
        else if address < ROM_SIZE as u16 + RAM_SIZE as u16 + IO_SIZE as u16 {
            if !self.io_enabled {
                return Err(MemoryMapperError::IoNotLoaded);
            }
        }
        // If the address is not within any of the memory regions, return an error
        else {
            return Err(MemoryMapperError::MemoryError(MemoryError::OutOfBounds));
        }
        Ok(())
    }

    /// Reads a byte from the given address.
    ///
    /// Returns an error if the address is out of bounds or if the memory region
    /// is not enabled.
    pub fn read_u8(&self, address: u16) -> MemoryMapperResult<u8> {
        self.validate_address(address)?;
        match address {
            a if a < ROM_SIZE as u16 => propagate_result(self.rom.read_u8(a), |error| {
                MemoryMapperError::MemoryError(error)
            }),
            a if a < ROM_SIZE as u16 + RAM_SIZE as u16 => {
                propagate_result(self.ram.read_u8(a - ROM_SIZE as u16), |error| {
                    MemoryMapperError::MemoryError(error)
                })
            }
            a if a < ROM_SIZE as u16 + RAM_SIZE as u16 + IO_SIZE as u16 => propagate_result(
                self.io[a as usize - ROM_SIZE - RAM_SIZE]
                    .as_ref()
                    .unwrap()
                    .read_u8(a - ROM_SIZE as u16 - RAM_SIZE as u16),
                |error| MemoryMapperError::IOError(error),
            ),
            _ => Err(MemoryMapperError::MemoryError(MemoryError::OutOfBounds)),
        }
    }

    /// Writes a byte to the given address.
    ///
    /// # Errors
    ///
    /// Returns an error if the address is out of bounds or if the memory region
    /// is not enabled.
    pub fn write_u8(&mut self, address: u16, value: u8) -> MemoryMapperResult<()> {
        self.validate_address(address)?;
        match address {
            // Check if the address is within the ROM region
            a if a < ROM_SIZE as u16 => propagate_result(self.rom.write_u8(a, value), |error| {
                MemoryMapperError::MemoryError(error)
            }),
            // Check if the address is within the RAM region
            a if a < ROM_SIZE as u16 + RAM_SIZE as u16 => {
                propagate_result(self.ram.write_u8(a - ROM_SIZE as u16, value), |error| {
                    MemoryMapperError::MemoryError(error)
                })
            }
            // Check if the address is within the IO region
            a if a < ROM_SIZE as u16 + RAM_SIZE as u16 + IO_SIZE as u16 => propagate_result(
                self.io[a as usize - ROM_SIZE - RAM_SIZE]
                    .as_mut()
                    .unwrap()
                    .write_u8(a - ROM_SIZE as u16 - RAM_SIZE as u16, value),
                |error| MemoryMapperError::IOError(error),
            ),
            // If the address is not within any of the memory regions, return an error
            _ => Err(MemoryMapperError::MemoryError(MemoryError::OutOfBounds)),
        }
    }

    /// Reads a 16-bit big-endian value from the given address.
    ///
    /// # Errors
    ///
    /// Returns an error if the address is out of bounds or if the memory region
    /// is not enabled.
    pub fn read_u16(&self, address: u16) -> MemoryMapperResult<u16> {
        self.validate_address(address)?;
        // Read the high byte
        let high = self.read_u8(address)?;

        // Read the low byte
        let low = self.read_u8(address + 1)?;

        // Combine the bytes
        Ok(u16::from_be_bytes([high, low]))
    }

    /// Writes a 16-bit big-endian value to the given address.
    ///
    /// # Errors
    ///
    /// Returns an error if the address is out of bounds or if the memory region
    /// is not enabled.
    pub fn write_u16(&mut self, address: u16, value: u16) -> MemoryMapperResult<()> {
        self.validate_address(address)?;
        // Read the high byte
        self.write_u8(address, (value >> 8) as u8)?;

        // Read the low byte
        self.write_u8(address + 1, value as u8)?;

        // Success
        Ok(())
    }
}
