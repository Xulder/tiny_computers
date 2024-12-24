use crate::core::isa::InstructionError;

/// Defines how instructions are encoded and decoded
///
/// This trait handles:
/// - Decoding binary data into instruction objects
/// - Encoding instructions back to binary
/// - Determining instruction size in bytes
pub trait InstructionCodec {
    type Word: Copy;
    type Error: From<InstructionError>;

    fn decode(bytes: &[Self::Word]) -> Result<Self, Self::Error>
    where
        Self: Sized;
    fn encode(&self) -> Vec<Self::Word>;
    fn size(&self) -> usize;
}
