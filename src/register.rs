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
            pc: PC_START,
            cond: 0,
        }
    }

    pub fn get(&self, register: Register) -> u16 {
        match register {
            Register::R0 => self.r0,
            Register::R1 => self.r1,
            Register::R2 => self.r2,
            Register::R3 => self.r3,
            Register::R4 => self.r4,
            Register::R5 => self.r5,
            Register::R6 => self.r6,
            Register::R7 => self.r7,
            Register::PC => self.pc,
            Register::COND => self.cond,
        }
    }

    pub fn update(&mut self, register: Register, value: u16) {
        match register {
            Register::R0 => self.r0 = value,
            Register::R1 => self.r1 = value,
            Register::R2 => self.r2 = value,
            Register::R3 => self.r3 = value,
            Register::R4 => self.r4 = value,
            Register::R5 => self.r5 = value,
            Register::R6 => self.r6 = value,
            Register::R7 => self.r7 = value,
            Register::PC => self.pc = value,
            Register::COND => self.cond = value,
        }
    }

    pub fn update_cond_register(&mut self, register: Register) {
        if self.get(register) == 0 {
            self.update(Register::COND, ConditionalFlag::ZRO as u16);
        } else if (self.get(register) >> 15) != 0 {
            // NOTE: A 1 in the left-most bit indicates a negative
            self.update(Register::COND, ConditionalFlag::NEG as u16);
        } else {
            self.update(Register::COND, ConditionalFlag::POS as u16);
        }
    }
}
