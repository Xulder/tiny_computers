#[derive(Clone, Copy)]
enum IoPort {
    Serial,
    Parallel,
    Empty,
}

/// Represents a CPU's registers.
/// All CPU's in tiny computers have a 16-bit addressing space
#[derive(Debug)]
pub struct Registers<const NUM_GEN_REGS: usize> {
    /// Program counter.
    pub pc: u16,
    /// Stack pointer.
    pub sp: u16,
    /// Flags register.
    pub flags: u8,
    /// General-purpose registers.
    pub general: [u8; NUM_GEN_REGS],
}

impl<const NUM_GEN_REGS: usize> Registers<NUM_GEN_REGS> {
    /// Creates a new `Registers` instance.
    pub fn new() -> Self {
        Self {
            pc: 0,
            sp: 0,
            flags: 0,
            general: [0; NUM_GEN_REGS],
        }
    }

    pub fn get(&self, gen_reg_idx: usize) -> u8 {
        assert!(gen_reg_idx < NUM_GEN_REGS);
        self.general[gen_reg_idx]
    }

    pub fn set(&mut self, gen_reg_idx: usize, value: u8) {
        assert!(gen_reg_idx < NUM_GEN_REGS);
        self.general[gen_reg_idx] = value;
    }
}

impl<const NUM_GEN_REGS: usize> Default for Registers<NUM_GEN_REGS> {
    fn default() -> Self {
        Self {
            pc: 0,
            sp: 0,
            flags: 0,
            general: [0; NUM_GEN_REGS],
        }
    }
}

/// Represents a CPU.
#[derive(Debug)]
pub struct Cpu<const ONCHIP_MEM: usize, const NUM_GEN_REGS: usize> {
    /// The CPU's architecture.
    pub arch: String,
    /// Clock speed in cycles per second.
    pub clock: u32,
    /// Onboard memory.
    pub onchip_mem: [u8; ONCHIP_MEM],
    /// CPU registers.
    pub registers: Registers<NUM_GEN_REGS>,
}

impl<const ONCHIP_MEM: usize, const NUM_GEN_REGS: usize> Cpu<ONCHIP_MEM, NUM_GEN_REGS> {
    /// Creates a new CPU instance.
    ///
    /// # Errors
    ///
    /// Returns an error if the `ONCHIP_MEM` or `NUM_GEN_REGS` constants are too large.
    pub fn new(arch: String, clock: u32) -> Result<Self, &'static str> {
        if ONCHIP_MEM > u16::MAX as usize {
            return Err("MEMORY_SIZE is too large");
        }
        if NUM_GEN_REGS > u8::MAX as usize {
            return Err("NUM_GENERAL_PURPOSE_REGS is too large");
        }
        Ok(Self {
            arch,
            clock,
            onchip_mem: unsafe { std::mem::zeroed::<[u8; ONCHIP_MEM]>() },
            registers: Registers::<NUM_GEN_REGS>::default(),
        })
    }

    pub fn read_mem(&self, addr: u16) -> u8 {
        self.onchip_mem[addr as usize]
    }

    pub fn write_mem(&mut self, addr: u16, value: u8) {
        self.onchip_mem[addr as usize] = value;
    }

    pub fn execute(&mut self, opcode: u8) -> bool {
        // TODO: Implement a parser instead of this garbage
        match opcode {
            // NOP
            0x00 => {
                println!("Executing NOP");
                self.registers.pc += 1;
                return true;
            }
            // ADD
            0x01 => {
                println!("Executing ADD");
                let reg1 = self.registers.get(0);
                let reg2 = self.registers.get(1);
                let result = reg1 + reg2;
                self.registers.set(0, result);
                self.registers.pc += 1;
                return true;
            },
            // SUB
            0x02 => {
                println!("Executing SUB");
                let reg1 = self.registers.get(0);
                let reg2 = self.registers.get(1);
                let result = reg1 - reg2;
                self.registers.set(0, result);
                self.registers.pc += 1;
                return true;
            }
            0xFF => {
                println!("Executing HALT");
                self.registers.pc += 1;
                return false;
            }
            _ => return false,
        };
    }
}

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
        let mut tiny = TinyComputer::<1024, 4096, 2>::new();
        tiny.write_mem(0x0000, 0x00);
        tiny.write_mem(0x0001, 0x01);
        tiny.write_mem(0x0002, 0x02);
        // FIXME: OOB check isn't right, fix memory mapping
        tiny.write_mem(0x0010, 0xFF);
        tiny.run();
    }
}