//! Memory management system for emulated devices.
//!
//! This module provides traits and implementations for managing memory in emulated systems.
//! It supports memory mapping, device management, and a flexible bus architecture that can
//! accommodate various types of memory devices (ROM, RAM, memory-mapped I/O, etc.).
//!
//! # Key Components
//!
//! - [`MemoryDevice`]: Core trait for any addressable memory device
//! - [`MemoryBus`]: Trait for managing multiple mapped memory devices
//! - [`MemoryMapper`]: Implementation of a memory bus that maps devices to address ranges
//! - [`MappedDevice`]: A memory device with its address range and access properties
//!
//! # Example
//!
//! ```
//! use tiny_computers::core::memory::{MemoryDevice, MemoryBus, MemoryMapper};
//!
//! // Create a new memory mapper
//! let mut mapper = MemoryMapper::<u16, u8, Box<dyn std::error::Error>>::new();
//!
//! // Attach devices to specific address ranges
//! let rom = Box::new(/* some ROM device */);
//! mapper.attach_device(0x0000, 0x1FFF, true, rom);
//!
//! let ram = Box::new(/* some RAM device */);
//! mapper.attach_device(0x2000, 0x3FFF, false, ram);
//! ```
//!
//! # Memory Map Example
//!
//! ```text
//! 0x0000 +-------------+
//!        |     ROM     | Read-only
//! 0x1FFF +-------------+
//! 0x2000 |     RAM     | Read-write
//! 0x3FFF +-------------+
//! ```

mod bus;
mod device;
mod error;
mod mapper;

pub use bus::MemoryBus;
pub use device::{BoxedMemoryDevice, MappedDevice, MemoryDevice};
pub use error::MemoryError;
pub use mapper::MemoryMapper;
