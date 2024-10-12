/// Represents a register file of the Tiny Computer.
pub trait RegisterFile {
    fn reset(&mut self);

    fn get_reg(&self, register: u8) -> u8;

    fn set_reg(&mut self, register: u8, value: u8);

    fn get_flags(&self) -> u8;

    fn get_mut_flags(&mut self) -> &mut u8;

    fn set_flags(&mut self, value: u8);
}

/// Zero = 0b0000_0000,
/// Negative = 0b0000_0010,
/// Carry = 0b0000_0100,
/// Overflow = 0b0000_1000,
/// Decimal = 0b0001_0000,
/// Condition = 0b0010_0000,
/// Interrupt = 0b0100_0000,
#[repr(transparent)]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
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
    /// Decimal flag.
    pub const DECIMAL: Self = Self(1 << 4);
    /// Condition flag.
    pub const CONDITION: Self = Self(1 << 5);
    /// Interrupt flag.
    pub const INTERRUPT: Self = Self(1 << 6);

    #[inline(always)]
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
    #[inline(always)]
    pub fn insert(&mut self, other: Self) {
        if !self.contains(other) {
            self.0 = self.0 | other.0;
        }
    }

    /// Clear the given flag.
    #[inline(always)]
    pub fn remove(&mut self, other: Self) {
        if self.contains(other) {
            self.0 = self.0 & !other.0;
        }
    }
}