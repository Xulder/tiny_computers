use crate::cpu::error::CPUResult;

/// This trait essentially encompasses an architecture.
pub trait CPU<R: RegisterFile> {
    fn get_register_file(&self) -> &R;
    fn get_register_file_mut(&mut self) -> &mut R;

    fn reset(&mut self);

    fn step(&mut self);

    fn run(&mut self) -> ! {

        self.reset();

        loop {
            self.step();
        }
    }

    fn stop(&mut self);
}

/// Represents a register file of the Tiny Computer.
pub trait RegisterFile {
    fn reset(&mut self);

    fn get_reg(&self, register: u8) -> CPUResult<u8>;

    fn set_reg(&mut self, register: u8, value: u8) -> CPUResult<()>;

    fn get_flags(&self) -> CPUResult<u8>;

    fn set_flags(&mut self, value: u8) -> CPUResult<()>;
}
