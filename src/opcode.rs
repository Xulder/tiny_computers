/// An enum representing all possible opcodes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Opcode {
    // TODO: change to hex format
    /// No operation.
    Nop = 0x00,

    /// add a, 2 -> adds 2 to register a
    AddImm8 = 0x01,
    AddImm16 = 0x02,
    AddRegDir = 0x03,
    AddReg = 0x04,
    AddDir = 0x05,

    /// sub a, 2 -> subtracts 2 from register a
    SubImm8 = 0x06,
    /// sub ab, [0x1111] -> subtracts 0x1111 from registers a and b treated as a big endian pair.
    SubImm16 = 0x07,
    /// sub a, [bc] -> subtracts the value at address contained in bc from register a
    SubRegDir = 0x08,
    /// sub a, [bc + 1] -> subtracts the value at address contained in bc plus 1 from register a
    SubRegInd = 0x09,
    /// sub a, [x05] -> subtracts the value at address [x05] from register a
    SubDir = 0x0A,

    /// mul a, 2 -> multiplies register a by 2
    MulImm8 = 0x0B,
    /// mul ab, [0x1111] -> multiplies registers a and b treated as a big endian pair by 0x1111
    MulImm16 = 0x0C,
    /// mul a, [bc] -> multiplies register a by the value at address contained in bc
    MulRegDir = 0x0D,
    /// mul a, [bc + 1] -> multiplies register a by the value at address contained in bc plus 1
    MulRegInd = 0x0E,
    /// mul a, [x05] -> multiplies register a by the value at address [x05]
    MulDir = 0x0F,

    /// div a, 2 -> divides register a by 2
    DivImm8 = 0x10,
    /// div ab, [0x1111] -> divides registers a and b treated as a big endian pair by 0x1111
    DivImm16 = 0x11,
    /// div a, [bc] -> divides register a by the value at address contained in bc
    DivRegDir = 0x12,
    /// div a, [bc + 1] -> divides register a by the value at address contained in bc plus 1
    DivRegInd = 0x13,
    /// div a, [x05] -> divides register a by the value at address [x05]
    DivDir = 0x14,

    /// mov a, 2 -> moves the value 2 to register a
    MovImm8 = 0x15,
    /// mov a, [bc] -> moves the value at address contained in bc to register a
    MovRegDir = 0x16,
    /// mov a, [bc + 1] -> moves the value at address contained in bc plus 1 to register a
    MovRegInd = 0x17,
    /// mov a, [x05] -> moves the value at address [x05] to register a
    MovDir = 0x18,

    /// psh a, 2 -> pushes the value 2 to the stack pointed to by register a
    PshImm8 = 0x19,
    /// psh a, [bc] -> pushes the value at address contained in bc to the stack pointed to by register a
    PshRegDir = 0x1A,
    /// psh a, [bc + 1] -> pushes the value at address contained in bc plus 1 to the stack pointed to by register a
    PshRegInd = 0x1B,
    /// psh a, [x05] -> pushes the value at address [x05] to the stack pointed to by register a
    PshDir = 0x1C,

    /// pop a -> pops the top value off the stack and moves it to register a
    Pop = 0x1D,

    /// wrt a, 2 -> writes the value 2 to the memory at the address contained in register a
    WrtImm8 = 0x1E,
    /// wrt a, [bc] -> writes the value at address contained in bc to the memory at the address contained in register a
    WrtRegDir = 0x1F,
    /// wrt a, [bc + 1] -> writes the value at address contained in bc plus 1 to the memory at the address contained in register a
    WrtRegInd = 0x20,

    /// cmp a, 2 -> compares the value 2 to register a
    CmpImm8 = 0x21,
    /// cmp a, [bc] -> compares the value at address contained in bc to register a
    CmpRegDir = 0x22,
    /// cmp a, [bc + 1] -> compares the value at address contained in bc plus 1 to register a
    CmpRegInd = 0x23,
    /// cmp a, [x05] -> compares the value at address [x05] to register a
    CmpDir = 0x24,

    /// jmp [x05] -> jumps to the address [x05]
    Jmp = 0x25,
    /// jz [x05] -> jumps to the address [x05] if the zero flag is set
    Jz = 0x26,
    /// jnz [x05] -> jumps to the address [x05] if the zero flag is not set
    Jnz = 0x27,

    /// hlt -> halts the processor
    Hlt = 0x28,
}

impl From<u8> for Opcode {
    fn from(value: u8) -> Self {
        match value {
            0x00 => Opcode::Nop,
            0x01 => Opcode::AddImm8,
            0x02 => Opcode::AddImm16,
            0x03 => Opcode::AddRegDir,
            0x04 => Opcode::AddReg,
            0x05 => Opcode::AddDir,
            0x06 => Opcode::SubImm8,
            0x07 => Opcode::SubImm16,
            0x08 => Opcode::SubRegDir,
            0x09 => Opcode::SubRegInd,
            0x0A => Opcode::SubDir,
            0x0B => Opcode::MulImm8,
            0x0C => Opcode::MulImm16,
            0x0D => Opcode::MulRegDir,
            0x0E => Opcode::MulRegInd,
            0x0F => Opcode::MulDir,
            0x10 => Opcode::DivImm8,
            0x11 => Opcode::DivImm16,
            0x12 => Opcode::DivRegDir,
            0x13 => Opcode::DivRegInd,
            0x14 => Opcode::DivDir,
            0x15 => Opcode::MovImm8,
            0x16 => Opcode::MovRegDir,
            0x17 => Opcode::MovRegInd,
            0x18 => Opcode::MovDir,
            0x19 => Opcode::PshImm8,
            0x1A => Opcode::PshRegDir,
            0x1B => Opcode::PshRegInd,
            0x1C => Opcode::PshDir,
            0x1D => Opcode::Pop,
            0x1E => Opcode::WrtImm8,
            0x1F => Opcode::WrtRegDir,
            0x20 => Opcode::WrtRegInd,
            0x21 => Opcode::CmpImm8,
            0x22 => Opcode::CmpRegDir,
            0x23 => Opcode::CmpRegInd,
            0x24 => Opcode::CmpDir,
            0x25 => Opcode::Jmp,
            0x26 => Opcode::Jz,
            0x27 => Opcode::Jnz,
            0x28 => Opcode::Hlt,
            _ => panic!("Invalid opcode."),
        }
    }
}

impl From<Opcode> for u8 {
    fn from(value: Opcode) -> Self {
        value as u8
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
// }


