use core::panic;

use crate::memory::Memory;
use crate::opcode::Opcode;
use crate::registers::{pack_registers, unpack_registers, Register, RegisterFile};

/// The CPU of the Tiny Computer
#[derive(Debug)]
pub struct Cpu<const M: usize> {
    /// The registers of the CPU
    pub registers: RegisterFile,
}

impl<const M: usize> Cpu<M> {
    /// Creates a new instance of the CPU
    pub fn new() -> Cpu<M> {
        Cpu {
            registers: RegisterFile {
                sp: (M as u16) - 1,
                ..Default::default()
            },
        }
    }

    /// Resets the CPU to its default state
    #[inline]
    pub fn reset(&mut self) {
        self.registers = RegisterFile::new();
    }
}

/// Memory operations
impl<const M: usize> Cpu<M> {
    #[inline]
    pub fn set_register(&mut self, register: Register, value: u8) {
        self.registers.set_reg(register, value)
    }

    #[inline]
    pub fn get_register(&self, register: Register) -> u8 {
        self.registers.get_reg(register)
    }

    pub fn get_sptr(&self) -> u16 {
        self.registers.sp
    }

    /// Pushes a value onto the stack
    pub fn push(&mut self, memory: &mut Memory<M>, value: u8) {
        memory.write_mem_u8(self.get_sptr(), value).unwrap();
        self.registers.sp -= 1;
    }

    /// Pops a value from the stack
    pub fn pop(&mut self, memory: &mut Memory<M>) -> u8 {
        let value = memory.read_mem_u8(self.get_sptr()).unwrap();
        self.registers.sp += 1;
        value
    }

    /// Consumes a single byte from memory and returns it
    ///
    /// # Arguments
    ///
    /// * `memory` - The memory to consume from
    pub fn consume(&mut self, memory: &Memory<M>) -> u8 {
        let value = memory.data[self.registers.pc as usize];
        self.registers.pc += 1;
        value
    }

    /// Consumes a single u16 from memory and returns it
    ///
    /// * `memory` - The memory to consume from
    pub fn consume_u16(&mut self, memory: &Memory<M>) -> u16 {
        let value = u16::from_be_bytes([
            memory.data[self.registers.pc as usize],
            memory.data[self.registers.pc as usize + 1],
        ]);
        self.registers.pc += 2;
        value
    }

    #[inline]
    fn consume_imm8(&mut self, memory: &Memory<M>) -> (Register, u8, u8) {
        let dest_register = Register::from(self.consume(memory));
        let dest_value = self.registers.get_reg(dest_register);
        let value = self.consume(memory);
        (dest_register, dest_value, value)
    }

    #[inline]
    fn consume_imm16(&mut self, memory: &Memory<M>) -> (Register, u16, u16) {
        let dest_register = Register::from(self.consume(memory));
        let dest_value = self.registers.get_reg_pair(dest_register);
        let value = self.consume_u16(memory);
        (dest_register, dest_value, value)
    }

    #[inline]
    fn consume_reg_dir(&mut self, memory: &Memory<M>) -> (Register, u8, u8) {
        // convert these into Register
        let (dest_register, src_register) = unpack_registers(self.consume(memory));
        let dest_value = self.registers.get_reg(dest_register.into());
        let src_value = self.registers.get_reg(src_register.into());
        (dest_register.into(), dest_value, src_value)
    }

    #[inline]
    fn consume_reg(&mut self, memory: &Memory<M>) -> (Register, u8, u8) {
        let (dest_register, src_register) = unpack_registers(self.consume(memory));
        let dest_value = self.registers.get_reg(dest_register.into());
        let value_addr = self.registers.get_reg_pair(src_register.into());
        let value = memory.read_mem_u8(value_addr).unwrap();
        (dest_register.into(), dest_value, value)
    }

    #[inline]
    fn consume_reg_ind_x(&mut self, memory: &Memory<M>) -> (Register, u8, u8) {
        let (dest_register, src_register) = unpack_registers(self.consume(memory));
        let offset = self.consume(memory);
        let dest_value = self.registers.get_reg(dest_register.into());
        let value_addr = self.registers.get_reg_pair(src_register.into()).wrapping_add(offset as u16);
        let value = memory.read_mem_u8(value_addr).unwrap();
        (dest_register.into(), dest_value, value)
    }

    #[inline]
    fn consume_dir(&mut self, memory: &Memory<M>) -> (Register, u8, u8) {
        let dest_register = Register::from(self.consume(memory));
        let dest_value = self.registers.get_reg(dest_register);
        let addr = self.consume_u16(memory);
        let value = memory.read_mem_u8(addr).unwrap();
        (dest_register, dest_value, value)
    }

    /// Executes a single instruction
    ///
    /// # Arguments
    ///
    /// * `memory` - The memory to execute the instruction on
    pub fn execute(&mut self, memory: &mut Memory<M>) {
        let opcode = Opcode::from(self.consume(memory));
        match opcode {
            Opcode::Nop => {}
            _ => unimplemented!(),
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use crate::opcode::*;
//     use super::*;
// }
