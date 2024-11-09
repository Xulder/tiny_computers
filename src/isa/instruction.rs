/// The address mode of an instruction.
pub enum AddressingMode {
    //--------------------------------------------------------------------------
    // Immediate Addressing
    //
    // These address modes use the immediate value as the source, whether
    // directly or from a register.
    //--------------------------------------------------------------------------
    /// The opcode followed by an immediate value in the next byte.
    Immediate = 0b0000_0000,

    /// The opcode followed by an immediate value in the next two bytes.
    ImmediateExtended = 0b0000_0001,

    /// The opcode is followed by a register as source.
    /// This does not allow for setting the source and destination as register pairs.
    /// In such a case, you must do two instructions to operate on each half of the register pairs.
    ///
    /// For example,`ld hl, ab` must be split into `ld h, a` and `ld l, b`.
    Register = 0b0000_0010,

    //--------------------------------------------------------------------------
    // Indirect Addressing
    //
    // These address modes use the value at a specific address as the source.
    // You can think of this as a pointer to a value, because that's what it is.
    //--------------------------------------------------------------------------
    /// The opcode is followed by a two byte address where the first byte is the high byte and the second byte is the low byte.
    /// The value at this address is used as the source.
    Indirect = 0b0000_0011,

    /// The opcode is followed by a register pair containing a two byte address.
    /// The value at this address is used as the source.
    RegisterIndirect = 0b0000_0100,

    //--------------------------------------------------------------------------
    // Relative Addressing
    //
    // These address modes use the program counter plus an offset as the source.
    //--------------------------------------------------------------------------
    /// The opcode is followed by an 8 bit displacement value in the next byte.
    /// This value is added to the program counter to determine the address to read or write to.
    /// This is most often seen used for relative jumps.
    Relative = 0b0000_0101,

    /// The opcode is followed by a 16 bit displacement value in the next two bytes.
    /// This value is added to the program counter to determine the address to read or write to.
    /// This is most often seen used for relative jumps.
    RelativeExtended = 0b0000_0110,

    /// The opcode is followed by a register containing an 8 bit displacement value.
    /// This value is added to the program counter to determine the address to read or write to.
    /// This is most often seen used for relative jumps.
    RegisterRelative = 0b0000_0111,

    /// The opcode is followed by a register pair containing a 16 bit displacement value.
    /// This value is added to the program counter to determine the address to read or write to.
    /// This is most often seen used for relative jumps.
    RegisterRelativeExtended = 0b0000_1000,

    //--------------------------------------------------------------------------
    // Indexed Addressing
    //
    // These address modes typically make use of special registers to determine
    // the address to read or write to. The register is followed by an offset
    // value, which is added to the value of the register to determine the
    // address to read or write to.
    //--------------------------------------------------------------------------


    /// The opcode is followed by either a base address or a register containing a base address, as well as an 8 bit displacement value.
    /// The 8 bit displacement value is added to the base address to determine the address to read or write to.
    /// This is most often used for accessing data structures like arrays, strings, tables, etc.
    Indexed = 0b0000_1001,

    /// The opcode is followed by either a base address or a register containing a base address, as well as an 16 bit displacement value.
    /// The 16 bit displacement value is added to the base address to determine the address to read or write to.
    /// This is most often used for accessing data structures like arrays, strings, tables, etc.
    IndexedExtended = 0b0000_1010,

    /// Implied addressing mode opcodes are used for instructions that have no address mode.
    /// They are used for instructions that have no source or destination, such as the `nop` instruction.
    Implied = 0b0000_1011,
}

/// An instruction is a byte encoded with an opcode and an address mode
/// The structure of an instruction is as follows: 
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Instruction(pub u8);