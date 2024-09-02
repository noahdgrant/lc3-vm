use std::str::FromStr;

const PC_START: u16 = 0x3000;

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
    COND,
}

#[derive(Debug, PartialEq, Eq)]
pub struct RegisterError;

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
            "COND" => Ok(Register::COND),
            _ => Err(RegisterError),
        }
    }
}

#[derive(Clone, Copy, Debug)]
#[repr(u16)]
enum ConditionalFlag {
    POS = 1 << 0, // Positive
    ZRO = 1 << 1, // Zero
    NEG = 1 << 2, // Negative
}

pub struct Registers {
    pub r0: u16,
    pub r1: u16,
    pub r2: u16,
    pub r3: u16,
    pub r4: u16,
    pub r5: u16,
    pub r6: u16,
    pub r7: u16,
    pub pc: u16,
    pub cond: u16,
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
            pc: PC_START, // TODO: change with to 0 and read from .asm file
            cond: 0,
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
            9 => self.cond,
            _ => panic!("Unknown register"),
        }
    }

    pub fn update(&mut self, register: u16, value: u16) {
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
            9 => self.cond = value,
            _ => panic!("Unknown register"),
        }
    }

    pub fn update_cond_register(&mut self, register: u16) {
        if self.get(register) == 0 {
            self.update(Register::COND as u16, ConditionalFlag::ZRO as u16);
        } else if (self.get(register) >> 15) != 0 {
            // NOTE: A 1 in the left-most bit indicates a negative
            self.update(Register::COND as u16, ConditionalFlag::NEG as u16);
        } else {
            self.update(Register::COND as u16, ConditionalFlag::POS as u16);
        }
    }

    pub fn increment_pc_register(&mut self) {
        self.pc += 1;
    }
}
