use crate::memory::Memory;
use crate::opcode::{Opcode, Instruction, Mode};
use crate::registers::{unpack_registers, Register, RegisterFile};

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
            Instruction::Load => {
                match mode {
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
                            let value = memory.read_mem_u16(value_addr).unwrap();
                            self.set_register_pair(dest_reg, value);
                        } else {
                            let value_addr = self.get_register_pair(src_reg);
                            let value = memory.read_mem_u8(value_addr).unwrap();
                            self.set_register(dest_reg, value);
                        }
                    }
                    Mode::RegisterIndexed => {
                        let (dest_reg, src_reg) = unpack_registers(self.consume_u8(memory));
                        let offset = self.consume_u16(memory);
                        if dest_reg.is_pair() {
                            let value_addr = self.get_register_pair(src_reg) + offset;
                            let value = memory.read_mem_u16(value_addr).unwrap();
                            self.set_register_pair(dest_reg, value);
                        } else {
                            let value_addr = self.get_register_pair(src_reg) + offset;
                            let value = memory.read_mem_u8(value_addr).unwrap();
                            self.set_register(dest_reg, value);
                        }
                    }
                    Mode::Direct => {
                        let dest_reg = Register::from(self.consume_u8(memory));
                        let addr = self.consume_u16(memory);
                        if dest_reg.is_pair() {
                            let value = memory.read_mem_u16(addr).unwrap();
                            self.set_register_pair(dest_reg, value);
                        } else {
                            let value = memory.read_mem_u8(addr).unwrap();
                            self.set_register(dest_reg, value);
                        }
                    }
                    _ => unimplemented!(),
                }
            },
            Instruction::Write => {
                match mode {
                    Mode::Direct => {
                        let dest_reg = Register::from(self.consume_u8(memory));
                        let addr = self.consume_u16(memory);
                        if dest_reg.is_pair() {
                            let value = self.get_register_pair(dest_reg);
                            memory.write_mem_u16(addr, value).unwrap();
                        } else {
                            let value = self.get_register(dest_reg);
                            memory.write_mem_u8(addr, value).unwrap();
                        }
                    }
                    _ => unimplemented!(),
                }
            }
            _ => unimplemented!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::registers::pack_registers;

    use super::*;

    const M: usize = 512;

    #[test]
    pub fn test_load_imm() {
        let mut cpu = Cpu::<M>::new();
        let mut memory = Memory::<M>::new();

        let load_imm = Opcode::new(Instruction::Load, Mode::Immediate);

        // writing test program to memory
        // load a 1
        memory.write_mem_u8(0, load_imm.as_u8()).unwrap();
        memory.write_mem_u8(1, Register::A.as_u8()).unwrap();
        memory.write_mem_u8(2, 1).unwrap();

        cpu.execute(&mut memory);

        assert_eq!(cpu.get_register(Register::A), 1);
    }

    #[test]
    fn test_load_reg() {
        let mut cpu = Cpu::<M>::new();
        let mut memory = Memory::<M>::new();

        cpu.set_register(Register::A, 0);
        cpu.set_register(Register::B, 1);
    
        let load_reg = Opcode::new(Instruction::Load, Mode::Register);
        let a_and_b = pack_registers(Register::A, Register::B);
    
        // load a b
        memory.write_mem_u8(0, load_reg.as_u8()).unwrap();
        memory.write_mem_u8(1, a_and_b).unwrap();
    
        cpu.execute(&mut memory);
    
        assert_eq!(cpu.get_register(Register::A), 1);
    }
    
    #[test]
    fn test_load_reg_indirect() {
        let mut cpu = Cpu::<M>::new();
        let mut memory = Memory::<M>::new();
    
        cpu.set_register(Register::A, 0);
        cpu.set_register_pair(Register::CD, 5);
    
        let load_reg_indirect = Opcode::new(Instruction::Load, Mode::RegisterIndirect);
        let a_and_cd = pack_registers(Register::A, Register::CD);

        // putting value 1 in memory at address 5
        memory.write_mem_u8(5, 1).unwrap();

        // load a [cd]
        memory.write_mem_u8(0, load_reg_indirect.as_u8()).unwrap();
        memory.write_mem_u8(1, a_and_cd).unwrap();
    
        cpu.execute(&mut memory);
    
        assert_eq!(cpu.get_register(Register::A), 1);
    }

    #[test]
    fn test_load_reg_indexed() {
        let mut cpu = Cpu::<M>::new();
        let mut memory = Memory::<M>::new();
    
        cpu.set_register(Register::A, 0);
        cpu.set_register_pair(Register::CD, 5);
    
        let load_reg_indirect = Opcode::new(Instruction::Load, Mode::RegisterIndexed);
        let a_and_cd = pack_registers(Register::A, Register::CD);

        // putting value 1 in memory at address 6
        memory.write_mem_u8(6, 1).unwrap();

        // load a [cd + 1]
        memory.write_mem_u8(0, load_reg_indirect.as_u8()).unwrap();
        memory.write_mem_u8(1, a_and_cd).unwrap();
        memory.write_mem_u16(2, 1).unwrap();
    
        cpu.execute(&mut memory);
    
        assert_eq!(cpu.get_register(Register::A), 1);
    }

    #[test]
    fn test_load_direct() {
        let mut cpu = Cpu::<M>::new();
        let mut memory = Memory::<M>::new();

        cpu.set_register(Register::A, 0);

        let load_direct = Opcode::new(Instruction::Load, Mode::Direct);

        // putting value 1 in memory at address 5
        memory.write_mem_u8(5, 1).unwrap();

        // load a [5]
        memory.write_mem_u8(0, load_direct.as_u8()).unwrap();
        memory.write_mem_u8(1, Register::A.as_u8()).unwrap();
        memory.write_mem_u16(2, 5).unwrap();
    
        cpu.execute(&mut memory);
    
        assert_eq!(cpu.get_register(Register::A), 1);
    }

    #[test]
    fn test_write() {
        let mut cpu = Cpu::<M>::new();
        let mut memory = Memory::<M>::new();

        cpu.set_register(Register::A, 2);

        let write = Opcode::new(Instruction::Write, Mode::Direct);

        // write a [5]
        memory.write_mem_u8(0, write.as_u8()).unwrap();
        memory.write_mem_u8(1, Register::A.as_u8()).unwrap();
        memory.write_mem_u16(2, 5).unwrap();
    
        cpu.execute(&mut memory);
    
        assert_eq!(memory.read_mem_u8(5).unwrap(), 2);
    }
}