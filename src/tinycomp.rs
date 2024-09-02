// use crate::ports::Ports;

use crate::memory::Memory;
use crate::cpu::Cpu;


#[derive(Debug)]
pub struct TinyComputer<const M: usize, const R: usize, const S: usize> {
    mem: Memory<M>,
    cpu: Cpu<R>,
    // ports: Ports,
}

impl<const M: usize, const R: usize, const S: usize> TinyComputer<M, R, S> {
    pub fn new() -> TinyComputer<M, R, S> {
        if M == 0 || R <= 1 || S == 0 {
            panic!("Tiny Computer: Memory, Registers, and Stack sizes must all be greater than 0.")
        }
        if M % 2 != 0 {
            panic!("Tiny Computer: Memory size must be a multiple of 2.")
        }
        if S > 255 || R > 255 {
            panic!("Tiny Computer: Stack and Register size must be less than 255.")
        }
        TinyComputer {
            mem: Memory::new(),
            cpu: Cpu::new(),
            // ports: Ports::new(),
        }
    }

    pub fn run(&mut self) {
        // self.cpu.run(&mut self.mem, &mut self.ports);
    }

    pub fn step(&mut self) {
        // self.cpu.step(&mut self.mem, &mut self.ports);
    }
}
