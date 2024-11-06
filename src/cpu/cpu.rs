use crate::cpu::{
    error::{CPUError, CPUResult},
    traits::{CPU, RegisterFile}
};

pub struct TestCPU {
    pub register_file: TestRegisterFile,
    pub sp: u16,
    pub pc: u16,
}

impl TestCPU {
    pub fn new() -> TestCPU {
        TestCPU {
            register_file: TestRegisterFile::new(),
            sp: 0,
            pc: 0,
        }
    }
}

impl CPU<TestRegisterFile> for TestCPU {
    fn get_register_file(&self) -> &TestRegisterFile {
        &self.register_file
    }

    fn get_register_file_mut(&mut self) -> &mut TestRegisterFile {
        &mut self.register_file
    }

    fn reset(&mut self) {
        self.register_file.reset();
        self.sp = 0;
        self.pc = 0;
    }

    fn step(&mut self) {
        self.pc += 1;
        unimplemented!("step not implemented");
    }

    fn stop(&mut self) {
        unimplemented!("stop not implemented");
    }
}

pub struct TestRegisterFile {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,
    pub flags: FlagSet,
}

impl TestRegisterFile {
    pub fn new() -> TestRegisterFile {
        TestRegisterFile {
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            h: 0,
            l: 0,
            flags: FlagSet(0),
        }
    }
}

impl RegisterFile for TestRegisterFile {
    fn reset(&mut self) {
        self.a = 0;
        self.b = 0;
        self.c = 0;
        self.d = 0;
        self.e = 0;
        self.h = 0;
        self.l = 0;
    }

    fn get_reg(&self, register: u8) -> CPUResult<u8>{
        match register {
            0 => Ok(self.a),
            1 => Ok(self.b),
            2 => Ok(self.c),
            3 => Ok(self.d),
            4 => Ok(self.e),
            5 => Ok(self.h),
            6 => Ok(self.l),
            _ => Err(CPUError::InvalidRegister),
        }
    }

    fn set_reg(&mut self, register: u8, value: u8) -> CPUResult<()> {
        match register {
            0 => Ok(self.a = value),
            1 => Ok(self.b = value),
            2 => Ok(self.c = value),
            3 => Ok(self.d = value),
            4 => Ok(self.e = value),
            5 => Ok(self.h = value),
            6 => Ok(self.l = value),
            _ => Err(CPUError::InvalidRegister),
        }
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
/// Negative = 0b0000_0010,
/// Carry = 0b0000_0100,
/// Overflow = 0b0000_1000,
/// Decimal = 0b0001_0000,
/// Condition = 0b0010_0000,
/// Interrupt = 0b0100_0000,
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
