#[derive(Debug)]
#[repr(u8)]
enum OpCode {
    /// Conditional branch
    BR,
    /// Add
    ADD,
    /// Load
    LD,
    /// Store
    ST,
    /// Jump register (jump to subroutine)
    JSR,
    /// Bitwise and
    AND,
    /// Load register (load base + offset)
    LDR,
    /// Store register (store base + offset)
    STR,
    /// Unused
    RTI,
    /// Bitwise not
    NOT,
    /// Load indirect
    LDI,
    /// Store indirect
    STI,
    /// Jump
    JMP,
    /// Reserved (unused)
    RES,
    /// Load effective address
    LEA,
    /// System call
    TRAP,
}

#[derive(Debug)]
pub enum TrapCode {
    /// Get character from keyboard
    GETC = 0x20,
    /// Output a character
    OUT = 0x21,
    /// Output a word string
    PUTS = 0x22,
    /// Input a string
    IN = 0x23,
    /// Output a byte string
    PUTSP = 0x24,
    /// Halt the program
    HALT = 0x25,
}
