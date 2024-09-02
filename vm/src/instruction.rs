// Comments before instructions taken from:
// https://github.com/digorithm/LC-3-Rust/blob/main/src/hardware/instruction/mod.rs

use std::str::FromStr;

use crate::{Register, VirtualMachine, MEMORY_SIZE};

// TODO: Implement Display trait
#[derive(Debug, Copy, Clone)]
#[repr(u8)]
pub enum OpCode {
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

#[derive(Debug, PartialEq, Eq)]
pub struct OpCodeError;

impl FromStr for OpCode {
    type Err = OpCodeError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "BR" => Ok(OpCode::BR),
            "ADD" => Ok(OpCode::ADD),
            "LD" => Ok(OpCode::LD),
            "ST" => Ok(OpCode::ST),
            "JSR" => Ok(OpCode::JSR),
            "AND" => Ok(OpCode::AND),
            "LDR" => Ok(OpCode::LDR),
            "STR" => Ok(OpCode::STR),
            "RTI" => Ok(OpCode::RTI),
            "NOT" => Ok(OpCode::NOT),
            "LDI" => Ok(OpCode::LDI),
            "STI" => Ok(OpCode::STI),
            "JMP" => Ok(OpCode::JMP),
            "RES" => Ok(OpCode::RES),
            "LEA" => Ok(OpCode::LEA),
            "HALT" => Ok(OpCode::TRAP),
            _ => Err(OpCodeError),
        }
    }
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

/// ADD takes two values and stores them in a register.
/// In register mode, the second value to add is found in a register.
/// In immediate mode, the second value is embedded in the right-most 5 bits of the instruction.
/// Values which are shorter than 16 bits need to be sign extended.
/// Any time an instruction modifies a register, the condition flags need to be updated
/// If bit [5] is 0, the second source operand is obtained from SR2.
/// If bit [5] is 1, the second source operand is obtained by sign-extending the imm5 field to 16 bits.
/// In both cases, the second source operand is added to the contents of SR1 and the result stored in DR.
/// The condition codes are set, based on whether the result is negative, zero, or positive.
/// Encoding:
///
/// 15           12 │11        9│8         6│ 5 │4     3│2         0
/// ┌───────────────┼───────────┼───────────┼───┼───────┼───────────┐
/// │      0001     │     DR    │  SR1      │ 0 │  00   │    SR2    │
/// └───────────────┴───────────┴───────────┴───┴───────┴───────────┘
///
///  15           12│11        9│8         6│ 5 │4                 0
/// ┌───────────────┼───────────┼───────────┼───┼───────────────────┐
/// │      0001     │     DR    │  SR1      │ 1 │       IMM5        │
/// └───────────────┴───────────┴───────────┴───┴───────────────────┘
fn add(vm: &mut VirtualMachine, instruction: u16) {
    let dr = (instruction >> 9) & 0x7;
    let sr1 = (instruction >> 6) & 0x7;
    let imm_flag = (instruction >> 5) & 0x1;

    if imm_flag == 1 {
        let imm5 = sign_extend(instruction & 0x1F, 5);
        let result = vm.registers.get(sr1) + imm5;
        vm.registers.set(dr, result);
    } else {
        let sr2 = instruction & 0x7;
        let result = vm.registers.get(sr1) + vm.registers.get(sr2);
        vm.registers.set(dr, result);
    }

    vm.registers.update_cond_register(dr);
}

/// An address is computed by sign-extending bits [8:0] to 16 bits and
/// adding this value to the incremented PC.
/// The contents of memory at this address are loaded into DR.
/// The condition codes are set, based on whether the value loaded is negative, zero, or positive.
///
///  15           12│11        9│8                                 0
/// ┌───────────────┼───────────┼───────────────────────────────────┐
/// │      0010     │     DR    │            PCOffset9              │
/// └───────────────┴───────────┴───────────────────────────────────┘
fn ld(vm: &mut VirtualMachine, instruction: u16) {
    let dr = (instruction >> 9) & 0x7;
    let offset = sign_extend(instruction & 0x1FF, 9);
    let pc = vm.registers.get(Register::PC as u16);

    let address: u32 = pc as u32 + offset as u32;
    if address > MEMORY_SIZE as u32 {
        panic!("Tried to access invalid memory address 0x{:X}", address);
    }

    let value = vm.read_memory(address as u16);
    vm.registers.set(dr, value);
    vm.registers.update_cond_register(dr);
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

fn sign_extend(mut x: u16, bit_count: u8) -> u16 {
    // bit_count is the original number of bits
    // that this binary value has. We want to take that
    // and transform it into a 16 bits value.

    // Then check if it's different than zero,
    // if it is, it's signed as 1 (negative)
    // Meaning we have to pad with ones instead of zeroes
    if (x >> (bit_count - 1)) & 0x1 != 0 {
        x |= 0xFFFF << bit_count;
    }

    // If it's positive, return as is, it will be padded
    // with zeroes.
    x
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sign_extend_positive() {
        let x: u16 = 3;
        let value = sign_extend(x, 5);
        assert_eq!(3, value);
    }

    #[test]
    fn sign_extend_negative() {
        let x: i16 = -5;
        let value = sign_extend(x as u16, 5);
        assert_eq!(0xFFFB, value);
    }
}
