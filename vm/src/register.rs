use std::fmt;
use std::str::FromStr;

#[derive(Clone, Copy, Debug)]
#[repr(u16)]
pub enum Register {
    R0,
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
    PC,
    IR,
    PSR,
}

#[derive(Debug, PartialEq, Eq)]
pub enum RegisterError {
    UnknownRegister(String),
}

impl fmt::Display for RegisterError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RegisterError::UnknownRegister(s) => write!(f, "Unknown register {}", s),
        }
    }
}

impl FromStr for Register {
    type Err = RegisterError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "R0" => Ok(Register::R0),
            "R1" => Ok(Register::R1),
            "R2" => Ok(Register::R2),
            "R3" => Ok(Register::R3),
            "R4" => Ok(Register::R4),
            "R5" => Ok(Register::R5),
            "R6" => Ok(Register::R6),
            "R7" => Ok(Register::R7),
            "PC" => Ok(Register::PC),
            "IR" => Ok(Register::IR),
            "PSR" => Ok(Register::PSR),
            _ => Err(RegisterError::UnknownRegister(s.into())),
        }
    }
}

impl From<Register> for u16 {
    fn from(register: Register) -> Self {
        register as u16
    }
}

/// Bits \[2:0\] of the PSR register
/// Bit 2: Negative
/// Bit 1: Zero
/// Bit 0: Positive
#[derive(Clone, Copy, Debug)]
#[repr(u16)]
pub enum ConditionalFlag {
    Positive = 1 << 0,
    Zero = 1 << 1,
    Negative = 1 << 2,
}

impl From<ConditionalFlag> for u16 {
    fn from(flag: ConditionalFlag) -> Self {
        flag as u16
    }
}

#[derive(Debug, Default)]
pub struct Registers {
    r0: u16,
    r1: u16,
    r2: u16,
    r3: u16,
    r4: u16,
    r5: u16,
    r6: u16,
    r7: u16,
    pc: u16,
    ir: u16,
    psr: u16,
}

impl Registers {
    pub fn new() -> Self {
        Self {
            r0: 0,
            r1: 0,
            r2: 0,
            r3: 0,
            r4: 0,
            r5: 0,
            r6: 0,
            r7: 0,
            pc: 0x3000,
            ir: 0,
            psr: 0x8002,
        }
    }

    pub fn get(&self, register: u16) -> u16 {
        match register {
            0 => self.r0,
            1 => self.r1,
            2 => self.r2,
            3 => self.r3,
            4 => self.r4,
            5 => self.r5,
            6 => self.r6,
            7 => self.r7,
            8 => self.pc,
            9 => self.ir,
            10 => self.psr,
            _ => panic!("Can't get unknown register {register}"),
        }
    }

    pub fn set(&mut self, register: u16, value: u16) {
        match register {
            0 => self.r0 = value,
            1 => self.r1 = value,
            2 => self.r2 = value,
            3 => self.r3 = value,
            4 => self.r4 = value,
            5 => self.r5 = value,
            6 => self.r6 = value,
            7 => self.r7 = value,
            8 => self.pc = value,
            9 => self.ir = value,
            10 => self.psr = value,
            _ => panic!("Can't set unknown register {register}"),
        }
    }

    pub fn dump(&self) {
        println!(
            "R0: 0x{:04X} | R1: 0x{:04X} | R2: 0x{:04X} | R3: 0x{:04X} | R4: 0x{:04X} | R5: 0x{:04X}",
            self.get(Register::R0.into()),
            self.get(Register::R1.into()),
            self.get(Register::R2.into()),
            self.get(Register::R3.into()),
            self.get(Register::R4.into()),
            self.get(Register::R5.into()),
        );
        println!(
            "R6: 0x{:04X} | R7: 0x{:04X} | PC: 0x{:04X} | IR: 0x{:04X} | PSR: 0x{:04X}",
            self.get(Register::R6.into()),
            self.get(Register::R7.into()),
            self.get(Register::PC.into()),
            self.get(Register::IR.into()),
            self.get(Register::PSR.into())
        );
    }

    pub fn set_condition_codes(&mut self, register: u16) {
        let psr = self.get(Register::PSR.into());
        let mask = 0xFFF8;
        if self.get(register) == 0 {
            self.set(
                Register::PSR.into(),
                (psr & mask) | ConditionalFlag::Zero as u16,
            );
        } else if (self.get(register) >> 15) != 0 {
            // NOTE: A 1 in the left-most bit indicates a negative
            self.set(
                Register::PSR.into(),
                (psr & mask) | ConditionalFlag::Negative as u16,
            );
        } else {
            self.set(
                Register::PSR.into(),
                (psr & mask) | ConditionalFlag::Positive as u16,
            );
        }
    }

    pub fn increment_pc_register(&mut self) {
        self.pc += 1;
    }
}
