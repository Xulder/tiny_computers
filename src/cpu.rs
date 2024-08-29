
use core::panic;

use crate::memory::Memory;
use crate::registers::{Register, RegisterFile};
use crate::opcode::{decode_opcode, unpack_registers, AddressingMode, Opcode};

/// The CPU of the Tiny Computer
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
            }
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
        self.registers.set(register, value)
    }

    #[inline]
    pub fn get_register(&self, register: Register) -> u8 {
        self.registers.get(register)
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
    fn consume_immediate(&mut self, memory: &Memory<M>) -> (Register, u8, u8) {
        let dest_register = Register::from(self.consume(memory));
        let dest_value = self.registers.get(dest_register);
        let value = self.consume(memory);
        (dest_register, dest_value, value)
    }

    #[inline]
    fn consume_reg_dir(&mut self, memory: &Memory<M>) -> (Register, u8, u8) {
        let (dest_register, src_register) = unpack_registers(self.consume(memory));
        let dest_value = self.registers.get(dest_register);
        let src_value = self.registers.get(src_register);
        (dest_register, dest_value, src_value)
    }

    #[inline]
    fn consume_reg_ind(&mut self, memory: &Memory<M>) -> (Register, u8, u8) {
        let (dest_register, src_register) = unpack_registers(self.consume(memory));
        let dest_value = self.registers.get(dest_register);
        let high = self.registers.get(src_register);
        let low = self.registers.get(Register::from((src_register as u8) + 1));
        let value_addr = u16::from_be_bytes([high, low]);
        let value = memory.read_mem_u8(value_addr).unwrap();
        (dest_register, dest_value, value)
    }

    #[inline]
    fn consume_reg_ind_x(&mut self, memory: &Memory<M>) -> (Register, u8, u8) {
        let (dest_register, src_register) = unpack_registers(self.consume(memory));
        let offset = self.consume(memory);
        let dest_value = self.registers.get(dest_register);
        let high = self.registers.get(src_register);
        let low = self.registers.get(Register::from((src_register as u8) + 1));
        let value_addr = u16::from_be_bytes([high, low]);
        let addr = value_addr.wrapping_add(offset as u16);
        let value = memory.read_mem_u8(addr).unwrap();
        (dest_register, dest_value, value)
    }

    #[inline]
    fn consume_dir(&mut self, memory: &Memory<M>) -> (Register, u8, u8) {
        let dest_register = Register::from(self.consume(memory));
        let dest_value = self.registers.get(dest_register);
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
        let (opcode, mode)= decode_opcode(self.consume(memory));

        match opcode {
            Opcode::Nop => {},
            Opcode::Add => {
                match mode {
                    AddressingMode::Immediate => {
                        let (dest_register, dest_value, value) = self.consume_immediate(memory);
                        self.registers.set(dest_register, dest_value.wrapping_add(value));
                    },
                    AddressingMode::Register => {
                        let (dest_register, dest_value, src_value) = self.consume_reg_dir(memory);
                        self.registers.set(dest_register, dest_value.wrapping_add(src_value));
                    },
                    AddressingMode::RegisterIndirect => {
                        let (dest_register, dest_value, value) = self.consume_reg_ind(memory);
                        self.registers.set(dest_register, dest_value.wrapping_add(value));
                    },
                    AddressingMode::RegisterIndirectIndexed => {
                        let (dest_register, dest_value, value) = self.consume_reg_ind_x(memory);
                        self.registers.set(dest_register, dest_value.wrapping_add(value));
                    },
                    AddressingMode::Direct => {
                        let (dest_register, dest_value, value) = self.consume_dir(memory);
                        self.registers.set(dest_register, dest_value.wrapping_add(value));
                    },
                    AddressingMode::Relative => panic!("Add does not support relative addressing."),
                }
            },
            Opcode::Sub => {
                match mode {
                    AddressingMode::Immediate => {
                        let (dest_register, dest_value, value) = self.consume_immediate(memory);
                        self.registers.set(dest_register, dest_value.wrapping_sub(value));
                    },
                    AddressingMode::Register => {
                        let (dest_register, dest_value, src_value) = self.consume_reg_dir(memory);
                        self.registers.set(dest_register, dest_value.wrapping_sub(src_value));
                    },
                    AddressingMode::RegisterIndirect => {
                        let (dest_register, dest_value, value) = self.consume_reg_ind(memory);
                        self.registers.set(dest_register, dest_value.wrapping_sub(value));
                    },
                    AddressingMode::RegisterIndirectIndexed => {
                        let (dest_register, dest_value, value) = self.consume_reg_ind_x(memory);
                        self.registers.set(dest_register, dest_value.wrapping_sub(value));
                    },
                    AddressingMode::Direct => {
                        let (dest_register, dest_value, value) = self.consume_dir(memory);
                        self.registers.set(dest_register, dest_value.wrapping_sub(value));
                    },
                    AddressingMode::Relative => panic!("Sub does not support relative addressing."),
                }
            },

            Opcode::Mul => {
                match mode {
                    AddressingMode::Immediate => {
                        let (dest_register, dest_value, value) = self.consume_immediate(memory);
                        self.registers.set(dest_register, dest_value.wrapping_mul(value));
                    },
                    AddressingMode::Register => {
                        let (dest_register, dest_value, src_value) = self.consume_reg_dir(memory);
                        self.registers.set(dest_register, dest_value.wrapping_mul(src_value));
                    },
                    AddressingMode::RegisterIndirect => {
                        let (dest_register, dest_value, value) = self.consume_reg_ind(memory);
                        self.registers.set(dest_register, dest_value.wrapping_mul(value));
                    },
                    AddressingMode::RegisterIndirectIndexed => {
                        let (dest_register, dest_value, value) = self.consume_reg_ind_x(memory);
                        self.registers.set(dest_register, dest_value.wrapping_mul(value));
                    },
                    AddressingMode::Direct => {
                        let (dest_register, dest_value, value) = self.consume_dir(memory);
                        self.registers.set(dest_register, dest_value.wrapping_mul(value));
                    },
                    AddressingMode::Relative => {},
                }
            },

            Opcode::Div => {
                match mode {
                    AddressingMode::Immediate => {
                        let (dest_register, dest_value, value) = self.consume_immediate(memory);
                        self.registers.set(dest_register, dest_value.wrapping_div(value));
                    },
                    AddressingMode::Register => {
                        let (dest_register, dest_value, src_value) = self.consume_reg_dir(memory);
                        self.registers.set(dest_register, dest_value.wrapping_div(src_value));
                    },
                    AddressingMode::RegisterIndirect => {
                        let (dest_register, dest_value, value) = self.consume_reg_ind(memory);
                        self.registers.set(dest_register, dest_value.wrapping_div(value));
                    },
                    AddressingMode::RegisterIndirectIndexed => {
                        let (dest_register, dest_value, value) = self.consume_reg_ind_x(memory);
                        self.registers.set(dest_register, dest_value.wrapping_div(value));
                    },
                    AddressingMode::Direct => {
                        let (dest_register, dest_value, value) = self.consume_dir(memory);
                        self.registers.set(dest_register, dest_value.wrapping_div(value));
                    },
                    AddressingMode::Relative => {},
                }
            },

            Opcode::Mov => {
                match mode {
                    AddressingMode::Immediate => {
                        let (dest_register, _, value) = self.consume_immediate(memory);
                        self.registers.set(dest_register, value);
                    },
                    AddressingMode::Register => {
                        let (dest_register, _, src_value) = self.consume_reg_dir(memory);
                        self.registers.set(dest_register, src_value);
                    },
                    AddressingMode::RegisterIndirect => {
                        let (dest_register, _, value) = self.consume_reg_ind(memory);
                        self.registers.set(dest_register, value);
                    },
                    AddressingMode::RegisterIndirectIndexed => {
                        let (dest_register, _, value) = self.consume_reg_ind_x(memory);
                        self.registers.set(dest_register, value);
                    },
                    AddressingMode::Direct => {
                        let (dest_register, _, value) = self.consume_dir(memory);
                        self.registers.set(dest_register, value);
                    },
                    AddressingMode::Relative => {},
                }
            },

            Opcode::Pop => {
                let dest_register = Register::from(self.consume(memory));
                let value = self.pop(memory);
                self.registers.set(dest_register, value);
            },

            Opcode::Psh => {
                let value = self.consume(memory);
                self.push(memory, value);
            },
            Opcode::Cmp => {
                match mode {
                    AddressingMode::Immediate => {},
                    AddressingMode::Register => {},
                    AddressingMode::RegisterIndirect => {},
                    AddressingMode::RegisterIndirectIndexed => {},
                    AddressingMode::Direct => {},
                    AddressingMode::Relative => {},
                }
            },

            Opcode::Jmp => {
                match mode {
                    AddressingMode::Immediate => {},
                    AddressingMode::Register => {},
                    AddressingMode::RegisterIndirect => {},
                    AddressingMode::RegisterIndirectIndexed => {},
                    AddressingMode::Direct => {},
                    AddressingMode::Relative => {},
                }
            },

            Opcode::Jz => {
                match mode {
                    AddressingMode::Immediate => {},
                    AddressingMode::Register => {},
                    AddressingMode::RegisterIndirect => {},
                    AddressingMode::RegisterIndirectIndexed => {},
                    AddressingMode::Direct => {},
                    AddressingMode::Relative => {},
                }
            },

            Opcode::Jnz => {
                match mode {
                    AddressingMode::Immediate => {},
                    AddressingMode::Register => {},
                    AddressingMode::RegisterIndirect => {},
                    AddressingMode::RegisterIndirectIndexed => {},
                    AddressingMode::Direct => {},
                    AddressingMode::Relative => {},
                }
            },

            Opcode::Hlt => {},
        }
    }

}

#[cfg(test)]
mod tests {
    use crate::opcode::*;

    use super::*;

    #[test]
    fn test_add_imm() {
        let mut cpu = Cpu::<256>::new();
        let mut mem = Memory::<256>::new();
        cpu.registers.set(Register::A, 2);
        let add_imm = encode_opcode(Opcode::Add, AddressingMode::Immediate);
        // ADD A, 2
        mem.write_mem_u8(0, add_imm).unwrap();
        mem.write_mem_u8(1, Register::A as u8).unwrap();
        mem.write_mem_u8(2, 2).unwrap();

        cpu.execute(&mut mem);

        assert_eq!(cpu.registers.get(Register::A), 4);
    }

    #[test]
    fn test_add_reg() {
        let mut cpu = Cpu::<256>::new();
        let mut mem = Memory::<256>::new();
        cpu.registers.set(Register::A, 2);
        cpu.registers.set(Register::B, 3);

        let add_reg = encode_opcode(Opcode::Add, AddressingMode::Register);
        // ADD A, B
        mem.write_mem_u8(0, add_reg).unwrap();
        mem.write_mem_u8(1, pack_registers(Register::A as u8, Register::B as u8)).unwrap();

        cpu.execute(&mut mem);

        assert_eq!(cpu.registers.get(Register::A), 5);
    }

    #[test]
    fn test_add_reg_ind() {
        let mut cpu = Cpu::<256>::new();
        let mut mem = Memory::<256>::new();
        mem.write_mem_u8(2, 2).unwrap();
        cpu.registers.set(Register::A, 2);
        cpu.registers.set(Register::B, 0);
        cpu.registers.set(Register::C, 2);

        let add_reg_ind = encode_opcode(Opcode::Add, AddressingMode::RegisterIndirect);

        // ADD A, [B]
        mem.write_mem_u8(0, add_reg_ind).unwrap();
        mem.write_mem_u8(1, pack_registers(Register::A as u8, Register::B as u8)).unwrap();

        cpu.execute(&mut mem);

        assert_eq!(cpu.registers.get(Register::A), 4);
    }

    #[test]
    fn test_add_reg_ind_x() {
        let mut cpu = Cpu::<256>::new();
        let mut mem = Memory::<256>::new();
        mem.write_mem_u8(6, 3).unwrap();
        cpu.set_register(Register::A, 2);
        cpu.set_register(Register::B, 0);
        cpu.set_register(Register::C, 5);

        let add_reg_ind_x = encode_opcode(Opcode::Add, AddressingMode::RegisterIndirectIndexed);

        // ADD A, [B+1]
        mem.write_mem_u8(0, add_reg_ind_x).unwrap();
        mem.write_mem_u8(1, pack_registers(Register::A as u8, Register::B as u8)).unwrap();
        mem.write_mem_u8(2, 1).unwrap();

        cpu.execute(&mut mem);

        assert_eq!(cpu.get_register(Register::A), 5);
    }

    #[test]
    fn test_add_dir() {
        let mut cpu = Cpu::<256>::new();
        let mut mem = Memory::<256>::new();
        mem.write_mem_u8(5, 3).unwrap();
        cpu.set_register(Register::A, 2);

        let add_reg_dir = encode_opcode(Opcode::Add, AddressingMode::Direct);

        // ADD A, [x05]
        mem.write_mem_u8(0, add_reg_dir).unwrap();
        mem.write_mem_u8(1, Register::A as u8).unwrap();
        mem.write_mem_u16(2, 0x05).unwrap();

        cpu.execute(&mut mem);

        assert_eq!(cpu.get_register(Register::A), 5);
    }

    #[test]
    fn test_sub_imm() {
        let mut cpu = Cpu::<256>::new();
        let mut mem = Memory::<256>::new();
        cpu.set_register(Register::A, 5);

        let sub_imm = encode_opcode(Opcode::Sub, AddressingMode::Immediate);

        // SUB A, 2
        mem.write_mem_u8(0, sub_imm).unwrap();
        mem.write_mem_u8(1, Register::A as u8).unwrap();
        mem.write_mem_u8(2, 2).unwrap();

        cpu.execute(&mut mem);

        assert_eq!(cpu.get_register(Register::A), 3);
    }

    #[test]
    fn test_sub_reg() {
        let mut cpu = Cpu::<256>::new();
        let mut mem = Memory::<256>::new();
        cpu.set_register(Register::A, 5);
        cpu.set_register(Register::B, 2);

        let sub_reg = encode_opcode(Opcode::Sub, AddressingMode::Register);

        // SUB A, B
        mem.write_mem_u8(0, sub_reg).unwrap();
        mem.write_mem_u8(1, pack_registers(Register::A as u8, Register::B as u8)).unwrap();

        cpu.execute(&mut mem);

        assert_eq!(cpu.get_register(Register::A), 3);
    }

    #[test]
    fn test_sub_reg_ind() {
        let mut cpu = Cpu::<256>::new();
        let mut mem = Memory::<256>::new();
        mem.write_mem_u8(5, 2).unwrap();
        cpu.set_register(Register::A, 5);
        cpu.set_register(Register::B, 0);
        cpu.set_register(Register::C, 5);

        let sub_reg_ind = encode_opcode(Opcode::Sub, AddressingMode::RegisterIndirect);

        // SUB A, [B]
        mem.write_mem_u8(0, sub_reg_ind).unwrap();
        mem.write_mem_u8(1, pack_registers(Register::A as u8, Register::B as u8)).unwrap();

        cpu.execute(&mut mem);

        assert_eq!(cpu.get_register(Register::A), 3);
    }

    #[test]
    fn test_sub_reg_ind_x() {
        let mut cpu = Cpu::<256>::new();
        let mut mem = Memory::<256>::new();
        mem.write_mem_u8(6, 2).unwrap();
        cpu.set_register(Register::A, 5);
        cpu.set_register(Register::B, 0);
        cpu.set_register(Register::C, 5);

        let sub_reg_ind_x = encode_opcode(Opcode::Sub, AddressingMode::RegisterIndirectIndexed);

        // SUB A, [B+1]
        mem.write_mem_u8(0, sub_reg_ind_x).unwrap();
        mem.write_mem_u8(1, pack_registers(Register::A as u8, Register::B as u8)).unwrap();
        mem.write_mem_u8(2, 1).unwrap();

        cpu.execute(&mut mem);

        assert_eq!(cpu.get_register(Register::A), 3);
    }

    #[test]
    fn test_sub_dir() {
        let mut cpu = Cpu::<256>::new();
        let mut mem = Memory::<256>::new();
        mem.write_mem_u8(5, 2).unwrap();
        cpu.set_register(Register::A, 5);

        let sub_reg_dir = encode_opcode(Opcode::Sub, AddressingMode::Direct);

        // SUB A, [x05]
        mem.write_mem_u8(0, sub_reg_dir).unwrap();
        mem.write_mem_u8(1, Register::A as u8).unwrap();
        mem.write_mem_u16(2, 0x05).unwrap();

        cpu.execute(&mut mem);

        assert_eq!(cpu.get_register(Register::A), 3);
    }

}