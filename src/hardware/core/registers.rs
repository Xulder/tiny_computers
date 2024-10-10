#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct FlagSet(u8);

impl FlagSet {
    /// Zero flag.
    pub const ZERO: Self = Self(1 << 0);
    /// Negative flag.
    pub const NEGATIVE: Self = Self(1 << 1);
    /// Carry flag.
    pub const CARRY: Self = Self(1 << 2);
    /// Overflow flag.
    pub const OVERFLOW: Self = Self(1 << 3);
    /// Interrupt flag.
    pub const INTERRUPT: Self = Self(1 << 4);
    /// Decimal flag.
    pub const DECIMAL: Self = Self(1 << 5);
    /// Condition flag.
    pub const CONDITION: Self = Self(1 << 6);

    /// Check if the given flag is set.
    #[inline]
    pub fn contains(self, other: Self) -> bool {
        (self.0 & other.0) != 0
    }

    #[inline]
    pub fn is_clear(self) -> bool {
        self.0 == 0
    }

    #[inline]
    pub fn clear(self) -> Self {
        Self(0)
    }

    /// Set the given flag.
    #[inline]
    pub fn insert(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }

    /// Clear the given flag.
    #[inline]
    pub fn remove(self, other: Self) -> Self {
        Self(self.0 & !other.0)
    }
}

/// Represents a register of the Tiny Computer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum Register {
    /// General purpose registers
    A = 0b000,
    B = 0b001,
    C = 0b010,
    D = 0b011,
    G = 0b100,
    /// H and L are the high and low byte of the HL register pair. Used by some instructions.
    H = 0b101, // High byte
    L = 0b110, // Low byte
    Z = 0b111,

    /// Register Pairs
    AB = 0b1000,
    CD = 0b1001,
    GZ = 0b1010,
    HL = 0b1011,
}

impl Register {
    pub fn is_pair(&self) -> bool {
        matches!(self, Self::AB | Self::CD | Self::GZ | Self::HL)
    }
    
    pub fn as_u8(self) -> u8 {
        u8::from(self)
    }
}

impl From<u8> for Register {
    /// Converts an `u8` value to a `Register` enum.
    fn from(value: u8) -> Self {
        unsafe { std::mem::transmute(value) }
    }
}

impl From<Register> for u8 {
    /// Converts a `Register` enum to an `u8` value.
    fn from(value: Register) -> Self {
        value as u8
    }
}

pub fn pack_registers(regr1: Register, regr2: Register) -> u8 {
    (u8::from(regr1) << 4) | u8::from(regr2)
}

pub fn unpack_registers(value: u8) -> (Register, Register) {
    let regr1 = Register::from((value >> 4) & 0b1111);
    let regr2 = Register::from(value & 0b1111);
    (regr1, regr2)
}


/// Represents the registers of the Tiny Computer.
#[repr(C)]
#[derive(Default, Debug, Clone, Copy)]
pub struct RegisterFile {
    /// Program counter stores the address of the next instruction to be executed.
    pub pc: u16,
    /// Stack pointer stores the address of the next free location on the stack.
    pub sp: u16,
    /// 8 General Purpose Registers.
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub g: u8,
    pub z: u8,
    /// h and l are the high and low bytes of the HL register pair. These work the same way as the general purpose registers.
    pub h: u8,
    pub l: u8,
    /// 2 Swap Registers for a and b
    /// NOTE: These can only be used by the SWAP instruction. That's why there is no `a_swap` and `b_swap` variants in the Register enum.
    pub a_swap: u8,
    pub b_swap: u8,
    /// Flags Register.
    pub flags: FlagSet,
}

impl RegisterFile {
    pub fn new() -> RegisterFile {
        RegisterFile {
            ..Default::default()
        }
    }

    pub fn reset(&mut self) {
        *self = RegisterFile::new();
    }

    #[inline]
    pub fn get_reg(&self, register: Register) -> u8 {
        match register {
            Register::A => self.a,
            Register::B => self.b,
            Register::C => self.c,
            Register::D => self.d,
            Register::G => self.g,
            Register::H => self.h,
            Register::L => self.l,
            Register::Z => self.z,
            _ => unreachable!(),
        }
    }


    pub fn set_reg(&mut self, register: Register, value: u8) {
        match register {
            Register::A => self.a = value,
            Register::B => self.b = value,
            Register::C => self.c = value,
            Register::D => self.d = value,
            Register::G => self.g = value,
            Register::H => self.h = value,
            Register::L => self.l = value,
            Register::Z => self.z = value,
            _ => unreachable!(),
        }
    }


    pub fn get_reg_pair(&self, register: Register) -> u16 {
        let [high, low] = match register {
            Register::AB => [self.a, self.b],
            Register::CD => [self.c, self.d],
            Register::GZ => [self.g, self.z],
            Register::HL => [self.h, self.l],
            _ => unreachable!(),
        };
        u16::from_be_bytes([high, low])
    }

    pub fn set_reg_pair(&mut self, register: Register, value: u16) {
        // Big Endian
        let [high, low] = value.to_be_bytes();
        match register {
            Register::AB => {
                self.a = high;
                self.b = low;
            }
            Register::CD => {
                self.c = high;
                self.d = low;
            }
            Register::GZ => {
                self.g = high;
                self.z = low;
            }
            Register::HL => {
                self.h = high;
                self.l = low;
            }
            _ => unreachable!(),
        }
    }

    pub fn swap_a_or_b(&mut self, register: Register) {
        match register {
            Register::A => {
                std::mem::swap(&mut self.a, &mut self.a_swap);
            }
            Register::B => {
                std::mem::swap(&mut self.b, &mut self.b_swap);
            }
            _ => unreachable!(),
        }
    }

    pub fn swap_a_and_b(&mut self) {
        std::mem::swap(&mut self.a, &mut self.a_swap);
        std::mem::swap(&mut self.b, &mut self.b_swap);
    }

    pub fn get_flags(&self) -> u8 {
        self.flags.0
    }

    pub fn set_flags(&mut self, value: u8) {
        self.flags.0 = value;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pack_unpack() {
        assert_eq!(pack_registers(Register::H, Register::G), 0b0101_0100);
        assert_eq!(unpack_registers(0b0101_0100), (Register::from(0b0101), Register::from(0b0100)));
    }

    #[test]
    fn test_get_set() {
        let mut reg = RegisterFile::new();

        // general registers
        for i in 0..8 {
            reg.set_reg(Register::from(i), 0b0101);
            assert_eq!(reg.get_reg(Register::from(i)), 0b0101);
        }

        // register pairs
        for i in 0..3 {
            reg.set_reg_pair(Register::from(i + 8), 0b0000_0000_0101_0100);
            assert_eq!(reg.get_reg_pair(Register::from(i + 8)), 0b0000_0000_0101_0100);
        }
    }

    #[test]
    fn test_flags() {
        let mut reg = RegisterFile::new();
        assert_eq!(reg.get_flags(), 0b0000_0000);

        reg.set_flags(0b1111_1111);
        assert_eq!(reg.get_flags(), 0b1111_1111);

        reg.set_flags(0b0000_0000);
        assert_eq!(reg.get_flags(), 0b0000_0000);

        reg.set_flags(0b1010_1010);
        assert_eq!(reg.get_flags(), 0b1010_1010);

        reg.set_flags(0b1111_1111);
        assert_eq!(reg.get_flags(), 0b1111_1111);

        reg.set_flags(0b0000_0000);
        assert_eq!(reg.get_flags(), 0b0000_0000);
    }
}

