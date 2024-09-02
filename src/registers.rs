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

// TODO: Add variants for register pairs
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
    BC = 0b1000,
    DG = 0b1001,
    HL = 0b1010,
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

pub fn pack_registers(regr1: u8, regr2: u8) -> u8 {
    (regr1 << 4) | regr2
}

pub fn unpack_registers(value: u8) -> (u8, u8) {
    (value >> 4, value & 0b1111)
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
    /// h and l are the high and low bytes of the 16-bit address register.
    pub h: u8,
    pub l: u8,
    /// 2 Swap Registers for a and b
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


    #[inline]
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


    #[inline]
    pub fn get_reg_pair(&self, register: Register) -> u16 {
        match register {
            Register::BC => u16::from_be_bytes([self.b, self.c]), 
            Register::DG => u16::from_be_bytes([self.d, self.g]), 
            Register::HL => u16::from_be_bytes([self.h, self.l]), 
            _ => unreachable!(),
        }
    }

    #[inline]
    pub fn set_reg_pair(&mut self, register: Register, value: u16) {
        // Big Endian
        let [high, low] = value.to_be_bytes();
        match register {
            Register::BC => {
                self.b = high;
                self.c = low;
            }
            Register::DG => {
                self.d = high;
                self.g = low;
            }
            Register::HL => {
                self.h = high;
                self.l = low;
            }
            _ => unreachable!(),
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pack_unpack() {
        assert_eq!(pack_registers(0b0101, 0b0100), 0b0101_0100);
        assert_eq!(unpack_registers(0b0101_0100), (0b0101, 0b0100));
    }

    #[test]
    fn test_get_set() {
        let mut reg = RegisterFile::new();

        reg.set_reg(Register::A, 0b0101);

        assert_eq!(reg.get_reg(Register::A), 0b0101);
    }

    #[test]
    fn test_get_set_pair() {
        let mut reg = RegisterFile::new();

        reg.set_reg_pair(Register::BC, 0b0101_0100_1111_1111);

        assert_eq!(reg.get_reg_pair(Register::BC), 0b0101_0100_1111_1111);
        assert_eq!(reg.get_reg(Register::B), 0b0101_0100);
        assert_eq!(reg.get_reg(Register::C), 0b1111_1111);
    }

}

