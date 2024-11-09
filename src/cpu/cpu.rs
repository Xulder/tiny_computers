use crate::{
    cpu::{
        error::{CPUError, CPUResult},
        register_files::TestRegisterFile,
        traits::{RegisterFile, CPU},
    },
    isa::{instruction::Instruction, traits::ISA},
};

pub struct TestCPU {
    pub isa: TestISA,
    pub register_file: TestRegisterFile,
    pub sp: u16,
    pub pc: u16,
}

impl CPU<TestRegisterFile, TestISA> for TestCPU {
    fn new(isa: TestISA) -> TestCPU {
        TestCPU {
            isa,
            register_file: TestRegisterFile::new(),
            sp: 0,
            pc: 0,
        }
    }

    fn get_register_file(&self) -> &TestRegisterFile {
        &self.register_file
    }

    fn get_register_file_mut(&mut self) -> &mut TestRegisterFile {
        &mut self.register_file
    }

    fn get_pc(&self) -> u16 {
        self.pc
    }

    fn set_pc(&mut self, pc: u16) {
        self.pc = pc
    }

    fn get_sp(&self) -> u16 {
        self.sp
    }

    fn set_sp(&mut self, sp: u16) {
        self.sp = sp
    }

    fn reset(&mut self) {
        self.register_file.reset();
        self.sp = 0;
        self.pc = 0;
    }

    fn step(&mut self) {
        self.pc += 1;
        // TODO: Implement opcode execution
    }
}

pub struct TestISA;

impl ISA for TestISA {
    fn decode_instruction(&self, instruction: u8) -> Instruction {
        Instruction(instruction)
    }

    fn encode_instruction(&self, instruction: Instruction) -> u8 {
        instruction.0
    }
}
