use crate::{
    cpu::Cpu,
    memory::Memory,
    opcode::{Instruction, Mode, Opcode},
    registers::{pack_registers, Register},
};

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