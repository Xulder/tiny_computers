


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Opcode {
    Nop = 0,
    Add = 1,
    Sub = 2,
    Mul = 3,
    Div = 4,
    Cmp = 5,
    Ld = 6,
    St = 7,
    Jmp = 8,
    Jz = 9,
    Jnz = 10,
    Hlt = 11,
}

impl Opcode {
    pub fn to_u8(self) -> u8 {
        self as u8
    }
}

impl From<u8> for Opcode {
    fn from(byte: u8) -> Self {
        match byte {
            0 => Opcode::Nop,
            1 => Opcode::Add,
            2 => Opcode::Sub,
            3 => Opcode::Mul,
            4 => Opcode::Div,
            5 => Opcode::Cmp,
            6 => Opcode::Ld,
            7 => Opcode::St,
            8 => Opcode::Jmp,
            9 => Opcode::Jz,
            10 => Opcode::Jnz,
            11 => Opcode::Hlt,
            _ => panic!("Invalid instruction code: {}", byte),
        }
    }
}

impl From<Opcode> for u8 {
    fn from(instruction: Opcode) -> Self {
        instruction.to_u8()
    }
}
