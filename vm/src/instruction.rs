use std::str::FromStr;

use crate::{PrivilegeMode, Register, VirtualMachine};

// TODO: Write tests for instructions

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
    /// Return from trap or interrupt
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
        0 => br(vm, instruction),
        1 => add(vm, instruction),
        2 => ld(vm, instruction),
        3 => st(vm, instruction),
        4 => jsr(vm, instruction),
        5 => and(vm, instruction),
        6 => ldr(vm, instruction),
        7 => str(vm, instruction),
        8 => rti(vm, instruction),
        9 => not(vm, instruction),
        10 => ldi(vm, instruction),
        11 => sti(vm, instruction),
        12 => jmp(vm, instruction),
        13 => res(vm, instruction),
        14 => lea(vm, instruction),
        15 => trap(vm, instruction),
        _ => panic!("Unknown opcode {opcode}"),
    }
}

/// Branch
/// The condition codes specified by bits [11:9] are tested. If bit [11] is 1, N is tested;
/// if bit [11] is 0, N is not tested. If bit [10] is 1, Z is tested, etc. If any of the condition
/// codes tested is 1, the program branches to the memory location specified by
/// adding the sign-extended PCoffset9 field to the incremented PC.
///
///  15           12│11 │10 │ 9 │8                                 0
/// ┌───────────────┼───┼───┼───┼───────────────────────────────────┐
/// │      0000     │ N │ Z │ P │             PCOffset9             │
/// └───────────────┴───┴───┴───┴───────────────────────────────────┘
fn br(vm: &mut VirtualMachine, instruction: u16) {
    let flags = (instruction >> 9) & 0x7;

    if flags & vm.registers.get(Register::PSR.into()) != 0 {
        let offset = sign_extend(instruction & 0x1FF, 9);
        let pc = vm.registers.get(Register::PC.into());
        let address = (pc as u32 + offset as u32) as u16;

        vm.registers.set(Register::PC.into(), address);
    }
}

/// Add
/// If bit [5] is 0, the second source operand is obtained from SR2. If bit [5] is 1, the
/// second source operand is obtained by sign-extending the imm5 field to 16 bits.
/// In both cases, the second source operand is added to the contents of SR1 and the
/// result stored in DR. The condition codes are set, based on whether the result is
/// negative, zero, or positive.
///
///  15           12│11        9│8         6│ 5 │4     3│2         0
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
        let result = (vm.registers.get(sr1) as u32 + imm5 as u32) as u16;
        vm.registers.set(dr, result);
    } else {
        let sr2 = instruction & 0x7;
        let result = vm.registers.get(sr1) + vm.registers.get(sr2);
        vm.registers.set(dr, result);
    }

    vm.registers.set_condition_codes(dr);
}

/// Load
/// An address is computed by sign-extending bits [8:0] to 16 bits and adding
/// this value to the incremented PC. If the address is to privileged memory and
/// PSR[15]=1, initiate ACV exception. If not, the contents of memory at this address
/// is loaded into DR. The condition codes are set, based on whether the value loaded
/// is negative, zero, or positive.
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
    if vm.memory.is_privileged(address) && vm.get_mode() == PrivilegeMode::User {
        todo!("Initiate ACV exception");
    } else {
        let value = vm.memory.read(address);
        vm.registers.set(dr, value);
        vm.registers.set_condition_codes(dr);
    }
}

/// Store
/// If the computed address is to privileged memory and PSR[15]=1, initiate ACV
/// exception. If not, the contents of the register specified by SR is stored in the
/// memory location whose address is computed by sign-extending bits [8:0] to 16
/// bits and adding this value to the incremented PC.
///
///  15           12│11        9│8                                 0
/// ┌───────────────┼───────────┼───────────────────────────────────┐
/// │      0011     │     SR    │            PCOffset9              │
/// └───────────────┴───────────┴───────────────────────────────────┘
fn st(vm: &mut VirtualMachine, instruction: u16) {
    let sr = (instruction >> 9) & 0x7;
    let offset = sign_extend(instruction & 0x1FF, 9);
    let pc = vm.registers.get(Register::PC.into());

    let value = vm.registers.get(sr);
    let address = (pc as u32 + offset as u32) as u16;
    if vm.memory.is_privileged(address) && vm.get_mode() == PrivilegeMode::User {
        todo!("Initiate ACV exception");
    } else {
        vm.memory.write(address, value);
    }
}

/// Jump to subroutine
/// First, the incremented PC is saved in a temporary location. Then the PC is loaded
/// with the address of the first instruction of the subroutine, which will cause an
/// unconditional jump to that address after the current instruction completes execution.
/// The address of the subroutine is obtained from the base register (if bit [11]
/// is 0), or the address is computed by sign-extending bits [10:0] and adding this
/// value to the incremented PC (if bit [11] is 1). Finally, R7 is loaded with the value
/// stored in the temporary location. This is the linkage back to the calling routine.
///
///  15           12│11 │10                                        0
/// ┌───────────────┼───┼───────────────────────────────────────────┐
/// │      0100     │ 1 │                PCOffset11                 │
/// └───────────────┴───┴───────────────────────────────────────────┘
///
///  15           12│11 │10    9│8     6│5                         0
/// ┌───────────────┼───┼───────┼───────┼───────────────────────────┐
/// │      0100     │ 0 │   00  │ BaseR │           00000           │
/// └───────────────┴───┴───────┴───────┴───────────────────────────┘
fn jsr(vm: &mut VirtualMachine, instruction: u16) {
    let flag = (instruction >> 11) & 0x1;
    let pc = vm.registers.get(Register::PC.into());
    vm.registers.set(Register::R7.into(), pc);

    if flag == 1 {
        let offset = sign_extend(instruction & 0x7FF, 11);
        let address = (pc as u32 + offset as u32) as u16;
        vm.registers.set(Register::PC.into(), address);
    } else {
        let reg = (instruction >> 6) & 0x7;
        let address = vm.registers.get(reg);
        vm.registers.set(Register::PC.into(), address);
    }
}

/// Bit-wise logical AND
/// If bit [5] is 0, the second source operand is obtained from SR2. If bit [5] is 1,
/// the second source operand is obtained by sign-extending the imm5 field to 16
/// bits. In either case, the second source operand and the contents of SR1 are bitwise
/// ANDed and the result stored in DR. The condition codes are set, based on
/// whether the binary value produced, taken as a 2’s complement integer, is negative,
/// zero, or positive.
///
///  15           12│11        9│8         6│ 5 │4     3│2         0
/// ┌───────────────┼───────────┼───────────┼───┼───────┼───────────┐
/// │      0101     │     DR    │  SR1      │ 0 │  00   │    SR2    │
/// └───────────────┴───────────┴───────────┴───┴───────┴───────────┘
///
///  15           12│11        9│8         6│ 5 │4                 0
/// ┌───────────────┼───────────┼───────────┼───┼───────────────────┐
/// │      0101     │     DR    │  SR1      │ 1 │       IMM5        │
/// └───────────────┴───────────┴───────────┴───┴───────────────────┘
fn and(vm: &mut VirtualMachine, instruction: u16) {
    let dr = (instruction >> 9) & 0x7;
    let sr1 = (instruction >> 6) & 0x7;
    let imm_flag = (instruction >> 5) & 0x1;

    if imm_flag == 1 {
        let imm5 = sign_extend(instruction & 0x1F, 5);
        let result = vm.registers.get(sr1) & imm5;
        vm.registers.set(dr, result);
    } else {
        let sr2 = instruction & 0x7;
        let result = vm.registers.get(sr1) & vm.registers.get(sr2);
        vm.registers.set(dr, result);
    }

    vm.registers.set_condition_codes(dr);
}

/// Load base+offset
/// An address is computed by sign-extending bits [5:0] to 16 bits and adding this
/// value to the contents of the register specified by bits [8:6]. If the computed address
/// is to privileged memory and PSR[15]=1, initiate ACV exception. If not, the contents
/// of memory at this address is loaded into DR. The condition codes are set,
/// based on whether the value loaded is negative, zero, or positive.
///
///  15           12│11        9│8             6│5                 0
/// ┌───────────────┼───────────┼───────────────┼───────────────────┐
/// │      1010     │     DR    │     BaseR     │     PCOffset6     │
/// └───────────────┴───────────┴───────────────┴───────────────────┘
fn ldr(vm: &mut VirtualMachine, instruction: u16) {
    let dr = (instruction >> 9) & 0x7;
    let reg = (instruction >> 6) & 0x7;
    let offset = sign_extend(instruction & 0x3F, 6);

    let address = (vm.registers.get(reg) as u32 + offset as u32) as u16;
    if vm.memory.is_privileged(address) && vm.get_mode() == PrivilegeMode::User {
        todo!("Initiate ACV exception");
    } else {
        let value = vm.memory.read(address);
        vm.registers.set(dr, value);
        vm.registers.set_condition_codes(dr);
    }
}

/// Store base+offset
/// If the computed address is to privileged memory and PSR[15]=1, initiate ACV
/// exception. If not, the contents of the register specified by SR is stored in the
/// memory location whose address is computed by sign-extending bits [5:0] to 16
/// bits and adding this value to the contents of the register specified by bits [8:6].
///
///  15           12│11        9│8         6│                      0
/// ┌───────────────┼───────────┼───────────┼───────────────────────┐
/// │      0111     │     SR    │   BaseR   │        PCOffset6      │
/// └───────────────┴───────────┴───────────┴───────────────────────┘
fn str(vm: &mut VirtualMachine, instruction: u16) {
    let sr = (instruction >> 9) & 0x7;
    let reg = (instruction >> 6) & 0x7;
    let offset = sign_extend(instruction & 0x3F, 6);

    let address = (vm.registers.get(reg) as u32 + offset as u32) as u16;
    if vm.memory.is_privileged(address) && vm.get_mode() == PrivilegeMode::User {
        todo!("Initiate ACV exception");
    } else {
        let value = vm.registers.get(sr);
        vm.memory.write(address, value)
    }
}

/// Return from trap or interrupt
/// If the processor is running in User mode, a privilege mode exception occurs. If
/// in Supervisor mode, the top two elements on the system stack are popped and
/// loaded into PC, PSR. After PSR is restored, if the processor is running in User
/// mode, the SSP is saved in Saved SSP, and R6 is loaded with Saved USP.
///
///  15           12│11                                            0
/// ┌───────────────┼───────────────────────────────────────────────┐
/// │      1000     │                  000000000000                 │
/// └───────────────┴───────────────────────────────────────────────┘
fn rti(vm: &mut VirtualMachine, _instruction: u16) {
    if vm.get_mode() == PrivilegeMode::User {
        todo!("Initiate privilege mode exception");
    } else {
        todo!("Figure out how to implement this instruction");
    }
}

// Bit-wise complement
/// The bit-wise complement of the contents of SR is stored in DR. The condition
/// codes are set, based on whether the binary value produced, taken as a 2’s
/// complement integer, is negative, zero, or positive.
///
///  15          12 │11        9│8         6│ 5 │4                 0
/// ┌───────────────┼───────────┼───────────┼───┼───────────────────┐
/// │      1001     │     DR    │     SR    │ 1 │       1111        │
/// └───────────────┴───────────┴───────────┴───┴───────────────────┘
fn not(vm: &mut VirtualMachine, instruction: u16) {
    let dr = (instruction >> 9) & 0x7;
    let sr = (instruction >> 6) & 0x7;

    let value = vm.registers.get(sr);
    vm.registers.set(dr, !value);
    vm.registers.set_condition_codes(dr);
}

/// Load indirect
/// An address is computed by sign-extending bits [8:0] to 16 bits and adding this
/// value to the incremented PC. What is stored in memory at this address is the
/// address of the data to be loaded into DR. If either address is to privileged memory
/// and PSR[15]=1, initiate ACV exception. If not, the data is loaded and the
/// condition codes are set, based on whether the value loaded is negative, zero, or
/// positive.
///
///  15           12│11        9│8                                 0
/// ┌───────────────┼───────────┼───────────────────────────────────┐
/// │      0010     │     DR    │            PCOffset9              │
/// └───────────────┴───────────┴───────────────────────────────────┘
fn ldi(vm: &mut VirtualMachine, instruction: u16) {
    let dr = (instruction >> 9) & 0x7;
    let offset = sign_extend(instruction & 0x1FF, 9);
    let pc = vm.registers.get(Register::PC.into());

    let indirect_address = (pc as u32 + offset as u32) as u16;
    let address = vm.memory.read(indirect_address);
    if (vm.memory.is_privileged(indirect_address) | vm.memory.is_privileged(address))
        && vm.get_mode() == PrivilegeMode::User
    {
        todo!("Initiate ACV exception");
    } else {
        vm.registers.set(dr, address);
        vm.registers.set_condition_codes(dr);
    }
}

/// Store indirect
/// If either computed address is to privileged memory and PSR[15]=1, initiate
/// ACV exception. If not, the contents of the register specified by SR is stored
/// in the memory location whose address is obtained as follows: Bits [8:0] are signextended
/// to 16 bits and added to the incremented PC. What is in memory at this
/// address is the address of the location to which the data in SR is stored.
///
///  15           12│11        9│8                                 0
/// ┌───────────────┼───────────┼───────────────────────────────────┐
/// │      1011     │     SR    │            PCOffset9              │
/// └───────────────┴───────────┴───────────────────────────────────┘
fn sti(vm: &mut VirtualMachine, instruction: u16) {
    let sr = (instruction >> 9) & 0x7;
    let offset = sign_extend(instruction & 0x1FF, 9);
    let pc = vm.registers.get(Register::PC.into());

    let value = vm.registers.get(sr);

    let indirect_address = (pc as u32 + offset as u32) as u16;
    let address = vm.memory.read(indirect_address);
    if (vm.memory.is_privileged(indirect_address) | vm.memory.is_privileged(address))
        && vm.get_mode() == PrivilegeMode::User
    {
        todo!("Initiate ACV exception");
    } else {
        vm.memory.write(address, value);
    }
}

/// Jump
/// The program unconditionally jumps to the location specified by the contents of
/// the base register. Bits [8:6] identify the base register.
///
/// The RET instruction is a special case of the JMP instruction, normally used in the
/// return from a subroutine. The PC is loaded with the contents of R7, which contains
/// the linkage back to the instruction following the subroutine call instruction.
///
///  15           12│11        9│8         6│5                     0
/// ┌───────────────┼───────────┼───────────┼───────────────────────┐
/// │      1100     │    000    │   BaseR   │       00000           │
/// └───────────────┴───────────┴───────────┴───────────────────────┘
///
///  15           12│11        9│8         6│5                     0
/// ┌───────────────┼───────────┼───────────┼───────────────────────┐
/// │      1100     │    000    │    111    │       00000           │
/// └───────────────┴───────────┴───────────┴───────────────────────┘
fn jmp(vm: &mut VirtualMachine, instruction: u16) {
    let reg = (instruction >> 6) & 0x7;
    let address = vm.registers.get(reg);
    vm.registers.set(Register::PC.into(), address);
}

/// Reserved (unused)
///
///  15           12│11                                            0
/// ┌───────────────┼───────────────────────────────────────────────┐
/// │      1101     │                                               │
/// └───────────────┴───────────────────────────────────────────────┘
fn res(_vm: &mut VirtualMachine, _instruction: u16) {
    todo!("Initiate an illegal opcode exception");
}

/// Load effective address
/// An address is computed by sign-extending bits [8:0] to 16 bits and adding this
/// value to the incremented PC. This address is loaded into DR.
///
///  15           12│11        9│8                                 0
/// ┌───────────────┼───────────┼───────────────────────────────────┐
/// │      1110     │     DR    │            PCOffset9              │
/// └───────────────┴───────────┴───────────────────────────────────┘
fn lea(vm: &mut VirtualMachine, instruction: u16) {
    let dr = (instruction >> 9) & 0x7;
    let offset = sign_extend(instruction & 0x1FF, 9);
    let pc = vm.registers.get(Register::PC.into());

    let address = (pc as u32 + offset as u32) as u16;
    vm.registers.set(dr, address);
}

/// System call
/// If the the program is executing in User mode, the User Stack Pointer must be
/// saved and the System Stack Pointer loaded. Then the PSR and PC are pushed
/// on the system stack. (This enables a return to the instruction physically following
/// the TRAP instruction in the original program after the last instruction in the
/// service routine (RTI) has completed execution.) Then the PC is loaded with the
/// starting address of the system call specified by trapvector8. The starting address
/// is contained in the memory location whose address is obtained by zero-extending
/// trapvector8 to 16 bits.
///
///  15           12│11        8│7                                 0
/// ┌───────────────┼───────────┼───────────────────────────────────┐
/// │      1111     │    0000   │            trapvect8              │
/// └───────────────┴───────────┴───────────────────────────────────┘
fn trap(vm: &mut VirtualMachine, _instruction: u16) {
    if vm.get_mode() == PrivilegeMode::User {
        todo!("Save stack pointers");
    }
    todo!("Implement based on textbook explanation");
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
