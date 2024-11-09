// NOTE: This isa module is designed to work with the `TestCpu` struct for now.
// TODO: Further abstract this into multiple modules.
// Ideally there should be a way to define different ISAs for different CPU types.
// NOTE: ISAs should assume a few things:
// * 8-bit registers
// * 8-bit instructions
// * Register file will always contain at least 2 generic registers: A and B
// * Register file will always contain a flag register
// * CPU will always contain a program counter
pub mod instruction;
pub mod opcode;
pub mod traits;