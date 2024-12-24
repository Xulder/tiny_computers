use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InstructionCategory {
    Arithmetic,
    Logic,
    DataTransfer,
    Control,
    Stack,
    IO,
    System,
    Misc,
}

impl Display for InstructionCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
