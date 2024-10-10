

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Mode {
    /// op, reg, (imm8/imm16)
    Immediate = 0b00,
    /// op reg reg
    Register = 0b01,
    /// op reg [reg]
    RegisterIndirect = 0b10,
    /// op reg [reg + imm16]
    RegisterIndexed = 0b11,
    /// op reg [imm16]
    Direct = 0b100,
    /// jmp label|offset|reg|(imm8/imm16)
    Relative = 0b101,
}

impl From<Mode> for u8 {
    /// Converts a `Mode` enum to an `u8` value.
    fn from(value: Mode) -> Self {
        value as u8
    }
}

impl From<u8> for Mode {
    fn from(value: u8) -> Self {
        match value {
            0b00 => Mode::Immediate,
            0b01 => Mode::Register,
            0b10 => Mode::RegisterIndirect,
            0b11 => Mode::RegisterIndexed,
            0b100 => Mode::Direct,
            0b101 => Mode::Relative,
            _ => panic!("Invalid mode value: {}", value),  // or handle error gracefully
        }
    }
}

impl std::fmt::Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Mode::Immediate => write!(f, "Immediate"),
            Mode::Register => write!(f, "Register"),
            Mode::RegisterIndirect => write!(f, "RegisterIndirect"),
            Mode::RegisterIndexed => write!(f, "RegisterIndexed"),
            Mode::Direct => write!(f, "Direct"),
            Mode::Relative => write!(f, "Relative"),
        }
    }
}


// TODO: Rewrite into a `InstructionSet` struct, named something like `TcAsm` or `Z80asm`. Move to `isa::sets`
/// An enum representing all possible instructions.
/// # Immediate
/// `op reg8/reg16 imm8/imm16`
/// 
/// # Register
/// `op reg8/reg16 reg8/reg16`
/// 
/// # RegisterIndirect
/// `op reg8/reg16 [reg16]`
/// 
/// # RegisterIndexed
/// `op reg8/reg16 [reg16 + imm16]`
/// 
/// # Direct
/// `op reg8/reg16 [imm16]`
/// 
/// # Relative
/// `jmp label|offset|reg|(imm8/imm16)`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Instruction {
    /// No operation.
    Nop = 0x00,

    /// Halt the CPU.

    Add = 0x01,

    Sub = 0x02,

    Mul = 0x03,

    Div = 0x04,

    Mod = 0x05,

    And = 0x06,

    Or = 0x07,

    Xor = 0x08,

    Not = 0x09,

    Lsh = 0x0a,

    Rsh = 0x0b,

    Cmp = 0x0c,

    // Jmp stands for "jump". It's used in direct jumps. It expects a label or an address (or a register pair containing an address)
    Jmp = 0x0d,

    // Jmpf stands for "jump forward". It's used in relative jumps.
    Jmpf = 0x0e,

    // FIXME: No good idea.
    Jmpb = 0x0f,

    Call = 0x10,

    Ret = 0x11,

    Push = 0x12,

    Pop = 0x13,

    Write = 0x14,

    Load = 0x15,

    In = 0x16,

    Out = 0x17,

    Halt = 0x1F,

}

impl From<Instruction> for u8 {
    /// Converts an `Opcode` enum to an `u8` value.
    fn from(value: Instruction) -> Self {
        value as u8
    }
}

impl From<u8> for Instruction {
    /// Converts an `u8` value to a `Opcode` enum.
    fn from(value: u8) -> Self {
        unsafe { std::mem::transmute(value) }
    }
}

impl std::fmt::Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Instruction::Nop => write!(f, "nop"),
            Instruction::Add => write!(f, "add"),
            Instruction::Sub => write!(f, "sub"),
            Instruction::Mul => write!(f, "mul"),
            Instruction::Div => write!(f, "div"),
            Instruction::Mod => write!(f, "mod"),
            Instruction::And => write!(f, "and"),
            Instruction::Or => write!(f, "or"),
            Instruction::Xor => write!(f, "xor"),
            Instruction::Not => write!(f, "not"),
            Instruction::Lsh => write!(f, "lsh"),
            Instruction::Rsh => write!(f, "rsh"),
            Instruction::Cmp => write!(f, "cmp"),
            Instruction::Jmp => write!(f, "jmp"),
            Instruction::Jmpf => write!(f, "jmpf"),
            Instruction::Jmpb => write!(f, "jmpb"),
            Instruction::Push => write!(f, "push"),
            Instruction::Pop => write!(f, "pop"),
            Instruction::Call => write!(f, "call"),
            Instruction::Ret => write!(f, "ret"),
            Instruction::Write => write!(f, "store"),
            Instruction::Load => write!(f, "load"),
            Instruction::In => write!(f, "in"),
            Instruction::Out => write!(f, "out"),
            Instruction::Halt => write!(f, "halt"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct Opcode(u8);

impl Opcode {

    pub fn new(instruction: Instruction, mode: Mode) -> Opcode {
        Opcode(((instruction as u8) << 3) | (mode as u8))
    }

    pub fn instruction(&self) -> Instruction {
        Instruction::from(self.0 >> 3)
    }

    pub fn mode(&self) -> Mode {
        Mode::from(self.0 & 0b111)
    }

    pub fn as_u8(&self) -> u8 {
        self.0
    }
}

impl From<Opcode> for u8 {
    fn from(value: Opcode) -> Self {
        value.0
    }
}

impl From<u8> for Opcode {
    fn from(value: u8) -> Self {
        Opcode(value)
    }
}

impl std::fmt::Display for Opcode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.instruction(), self.mode())
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
// }

