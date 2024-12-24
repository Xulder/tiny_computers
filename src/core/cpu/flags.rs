use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, Not};

/// Represents CPU flags with bitwise operations
pub trait FlagsRegister:
    Copy
    + BitOr<Output = Self>
    + BitAnd<Output = Self>
    + BitAndAssign
    + BitOrAssign
    + Not<Output = Self>
    + PartialEq
{
    fn get(&self) -> u8;
    fn set(&mut self, value: u8);
    fn update(&mut self, mask: u8, value: u8);
    fn test(&self, mask: u8) -> bool;
}
