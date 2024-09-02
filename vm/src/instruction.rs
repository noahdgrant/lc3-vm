// Comments before instructions taken from:
// https://github.com/digorithm/LC-3-Rust/blob/main/src/hardware/instruction/mod.rs

use std::str::FromStr;

use crate::{Register, VirtualMachine};

#[derive(Debug, Copy, Clone)]
#[repr(u8)]
pub enum Opcode {
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

impl From<Opcode> for u8 {
    fn from(opcode: Opcode) -> Self {
        opcode as u8
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum OpcodeError {
    UnknownOpcode(String),
}

impl FromStr for Opcode {
    type Err = OpcodeError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "BR" => Ok(Opcode::BR),
            "ADD" => Ok(Opcode::ADD),
            "LD" => Ok(Opcode::LD),
            "ST" => Ok(Opcode::ST),
            "JSR" => Ok(Opcode::JSR),
            "AND" => Ok(Opcode::AND),
            "LDR" => Ok(Opcode::LDR),
            "STR" => Ok(Opcode::STR),
            "RTI" => Ok(Opcode::RTI),
            "NOT" => Ok(Opcode::NOT),
            "LDI" => Ok(Opcode::LDI),
            "STI" => Ok(Opcode::STI),
            "JMP" => Ok(Opcode::JMP),
            "RES" => Ok(Opcode::RES),
            "LEA" => Ok(Opcode::LEA),
            "HALT" => Ok(Opcode::TRAP),
            _ => Err(OpcodeError::UnknownOpcode(s.to_string())),
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
    let opcode = instruction >> 12;

    match opcode {
        //0 => br(vm, instruction),
        1 => add(vm, instruction),
        2 => ld(vm, instruction),
        //3 => st(vm, instruction),
        //4 => jsr(vm, instruction),
        //5 => and(vm, instruction),
        //6 => ldr(vm, instruction),
        //7 => str(vm, instruction),
        //8 => rti(vm, instruction),
        //9 => not(vm, instruction),
        //10 => ldi(vm, instruction),
        //11 => sti(vm, instruction),
        //12 => jmp(vm, instruction),
        //13 => res(vm, instruction),
        //14 => lea(vm, instruction),
        //15 => trap(vm, instruction),
        _ => panic!("Unknown opcode {opcode}"),
    }
}

//fn br(vm: &mut VirtualMachine, instruction: u16) {
//    todo!()
//}

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
    let pc = vm.registers.get(Register::PC.into());

    let address = (pc as u32 + offset as u32) as u16;

    let value = vm.memory.read(address);
    vm.registers.set(dr, value);
    vm.registers.update_cond_register(dr);
}

//fn st(vm: &mut VirtualMachine, instruction: u16) {
//    todo!()
//}
//
//fn jsr(vm: &mut VirtualMachine, instruction: u16) {
//    todo!()
//}
//
//fn and(vm: &mut VirtualMachine, instruction: u16) {
//    todo!()
//}
//
//fn ldr(vm: &mut VirtualMachine, instruction: u16) {
//    todo!()
//}
//
//fn str(vm: &mut VirtualMachine, instruction: u16) {
//    todo!()
//}
//
//fn rti(vm: &mut VirtualMachine, instruction: u16) {
//    todo!()
//}
//
//fn not(vm: &mut VirtualMachine, instruction: u16) {
//    todo!()
//}
//
//fn ldi(vm: &mut VirtualMachine, instruction: u16) {
//    todo!()
//}
//
//fn sti(vm: &mut VirtualMachine, instruction: u16) {
//    todo!()
//}
//
//fn jmp(vm: &mut VirtualMachine, instruction: u16) {
//    todo!()
//}
//
//fn res(vm: &mut VirtualMachine, instruction: u16) {
//    todo!()
//}
//
//fn lea(vm: &mut VirtualMachine, instruction: u16) {
//    todo!()
//}
//
//fn trap(vm: &mut VirtualMachine, instruction: u16) {
//    todo!()
//}

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
