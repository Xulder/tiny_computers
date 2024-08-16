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
            // FIXME: These need to check if the carry is required
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
            // ADD R1 addr
            // adds the value at "addr" to R1 and stores it there
            0x11 => {
                println!("Executing ADD R1 addr");
                let addr_hi = self.read_mem(self.registers.pc + 1);
                let addr_lo = self.read_mem(self.registers.pc + 2);
                let addr = u16::from_be_bytes([addr_hi, addr_lo]);
                let value = self.read_mem(addr);
                let reg1 = self.registers.get(0);
                let result = reg1 + value;
                self.registers.set(0, result);
                self.registers.pc += 3;
                return true;
            }
            // ADD R2 addr
            // adds the value at "addr" to R2 and stores it there
            0x21 => {
                println!("Executing ADD addr");
                let addr_hi = self.read_mem(self.registers.pc + 1);
                let addr_lo = self.read_mem(self.registers.pc + 2);
                let addr = u16::from_be_bytes([addr_hi, addr_lo]);
                let value = self.read_mem(addr);
                let reg2 = self.registers.get(1);
                let result = reg2 + value;
                self.registers.set(1, result);
                self.registers.pc += 3;
                return true;
            }
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
            // MUL
            0x03 => {
                println!("Executing MUL");
                let reg1 = self.registers.get(0);
                let reg2 = self.registers.get(1);
                let result = reg1 * reg2;
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