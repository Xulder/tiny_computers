use crate::registers::Register;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AddressingMode {
    Immediate,
    Register,
    RegisterIndirect,
    RegisterIndirectIndexed,
    Direct,
    Relative,
}

impl From<u8> for AddressingMode {
    fn from(value: u8) -> Self {
        match value {
            0b00 => AddressingMode::Immediate,
            0b01 => AddressingMode::Register,
            0b10 => AddressingMode::RegisterIndirect,
            0b11 => AddressingMode::RegisterIndirectIndexed,
            0b100 => AddressingMode::Direct,
            0b101 => AddressingMode::Relative,
            _ => panic!("Invalid addressing mode."),
        }
    }
}

impl From<AddressingMode> for u8 {
    fn from(value: AddressingMode) -> Self {
        match value {
            AddressingMode::Immediate => 0b00,
            AddressingMode::Register => 0b01,
            AddressingMode::RegisterIndirect => 0b10,
            AddressingMode::RegisterIndirectIndexed => 0b11,
            AddressingMode::Direct => 0b100,
            AddressingMode::Relative => 0b101,
        }
    }
}

/// An enum representing all possible opcodes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Opcode {
    Nop,
    Add,
    Sub,
    Mul,
    Div,
    Mov,
    Pop,
    Psh,
    Cmp,
    Jmp,
    Jz,
    Jnz,
    // Wrt,
    // Jl,
    // Jle,
    // Jg,
    // Jge,
    Hlt,
}

impl From<u8> for Opcode {
    fn from(value: u8) -> Self {
        match value {
            0b0000 => Opcode::Nop,
            0b0001 => Opcode::Add,
            0b0010 => Opcode::Sub,
            0b0011 => Opcode::Mul,
            0b0100 => Opcode::Div,
            0b0101 => Opcode::Mov,
            0b0110 => Opcode::Psh,
            0b0111 => Opcode::Pop,
            0b1000 => Opcode::Cmp,
            0b1001 => Opcode::Jmp,
            0b1010 => Opcode::Jz,
            0b1011 => Opcode::Jnz,
            0b1100 => Opcode::Hlt,
            _ => panic!("Invalid opcode."),
        }
    }
}

impl From<Opcode> for u8 {
    fn from(value: Opcode) -> Self {
        match value {
            Opcode::Nop => 0b0000,
            Opcode::Add => 0b0001,
            Opcode::Sub => 0b0010,
            Opcode::Mul => 0b0011,
            Opcode::Div => 0b0100,
            Opcode::Mov => 0b0101,
            Opcode::Psh => 0b0110,
            Opcode::Pop => 0b0111,
            Opcode::Cmp => 0b1000,
            Opcode::Jmp => 0b1001,
            Opcode::Jz => 0b1010,
            Opcode::Jnz => 0b1011,
            Opcode::Hlt => 0b1100,
        }
    }
}

pub fn pack_registers(register1: u8, register2: u8) -> u8 {
    (register1 << 4) | register2
}

pub fn unpack_registers(registers: u8) -> (Register, Register){
    (Register::from((registers >> 4) & 0b1111), Register::from(registers & 0b1111))
}

pub fn encode_opcode(opcode: Opcode, addressing_mode: AddressingMode) -> u8 {
    ((opcode as u8) << 4) | u8::from(addressing_mode)
}

pub fn decode_opcode(opcode: u8) -> (Opcode, AddressingMode) {
    (Opcode::from((opcode as u8) >> 4), AddressingMode::from((opcode as u8) & 0b1111))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_opcode() {
        assert_eq!(encode_opcode(Opcode::Nop, AddressingMode::Immediate), 0b0000_0000);
        assert_eq!(encode_opcode(Opcode::Add, AddressingMode::Immediate), 0b0001_0000);
        assert_eq!(encode_opcode(Opcode::Mul, AddressingMode::RegisterIndirectIndexed), 0b0011_0011);
        assert_eq!(encode_opcode(Opcode::Div, AddressingMode::Direct), 0b0100_0100);
        assert_eq!(encode_opcode(Opcode::Mov, AddressingMode::Relative), 0b0101_0101);
    }

    #[test]
    fn test_decode_opcode() {
        assert_eq!(decode_opcode(0b0011_0011), (Opcode::Mul, AddressingMode::RegisterIndirectIndexed));
        assert_eq!(decode_opcode(0b0100_0100), (Opcode::Div, AddressingMode::Direct));
        assert_eq!(decode_opcode(0b0101_0101), (Opcode::Mov, AddressingMode::Relative));
    }

    #[test]
    fn test_pack_registers() {
        // op B, C
        assert_eq!(pack_registers(0b01, 0b10), 0b0001_0010);
        // op D, C
        assert_eq!(pack_registers(0b11, 0b11), 0b0011_0011);
        // op B, A
        assert_eq!(pack_registers(0b10, 0b00), 0b0010_0000);
        // op A, B
        assert_eq!(pack_registers(0b00, 0b01), 0b0000_0001);
    }

    #[test]
    fn test_unpack_registers() {
        // op B, C
        assert_eq!(unpack_registers(0b0001_0010), (Register::from(0b01), Register::from(0b10)));
        // op D, C
        assert_eq!(unpack_registers(0b0011_0011), (Register::from(0b11), Register::from(0b11)));
        // op B, A
        assert_eq!(unpack_registers(0b0010_0000), (Register::from(0b10), Register::from(0b00)));
        // op A, B
        assert_eq!(unpack_registers(0b0000_0001), (Register::from(0b00), Register::from(0b01)));
    }
}
