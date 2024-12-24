use super::error::InstructionError;
use crate::core::cpu::CpuState;

/// Defines how instruction operands are resolved to memory addresses
///
/// This trait handles:
/// - Resolving operands to actual memory addresses
/// - Validating registers and addresses
/// - Determining operand size and format
pub trait AddressingMode {
    type Register;
    type Address;
    type Error: From<InstructionError>;

    fn resolve(&self, cpu: &impl CpuState) -> Result<Self::Address, Self::Error>;
    fn size(&self) -> usize;
    fn format(&self) -> String;

    fn is_valid_register(&self, register: Self::Register) -> bool;
    fn is_valid_address(&self, address: Self::Address) -> bool;
    fn bytes_needed(&self) -> usize;
}
