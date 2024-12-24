//! Defines the behavior of CPU instructions
//!
//! The [`Instruction`] trait represents a single CPU instruction and defines how it
//! executes, affects CPU state, and can be displayed.

use crate::core::{cpu::CpuState, memory::MemoryDevice};

use super::{error::AddressingError, AddressingMode, InstructionCodec, InstructionError};

/// Represents a single CPU instruction
///
/// This trait defines the core behavior of a CPU instruction, including:
/// - How it executes and affects CPU/memory state
/// - Number of cycles it takes
/// - Whether it affects CPU flags
/// - How it's displayed in assembly format
pub trait Instruction: InstructionCodec {
    type Opcode;
    type Register;
    type Address;
    type Word: Copy;
    type Error: From<InstructionError>;
    type AddressingMode: AddressingMode<
        Register = Self::Register,
        Address = Self::Address,
        Error = AddressingError,
    >;

    fn execute(
        &self,
        cpu: &mut impl CpuState,
        memory: &mut impl MemoryDevice<
            Address = Self::Address,
            Word = <Self as Instruction>::Word,
            Error = <Self as Instruction>::Error,
        >,
    ) -> Result<u8, <Self as Instruction>::Error>;

    fn cycles(&self) -> u8;
    fn affects_flags(&self) -> bool;
    fn disassemble(&self) -> String;
}
