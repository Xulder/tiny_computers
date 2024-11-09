use crate::cpu::error::CPUResult;
use crate::isa::traits::ISA;

/// This trait essentially encompasses an architecture.
pub trait CPU<R: RegisterFile, I: ISA> {

    fn new(isa: I) -> Self;

    /// ----------------------------------------------------------------
    /// Register File
    /// ----------------------------------------------------------------

    /// Get a reference to the register file.
    fn get_register_file(&self) -> &R;

    /// Get a mutable reference to the register file.
    fn get_register_file_mut(&mut self) -> &mut R;

    /// ----------------------------------------------------------------
    /// Program Counter & Stack Pointer
    /// ----------------------------------------------------------------

    /// Get the current program counter.
    fn get_pc(&self) -> u16;

    /// Set the current program counter.
    fn set_pc(&mut self, pc: u16);

    /// Increment the current program counter. This is a no-op in most architectures.
    ///
    /// **Warning:** This will not wrap around on overflow.
    ///
    /// How the memory bus and/or mapping is implemented will determine if this is a problem.
    fn inc_pc(&mut self) {
        self.set_pc(self.get_pc() + 1);
    }

    /// Decrement the current program counter.
    ///
    /// **Warning:** This will not around on underflow.
    ///
    /// How the memory bus and/or mapping is implemented will determine if this is a problem.
    fn dec_pc(&mut self) {
        self.set_pc(self.get_pc() - 1);
    }

    /// Get the current stack pointer.
    fn get_sp(&self) -> u16;

    /// Set the current stack pointer.
    fn set_sp(&mut self, sp: u16);

    /// Increment the current stack pointer.
    ///
    /// **Warning:** This will not wrap around on overflow.
    /// The memory bus and/or mapping is responsible for this.
    /// Ideally this should result in an error like "Stack pointer OOB".
    fn inc_sp(&mut self) {
        self.set_sp(self.get_sp() + 1);
    }

    /// Decrement the current stack pointer.
    ///
    /// **Warning:** This will not wrap around on underflow.
    /// The memory bus and/or mapping is responsible for this.
    /// Ideally this should result in an error like "Stack pointer OOB".
    fn dec_sp(&mut self) {
        self.set_sp(self.get_sp() - 1);
    }

    // ----------------------------------------------------------------
    // CPU Operations
    // ----------------------------------------------------------------

    /// Reset all registers to zero, clear flags, set program counter and stack pointer to zero.
    fn reset(&mut self);

    /// Stop the CPU.
    fn stop(&mut self) {
        self.set_pc(0);
        todo!("CPU halt");
    }

    /// Execute one instruction. This should *always* move the program counter.
    fn step(&mut self);

    /// Run the CPU until it halts.
    ///
    /// # Errors
    ///
    /// Returns an error if the program counter overflows and wraps around.
    /// This is a fatal error and should never happen unless the HLT instruction is used.
    fn run(&mut self) -> CPUResult<()> {
        self.reset();

        loop {
            self.step();

            // If the program counter is zero, we've reached the end of the program.
            // HLT is the only instruction that sets the program counter to zero.
            // 0x0000 should never be reached, written to, or read from unless HLT is used.
            // This applies to all architectures in Tiny Computers.
            if self.get_pc() == 0 {
                break;
            }
        }
        Ok(())
    }
}

/// Represents a register file of the Tiny Computer.
pub trait RegisterFile {
    /// Reset all registers to zero.
    fn reset(&mut self);

    /// Get the value of a register.
    ///
    /// # Errors
    ///
    /// Returns an error if the register is invalid.
    fn get_reg(&self, register: u8) -> CPUResult<u8>;

    /// Set the value of a register.
    ///
    /// # Errors
    ///
    /// Returns an error if the register is invalid.
    fn set_reg(&mut self, register: u8, value: u8) -> CPUResult<()>;

    /// Get the current flags.
    ///
    /// # Errors
    ///
    /// Returns an error if the flags are invalid (which should never happen and be impossible).
    fn get_flags(&self) -> CPUResult<u8>;

    /// Set the flags.
    ///
    /// # Errors
    ///
    /// Returns an error if the flags are invalid (such as an unknown flag or an invalid combination).
    fn set_flags(&mut self, value: u8) -> CPUResult<()>;
}
