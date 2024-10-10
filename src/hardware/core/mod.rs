pub mod registers;
pub mod isa;
pub mod memory;

use crate::hardware::core::memory::Memory;
use self::isa::{Mode, Instruction, Opcode};

use self::registers::{unpack_registers, Register, RegisterFile};

/// The CPU of the Tiny Computer
#[derive(Debug)]
pub struct Cpu<const M: usize> {
    /// The registers of the CPU
    registers: RegisterFile,
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
    pub fn set_register_pair(&mut self, register: Register, value: u16) {
        self.registers.set_reg_pair(register, value)
    }

    #[inline]
    pub fn get_register(&self, register: Register) -> u8 {
        self.registers.get_reg(register)
    }

    #[inline]
    pub fn get_register_pair(&self, register: Register) -> u16 {
        self.registers.get_reg_pair(register)
    }

    pub fn get_sptr(&self) -> u16 {
        self.registers.sp
    }

    /// Pushes a value onto the stack
    pub fn push(&mut self, memory: &mut Memory<M>, value: u8) {
        memory.write_u8(self.get_sptr(), value).unwrap();
        self.registers.sp -= 1;
    }

    /// Pops a value from the stack
    pub fn pop(&mut self, memory: &mut Memory<M>) -> u8 {
        let value = memory.read_u8(self.get_sptr()).unwrap();
        self.registers.sp += 1;
        value
    }

    /// Consumes a single byte from memory and returns it
    ///
    /// # Arguments
    ///
    /// * `memory` - The memory to consume from
    pub fn consume_u8(&mut self, memory: &Memory<M>) -> u8 {
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

    fn read_memory_u16(&self, memory: &Memory<M>, addr: u16) -> u16 {
        memory.read_u16(addr).unwrap()
    }

    fn write_memory_u16(&mut self, memory: &mut Memory<M>, addr: u16, value: u16) {
        memory.write_u16(addr, value).unwrap();
    }

    /// Executes a single instruction
    ///
    /// # Arguments
    ///
    /// * `memory` - The memory to execute the instruction on
    pub fn execute(&mut self, memory: &mut Memory<M>) {
        let opcode = Opcode::from(self.consume_u8(memory));
        let instruction = opcode.instruction();
        let mode = opcode.mode();
        match instruction {
            Instruction::Load => match mode {
                Mode::Immediate => {
                    let dest_reg = Register::from(self.consume_u8(memory));
                    if dest_reg.is_pair() {
                        let value = self.consume_u16(memory);
                        self.set_register_pair(dest_reg, value);
                    } else {
                        let value = self.consume_u8(memory);
                        self.set_register(dest_reg, value);
                    }
                }
                Mode::Register => {
                    let (dest_reg, src_reg) = unpack_registers(self.consume_u8(memory));
                    if dest_reg.is_pair() {
                        let value = self.get_register_pair(src_reg);
                        self.set_register_pair(dest_reg, value);
                    } else {
                        let value = self.get_register(src_reg);
                        self.set_register(dest_reg, value);
                    }
                }
                Mode::RegisterIndirect => {
                    let (dest_reg, src_reg) = unpack_registers(self.consume_u8(memory));
                    if dest_reg.is_pair() {
                        let value_addr = self.get_register_pair(src_reg);
                        let value = self.read_memory_u16(memory, value_addr);
                        self.set_register_pair(dest_reg, value);
                    } else {
                        let value_addr = self.get_register_pair(src_reg);
                        let value = memory.read_u8(value_addr).unwrap();
                        self.set_register(dest_reg, value);
                    }
                }
                Mode::RegisterIndexed => {
                    let (dest_reg, src_reg) = unpack_registers(self.consume_u8(memory));
                    let offset = self.consume_u16(memory);
                    if dest_reg.is_pair() {
                        let value_addr = self.get_register_pair(src_reg) + offset;
                        let value = memory.read_u16(value_addr).unwrap();
                        self.set_register_pair(dest_reg, value);
                    } else {
                        let value_addr = self.get_register_pair(src_reg) + offset;
                        let value = memory.read_u8(value_addr).unwrap();
                        self.set_register(dest_reg, value);
                    }
                }
                Mode::Direct => {
                    let dest_reg = Register::from(self.consume_u8(memory));
                    let addr = self.consume_u16(memory);
                    if dest_reg.is_pair() {
                        let value = memory.read_u16(addr).unwrap();
                        self.set_register_pair(dest_reg, value);
                    } else {
                        let value = memory.read_u8(addr).unwrap();
                        self.set_register(dest_reg, value);
                    }
                }
                _ => unimplemented!(),
            },
            Instruction::Write => match mode {
                Mode::Direct => {
                    let src_reg = Register::from(self.consume_u8(memory));
                    let addr = self.consume_u16(memory);
                    if src_reg.is_pair() {
                        let value = self.get_register_pair(src_reg);
                        memory.write_u16(addr, value).unwrap();
                    } else {
                        let value = self.get_register(src_reg);
                        memory.write_u8(addr, value).unwrap();
                    }
                }
                _ => unimplemented!(),
            },
            _ => unimplemented!(),
        }
    }
}
