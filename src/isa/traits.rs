use crate::isa::instruction::Instruction;

pub trait ISA {
    fn decode_instruction(&self, instruction: u8) -> Instruction;
    fn encode_instruction(&self, instruction: Instruction) -> u8;
}
