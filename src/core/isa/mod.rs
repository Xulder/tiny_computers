//! Instruction Set Architecture (ISA) functionality
//!
//! This module provides traits and types for implementing CPU instruction sets.
//! The core components are:
//!
//! - [`InstructionSet`]: Main trait defining a complete instruction set architecture
//! - [`Instruction`]: Trait for individual CPU instructions
//! - [`InstructionCodec`]: Handles encoding/decoding of instructions
//! - [`AddressingMode`]: Defines how instruction operands are resolved
//! - [`InstructionCategory`]: Categorizes types of instructions
//!
//! # Example
//!
//! ```rust,no_run
//! use my_emu::core::isa::{InstructionSet, InstructionCategory};
//!
//! struct MyISA;
//!
//! impl InstructionSet for MyISA {
//!     // Implementation details...
//! #    type Instruction = ();
//! #    type Opcode = u8;
//! #    type Register = u8;
//! #    type Address = u16;
//! #    type Word = u8;
//! #    type Error = ();
//!
//! #    fn name(&self) -> &str { "MyISA" }
//! #    fn word_size(&self) -> u8 { 8 }
//! #    fn address_size(&self) -> u8 { 16 }
//! #    fn register_count(&self) -> usize { 8 }
//! #    fn is_valid_opcode(&self, _: Self::Opcode) -> bool { true }
//! #    fn categorize(&self, _: Self::Opcode) -> InstructionCategory { InstructionCategory::Arithmetic }
//! #    fn opcodes_in_category(&self, _: InstructionCategory) -> Vec<Self::Opcode> { vec![] }
//! }
//! ```

mod addressing;
mod category;
mod codec;
mod error;
mod instruction;

pub use addressing::AddressingMode;
pub use category::InstructionCategory;
pub use codec::InstructionCodec;
pub use error::{AddressingError, InstructionError};
pub use instruction::Instruction;

/// Represents a complete instruction set architecture (ISA)
pub trait InstructionSet {
    type Instruction: Instruction<
            Opcode = Self::Opcode,
            Register = Self::Register,
            Address = Self::Address,
            Word = Self::Word,
            Error = Self::Error,
        > + InstructionCodec;

    /// The instruction's operation code type
    type Opcode;
    /// CPU register identifier type
    type Register;
    /// Memory address type
    type Address;
    /// Data word type for the architecture
    type Word: Copy;
    /// Error type for instruction operations
    type Error: From<InstructionError>;

    fn name(&self) -> &str;
    fn word_size(&self) -> u8;
    fn address_size(&self) -> u8;
    fn register_count(&self) -> usize;
    fn is_valid_opcode(&self, opcode: Self::Opcode) -> bool;
    fn categorize(&self, opcode: Self::Opcode) -> InstructionCategory;
    fn opcodes_in_category(&self, category: InstructionCategory) -> Vec<Self::Opcode>;
}
