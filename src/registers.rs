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
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Register {
    A = 0b000,
    B = 0b001,
    C = 0b010,
    D = 0b011,
    G = 0b110,
    H = 0b101,
    L = 0b100,
    Z = 0b111,
    F = 0b1000, // Flags
    I = 0b1001, // index registers
    X = 0b1010,
    ASwap = 0b1011,
    BSwap = 0b1100,
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
    /// h and l are the high and low bytes of the 16-bit address register.
    pub h: u8,
    pub l: u8,
    pub z: u8,
    /// 2 Index Registers. Only used by certain opcodes, generally loops and jumps.
    pub i: u8,
    pub x: u8,
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
    pub fn get(&self, register: Register) -> u8 {
        match register {
            Register::A => self.a,
            Register::B => self.b,
            Register::C => self.c,
            Register::D => self.d,
            Register::G => self.g,
            Register::H => self.h,
            Register::L => self.l,
            Register::Z => self.z,
            Register::F => self.flags.0,
            _ => unreachable!(),
        }
    }

    #[inline]
    pub fn set(&mut self, register: Register, value: u8) {
        match register {
            Register::A => self.a = value,
            Register::B => self.b = value,
            Register::C => self.c = value,
            Register::D => self.d = value,
            Register::G => self.g = value,
            Register::H => self.h = value,
            Register::L => self.l = value,
            Register::Z => self.z = value,
            Register::F => self.flags.0 = value,
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_set() {
        let mut reg = RegisterFile::new();
        reg.set(Register::A, 0x12);
        assert_eq!(reg.get(Register::A), 0x12);
        reg.set(Register::F, 0x34);
        assert_eq!(reg.get(Register::F), 0x34);
    }
}
