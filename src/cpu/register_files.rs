use crate::cpu::{
    error::{CPUError, CPUResult},
    traits::RegisterFile,
};

pub struct TestRegisterFile {
    pub registers: [u8; 8],
    pub flags: FlagSet,
}

impl TestRegisterFile {
    pub fn new() -> TestRegisterFile {
        TestRegisterFile {
            registers: [0; 8],
            flags: FlagSet(0),
        }
    }
}

impl RegisterFile for TestRegisterFile {
    fn reset(&mut self) {
        self.registers = [0; 8];
        self.flags = FlagSet(0);
    }

    fn get_reg(&self, register: u8) -> CPUResult<u8> {
        self.registers
            .get(register as usize)
            .copied()
            .ok_or(CPUError::InvalidRegister)
    }

    fn set_reg(&mut self, register: u8, value: u8) -> CPUResult<()> {
        self.registers[register as usize] = value;
        Ok(())
    }

    fn get_flags(&self) -> CPUResult<u8> {
        Ok(self.flags.0)
    }

    fn set_flags(&mut self, value: u8) -> CPUResult<()> {
        self.flags = FlagSet(value);
        Ok(())
    }
}

/// Zero = 0b0000_0001,
///
/// Negative = 0b0000_0010,
///
/// Carry = 0b0000_0100,
///
/// Overflow = 0b0000_1000,
///
/// Decimal = 0b0001_0000,
///
/// Condition = 0b0010_0000,
///
/// Interrupt = 0b0100_0000
///
/// FlagSet is a bit mask of flags. It is used to represent
/// the flags of the CPU.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct FlagSet(pub u8);

impl FlagSet {
    pub const ZERO: Self = Self(0b0000_0001);
    pub const NEGATIVE: Self = Self(0b0000_0010);
    pub const CARRY: Self = Self(0b0000_0100);
    pub const OVERFLOW: Self = Self(0b0000_1000);
    pub const DECIMAL: Self = Self(0b0001_0000);
    pub const CONDITION: Self = Self(0b0010_0000);

    #[inline]
    pub fn contains(self, other: Self) -> bool {
        self.0 & other.0 != 0
    }

    #[inline]
    pub fn is_clear(self) -> bool {
        self.0 == 0
    }

    #[inline]
    pub fn clear(self) -> Self {
        Self(0b0000_0000)
    }

    #[inline]
    pub fn insert(&mut self, other: Self) {
        self.0 |= other.0;
    }

    #[inline]
    pub fn remove(&mut self, other: Self) {
        self.0 &= !other.0;
    }
}
