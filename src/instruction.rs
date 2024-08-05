use crate::VirtualMachine;

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

pub fn execute(vm: &mut VirtualMachine, instruction: u16) {
    let op_code = get_op_code(&instruction);

    match op_code {
        OpCode::BR => br(vm, instruction),
        OpCode::ADD => add(vm, instruction),
        OpCode::LD => ld(vm, instruction),
        OpCode::ST => st(vm, instruction),
        OpCode::JSR => jsr(vm, instruction),
        OpCode::AND => and(vm, instruction),
        OpCode::LDR => ldr(vm, instruction),
        OpCode::STR => str(vm, instruction),
        OpCode::RTI => rti(vm, instruction),
        OpCode::NOT => not(vm, instruction),
        OpCode::LDI => ldi(vm, instruction),
        OpCode::STI => sti(vm, instruction),
        OpCode::JMP => jmp(vm, instruction),
        OpCode::RES => res(vm, instruction),
        OpCode::LEA => lea(vm, instruction),
        OpCode::TRAP => trap(vm, instruction),
    }
}

fn get_op_code(instruction: &u16) -> OpCode {
    let op_code = instruction >> 12;

    match op_code {
        0 => OpCode::BR,
        1 => OpCode::ADD,
        2 => OpCode::LD,
        3 => OpCode::ST,
        4 => OpCode::JSR,
        5 => OpCode::AND,
        6 => OpCode::LDR,
        7 => OpCode::STR,
        8 => OpCode::RTI,
        9 => OpCode::NOT,
        10 => OpCode::LDI,
        11 => OpCode::STI,
        12 => OpCode::JMP,
        13 => OpCode::RES,
        14 => OpCode::LEA,
        15 => OpCode::TRAP,
        _ => panic!("Unknown opcode {}", op_code),
    }
}

fn br(vm: &mut VirtualMachine, instruction: u16) {
    todo!()
}

fn add(vm: &mut VirtualMachine, instruction: u16) {
    todo!()
}

fn ld(vm: &mut VirtualMachine, instruction: u16) {
    todo!()
}

fn st(vm: &mut VirtualMachine, instruction: u16) {
    todo!()
}

fn jsr(vm: &mut VirtualMachine, instruction: u16) {
    todo!()
}

fn and(vm: &mut VirtualMachine, instruction: u16) {
    todo!()
}

fn ldr(vm: &mut VirtualMachine, instruction: u16) {
    todo!()
}

fn str(vm: &mut VirtualMachine, instruction: u16) {
    todo!()
}

fn rti(vm: &mut VirtualMachine, instruction: u16) {
    todo!()
}

fn not(vm: &mut VirtualMachine, instruction: u16) {
    todo!()
}

fn ldi(vm: &mut VirtualMachine, instruction: u16) {
    todo!()
}

fn sti(vm: &mut VirtualMachine, instruction: u16) {
    todo!()
}

fn jmp(vm: &mut VirtualMachine, instruction: u16) {
    todo!()
}

fn res(vm: &mut VirtualMachine, instruction: u16) {
    todo!()
}

fn lea(vm: &mut VirtualMachine, instruction: u16) {
    todo!()
}

fn trap(vm: &mut VirtualMachine, instruction: u16) {
    todo!()
}
