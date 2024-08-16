
use crate::cpu::Cpu;

pub struct TinyComputer<
    const ONCHIP_MEM: usize,
    const ONBOARD_MEM: usize,
    const NUM_GEN_REGS: usize,
> {
    cpu: Cpu<ONCHIP_MEM, NUM_GEN_REGS>,
    onboard_mem: [u8; ONBOARD_MEM],
}

impl<const ONCHIP_MEM: usize, const ONBOARD_MEM: usize, const NUM_GEN_REGS: usize>
    TinyComputer<ONCHIP_MEM, ONBOARD_MEM, NUM_GEN_REGS>
{
    pub fn new() -> Self {
        Self {
            cpu: Cpu::new("TinyCore16KB ".to_string(), 1000).unwrap(),
            onboard_mem: [0; ONBOARD_MEM],
        }
    }

    fn map_addr(&self, addr: u16) -> u16 {
        if addr < ONCHIP_MEM as u16 {
            addr
        } else if addr > (ONCHIP_MEM + ONBOARD_MEM) as u16 {
            panic!("Address out of bounds");
        } else {
            addr - ONCHIP_MEM as u16
        }
    }

    pub fn read_mem(&self, addr: u16) -> u8 {
        let mapped_addr = self.map_addr(addr);
        if mapped_addr < ONCHIP_MEM as u16 {
            self.cpu.read_mem(mapped_addr)
        } else {
            self.onboard_mem[mapped_addr as usize - ONCHIP_MEM]
        }
    }

    pub fn write_mem(&mut self, addr: u16, value: u8) {
        let mapped_addr = self.map_addr(addr);
        if mapped_addr < ONCHIP_MEM as u16 {
            self.cpu.write_mem(mapped_addr, value);
        } else {
            self.onboard_mem[mapped_addr as usize - ONCHIP_MEM] = value;
        }
    }

    pub fn run(&mut self) {
        loop {
            let opcode = self.read_mem(self.cpu.registers.pc);
            let result = self.cpu.execute(opcode);
            if !result {
                break;
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_tiny_computer() {
        let mut tiny= TinyComputer::<1024, 4096, 2>::new();
        // Equivalent pseudocode:
        // ```
        // NOP
        // ADD
        // ADD -- These do nothing, there's no values in the registers /// 
        // ADD R1 0x0100 -- literal 2 is stored there
        // ADD R2 0x0100
        // ADD -- R1+R2 -> R1  2+2 with 4 stored to R1
        // HALT
        // ```
        tiny.write_mem(0x0000, 0x00);
        tiny.write_mem(0x0001, 0x01);
        tiny.write_mem(0x0002, 0x01);
        tiny.write_mem(0x0003, 0x11);
        tiny.write_mem(0x0004, 0b0000_0001);
        tiny.write_mem(0x0005, 0b0000_0000);
        tiny.write_mem(0x0006, 0x21);
        tiny.write_mem(0x0007, 0b0000_0001);
        tiny.write_mem(0x0008, 0b0000_0000);
        tiny.write_mem(0x0009, 0x01);
        tiny.write_mem(0x000A, 0xFF);
        tiny.write_mem(0x0100, 0x02);
        tiny.run();
    }
}