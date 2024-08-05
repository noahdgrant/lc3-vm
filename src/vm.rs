use crate::register::{Register, Registers};

const MEMORY_SIZE: usize = u16::MAX as usize;

pub struct VirtualMachine {
    registers: Registers,
    memory: [u16; MEMORY_SIZE],
}

impl VirtualMachine {
    pub fn new() -> Self {
        Self {
            registers: Registers::new(),
            memory: [0; MEMORY_SIZE],
        }
    }

    pub fn read_memory(&self, address: u16) -> u16 {
        self.memory[address as usize]
    }

    // TODO: Could return error if user tries to write to memory they are
    // not allowed to
    pub fn write_memory(&mut self, address: usize, value: u16) {
        self.memory[address] = value;
    }
}
