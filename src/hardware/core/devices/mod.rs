pub mod memory;

pub trait Device {
    fn reset(&mut self);
}