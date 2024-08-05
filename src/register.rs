const PC_START: u16 = 0x3000;

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
    cond: u16,
}

enum ConditionalFlag {
    POS = 1 << 0, // Positive
    ZRO = 1 << 1, // Zero
    NEG = 1 << 2, // Negative
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

    pub fn update(&mut self, index: u16, value: u16) {
        match index {
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
            _ => panic!("Index out of bounds"),
        }
    }

    pub fn get(&self, index: u16) -> u16 {
        match index {
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
            _ => panic!("Index out of bounds"),
        }
    }

    pub fn update_cond_register(&mut self, r: u16) {
        if self.get(r) == 0 {
            self.update(Register::COND as u16, ConditionalFlag::ZRO as u16);
        } else if (self.get(r) >> 15) != 0 {
            // NOTE: A 1 in the left-most bit indicates a negative
            self.update(Register::COND as u16, ConditionalFlag::NEG as u16);
        } else {
            self.update(Register::COND as u16, ConditionalFlag::POS as u16);
        }
    }
}
